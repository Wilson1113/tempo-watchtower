import json,sys,collections,statistics
sys.path.insert(0,'/tmp/tw'); from k import keccak
E=lambda s:'0x'+keccak(s.encode())
TO=E('ChannelOpened(bytes32,address,address,address,address,address,bytes32,bytes32,uint96)')
TS=E('Settled(bytes32,address,address,uint96,uint96,uint96)')
TR=E('CloseRequested(bytes32,address,address,uint256)')
TC=E('ChannelClosed(bytes32,address,address,uint96,uint96)')
TX=E('CloseRequestCancelled(bytes32,address,address)')
logs=json.load(open('/tmp/tw/logs.json'))
key=lambda L:(int(L['blockNumber'],16),int(L['logIndex'],16))
rows={r[0]:r for r in json.load(open('/tmp/tw/rows.json'))}  # cid -> [cid,from,payer,payee,settled,refund,ts,graceEnd]
ev=collections.defaultdict(list); desc={}
for L in logs:
  t=L['topics'][0]
  if t in (TS,TR,TC,TX): ev[L['topics'][1]].append((key(L),t,L))
  elif t==TO: desc[L['topics'][1]]=('0x'+L['topics'][3][-40:], '0x'+L['data'][2+24:2+64])  # payee, operator
out=collections.Counter(); resp_delay=[]; noresp_used=[]; noresp=[]; per_payee=collections.Counter(); dom='0xca4e835f803cb0b7c428222b3a3b98518d4779fe'
for cid,lst in ev.items():
  lst.sort(); reqs=[i for i,(k,t,L) in enumerate(lst) if t==TR]
  if not reqs: continue
  i=reqs[-1]; rk=lst[i][0]
  after=lst[i+1:]
  settle_in=[x for x in after if x[1]==TS]
  closed=[x for x in after if x[1]==TC]
  prior_settle=any(x[1]==TS for x in lst[:i])
  payee,op=desc.get(cid,('?','?'))
  if not closed: out['unresolved (no close yet)']+=1; continue
  r=rows.get(cid)
  forced = r is not None and r[1]==r[2]
  if not forced: out['closed by payee/operator']+=1; continue
  if settle_in:
    out['forced withdraw, but payee-side settled in grace (responded)']+=1
  else:
    out['forced withdraw, NO payee-side response']+=1; noresp.append(r); per_payee[r[3]]+=1
    if prior_settle: noresp_used.append(r)
print(dict(out))
print('no-response forced withdraws:',len(noresp),'distinct payees',len(per_payee))
print('  of which dominant payee:',per_payee[dom])
print('provably-used & unanswered:',len(noresp_used),'payees',len({r[3] for r in noresp_used}),'refunded $',round(sum(r[5] for r in noresp_used)/1e6,2))
refs=sorted(r[5]/1e6 for r in noresp); print('refund per unanswered withdraw: median $',statistics.median(refs),' max $',refs[-1],' total $',round(sum(refs),2))
ops=[(p,o) for p,o in desc.values() if int(o,16)!=0]; print('operator channels',len(ops),'self-assigned (op==payee)',sum(1 for p,o in ops if p==o),'delegated',sum(1 for p,o in ops if p!=o),'distinct delegated operators',len({o for p,o in ops if p!=o}))
