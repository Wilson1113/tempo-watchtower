# Dev shortcuts. `make` alone runs fmt-check + clippy + test.
RPC       ?= https://rpc.moderato.tempo.xyz
ADDR_FILE ?= /tmp/tw-addresses.txt
TOKEN     ?= 0x20c0000000000000000000000000000000000000

.PHONY: ci fmt fmt-check clippy test grace probe daemon fund balances

ci: fmt-check clippy test

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

test:
	cargo test --workspace

# Prints CLOSE_GRACE_PERIOD() (expect 900).
grace:
	cargo run -p tw-demo -- grace

# Open, cancel pre-check, operator close.
probe:
	cargo run -p tw-chain --example probe

daemon:
	cargo run -p tw-daemon

# Testnet faucet for every address in $(ADDR_FILE).
fund:
	@addrs=$$(grep -oE '0x[0-9a-fA-F]{40}' $(ADDR_FILE)); \
	[ -n "$$addrs" ] || { echo "no addresses in $(ADDR_FILE)"; exit 1; }; \
	for a in $$addrs; do \
	  echo "fund $$a"; \
	  curl -s -X POST -H 'content-type: application/json' \
	    --data "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"tempo_fundAddress\",\"params\":[\"$$a\"]}" $(RPC); echo; \
	done

# balanceOf(addr) on $(TOKEN) for every address in $(ADDR_FILE).
balances:
	@addrs=$$(grep -oE '0x[0-9a-fA-F]{40}' $(ADDR_FILE)); \
	[ -n "$$addrs" ] || { echo "no addresses in $(ADDR_FILE)"; exit 1; }; \
	for a in $$addrs; do \
	  printf '%s ' "$$a"; \
	  curl -s -X POST -H 'content-type: application/json' \
	    --data "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"eth_call\",\"params\":[{\"to\":\"$(TOKEN)\",\"data\":\"0x70a08231000000000000000000000000$${a#0x}\"},\"latest\"]}" $(RPC); echo; \
	done
