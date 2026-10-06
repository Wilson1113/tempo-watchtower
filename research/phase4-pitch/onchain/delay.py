import json,sys,collections,statistics,urllib.request,concurrent.futures as cf
sys.path.insert(0,'/tmp/tw'); from k import keccak
E=lambda s:'0x'+keccak(s.encode())
TS=E('Settled(bytes32,address,address,uint96,uint96,uint96)'); TR=E('CloseRequested(bytes32,address,address,uint256)'); TC=E('ChannelClosed(bytes32,address,address,uint96,uint96)')
logs=json.load(open('/tmp/tw/logs.json')); key=lambda L:(int(L['blockNumber'],16),int(L['logIndex'],16))
rows={r[0]:r for r in json.load(open('/tmp/tw/rows.json'))}
ev=collections.defaultdict(list)
for L in logs:
  if L['topics'][0] in (TS,TR,TC): ev[L['topics'][1]].append((key(L),L['topics'][0]))
pairs=[]
for cid,l in ev.items():
  l.sort(); rq=[i for i,x in enumerate(l) if x[1]==TR]
  if not rq: continue
  i=rq[-1]; s=[x for x in l[i+1:] if x[1]==TS]; c=[x for x in l[i+1:] if x[1]==TC]
  if c and s and cid in rows and rows[cid][1]==rows[cid][2]: pairs.append((l[i][0][0],s[0][0][0]))
def ts(b):
  r=urllib.request.Request('https://rpc.tempo.xyz',json.dumps({"jsonrpc":"2.0","id":1,"method":"eth_getBlockByNumber","params":[hex(b),False]}).encode(),{'content-type':'application/json','user-agent':'curl/8.4.0'})
  return int(json.load(urllib.request.urlopen(r,timeout=60))['result']['timestamp'],16)
with cf.ThreadPoolExecutor(10) as ex:
  d=list(ex.map(lambda p: ts(p[1])-ts(p[0]), pairs))
print('in-grace settle responses',len(d),'delay s: min',min(d),'median',statistics.median(d),'max',max(d))
