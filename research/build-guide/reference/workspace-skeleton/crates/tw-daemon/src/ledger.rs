//! GIVEN core (M4) + exercises (`all`, `active`). Folded in from the former `tw-store` crate.
use std::path::Path;

use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};
use tw_core::ChannelRecord;

const CHANNELS: TableDefinition<&[u8; 32], &[u8]> = TableDefinition::new("channels");

#[derive(Debug, thiserror::Error)]
pub enum LedgerError {
    #[error("database: {0}")]
    Db(#[from] redb::DatabaseError),
    #[error("transaction: {0}")]
    Txn(#[from] redb::TransactionError),
    #[error("table: {0}")]
    Table(#[from] redb::TableError),
    #[error("storage: {0}")]
    Storage(#[from] redb::StorageError),
    #[error("commit: {0}")]
    Commit(#[from] redb::CommitError),
    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),
}

pub struct Ledger {
    db: Database,
}

impl Ledger {
    pub fn open(path: &Path) -> Result<Self, LedgerError> {
        let db = Database::create(path)?;
        let w = db.begin_write()?;
        {
            w.open_table(CHANNELS)?; // create the table; the guard drops before commit
        }
        w.commit()?;
        Ok(Self { db })
    }

    /// Durable (fsync) when this returns: persist BEFORE acting on a decision.
    pub fn put(&self, r: &ChannelRecord) -> Result<(), LedgerError> {
        let bytes = serde_json::to_vec(r)?;
        let w = self.db.begin_write()?;
        {
            let mut t = w.open_table(CHANNELS)?;
            t.insert(&r.id.0, bytes.as_slice())?;
        }
        w.commit()?;
        Ok(())
    }

    pub fn get(&self, id: &[u8; 32]) -> Result<Option<ChannelRecord>, LedgerError> {
        let r = self.db.begin_read()?;
        let t = r.open_table(CHANNELS)?;
        match t.get(id)? {
            Some(v) => Ok(Some(serde_json::from_slice(v.value())?)),
            None => Ok(None),
        }
    }

    /// EXERCISE (M4). SPOILER below.
    pub fn all(&self) -> Result<Vec<ChannelRecord>, LedgerError> {
        let r = self.db.begin_read()?;
        let t = r.open_table(CHANNELS)?;
        let mut out = Vec::new();
        for row in t.iter()? {
            let (_k, v) = row?;
            out.push(serde_json::from_slice(v.value())?);
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{Address, B256};
    use tw_core::{Action, Descriptor, Input, Policy, Voucher, step};

    fn rec() -> ChannelRecord {
        let d = Descriptor {
            payer: Address::repeat_byte(1),
            payee: Address::repeat_byte(2),
            operator: Address::repeat_byte(3),
            token: Address::repeat_byte(4),
            salt: B256::ZERO,
            authorized_signer: Address::ZERO,
            expiring_nonce_hash: B256::ZERO,
        };
        let mut r = ChannelRecord::new(B256::repeat_byte(7), d, 1_000_000);
        r.spent = 250_000;
        r.best_voucher = Some(Voucher {
            cumulative: 260_000,
            signature: vec![1; 65],
        });
        r
    }

    #[test]
    fn survives_reopen() -> Result<(), LedgerError> {
        let dir = tempfile::tempdir().expect("tempdir");
        let p = dir.path().join("l.redb");
        let r = rec();
        Ledger::open(&p)?.put(&r)?;
        assert_eq!(Ledger::open(&p)?.get(&r.id.0)?, Some(r));
        Ok(())
    }

    #[test]
    fn crash_with_close_in_flight_resubmits_after_restart() -> Result<(), LedgerError> {
        // R4: persist Closing{in_flight: true}, "crash", reload, feed Restarted -> SubmitClose.
        let dir = tempfile::tempdir().expect("tempdir");
        let p = dir.path().join("l.redb");
        let mut r = rec();
        let a = step(
            &mut r,
            Input::CloseRequested {
                grace_end: 2_000,
                requested_at_ms: 0,
            },
            1_000_000,
            &Policy::default(),
        );
        assert!(a.iter().any(|x| matches!(x, Action::SubmitClose(_))));
        Ledger::open(&p)?.put(&r)?; // process dies here, before CloseSent/CloseFailed
        let mut loaded = Ledger::open(&p)?.all()?;
        let r2 = &mut loaded[0];
        let a = step(r2, Input::Restarted, 1_060_000, &Policy::default());
        assert!(a.iter().any(|x| matches!(x, Action::SubmitClose(_))));
        Ok(())
    }
}
