import json,urllib.request,concurrent.futures as cf,collections,datetime
RPC='https://rpc.tempo.xyz'
def rpc(m,p):
  r=urllib.request.Request(RPC,json.dumps({"jsonrpc":"2.0","id":1,"method":m,"params":p}).encode(),{'content-type':'application/json','user-agent':'curl/8.4.0'})
  for _ in range(4):
    try: return json.load(urllib.request.urlopen(r,timeout=60))['result']
    except Exception as e: err=e
  raise err
C=json.load(open('/tmp/tw/cand.json')); cand=C['cand']; cr=C['cr']
def info(item):
  cid,(b,settled,refund,txh,payer,payee)=item
  tx=rpc('eth_getTransactionByHash',[txh]); frm=(tx or {}).get('from','').lower()
  req_b,grace_end,_=cr[cid][-1]
  blk=rpc('eth_getBlockByNumber',[hex(b),False]); ts=int(blk['timestamp'],16)
  return cid,frm,payer.lower(),payee.lower(),settled,refund,ts,grace_end
with cf.ThreadPoolExecutor(12) as ex: rows=list(ex.map(info,cand))
kind=collections.Counter(); forced=[]; resp=[]
for cid,frm,payer,payee,s,r,ts,ge in rows:
  if frm==payer: kind['forced withdraw (payer)']+=1; forced.append((s,r))
  elif frm==payee: kind['closed by payee']+=1; resp.append(ts-(ge-900))
  else: kind['closed by other (operator/relayer)']+=1; resp.append(ts-(ge-900))
print(dict(kind))
print('forced withdraws: settled-before-withdraw==0:',sum(1 for s,r in forced if s==0),' refunded to payers $',sum(r for s,r in forced)/1e6,' payee had settled $',sum(s for s,r in forced)/1e6)
resp.sort(); print('payee/operator close delay after request (s): min',resp[:1],'median',resp[len(resp)//2] if resp else None,'max',resp[-1:])
json.dump(rows,open('/tmp/tw/rows.json','w'))
# timeline of openings
