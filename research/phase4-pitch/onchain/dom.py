import json,sys,collections,datetime,urllib.request,concurrent.futures as cf
sys.path.insert(0,'/tmp/tw'); from k import keccak
E=lambda s:'0x'+keccak(s.encode())
TS=E('Settled(bytes32,address,address,uint96,uint96,uint96)'); TR=E('CloseRequested(bytes32,address,address,uint256)'); TC=E('ChannelClosed(bytes32,address,address,uint96,uint96)')
dom='ca4e835f803cb0b7c428222b3a3b98518d4779fe'
logs=[L for L in json.load(open('/tmp/tw/logs.json')) if L['topics'][0] in (TS,TR,TC) and L['topics'][3].endswith(dom)]
rows={r[0]:r for r in json.load(open('/tmp/tw/rows.json'))}
key=lambda L:(int(L['blockNumber'],16),int(L['logIndex'],16))
ev=collections.defaultdict(list)
for L in logs: ev[L['topics'][1]].append((key(L),L['topics'][0]))
res=[]
for cid,l in ev.items():
  l.sort(); rq=[i for i,x in enumerate(l) if x[1]==TR]
  if not rq: continue
  i=rq[-1]; after=l[i+1:]
  if not any(x[1]==TC for x in after): continue
  forced=cid in rows and rows[cid][1]==rows[cid][2]
  answered=(not forced) or any(x[1]==TS for x in after)
  res.append((l[i][0][0],answered))
def ts(b):
  r=urllib.request.Request('https://rpc.tempo.xyz',json.dumps({"jsonrpc":"2.0","id":1,"method":"eth_getBlockByNumber","params":[hex(b),False]}).encode(),{'content-type':'application/json','user-agent':'curl/8.4.0'})
  return int(json.load(urllib.request.urlopen(r,timeout=60))['result']['timestamp'],16)
with cf.ThreadPoolExecutor(10) as ex: tss=list(ex.map(lambda x:ts(x[0]),res))
by=collections.defaultdict(lambda:[0,0])
for (b,a),t in zip(res,tss):
  wk=datetime.datetime.utcfromtimestamp(t).strftime('%Y-%m-%d')[:7]+(' H1' if datetime.datetime.utcfromtimestamp(t).day<=15 else ' H2')
  by[wk][0 if a else 1]+=1
first_ans=min((t for (b,a),t in zip(res,tss) if a),default=None)
print('dominant payee resolved close requests:',len(res))
for k in sorted(by): print(' ',k,'answered',by[k][0],'unanswered',by[k][1])
print('first answered:',datetime.datetime.utcfromtimestamp(first_ans))
after=[a for (b,a),t in zip(res,tss) if t>=first_ans]; print('since first answer: answered',sum(after),'unanswered',len(after)-sum(after))
