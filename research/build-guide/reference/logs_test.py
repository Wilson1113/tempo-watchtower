import json,sys,urllib.request
sys.path.insert(0,'/tmp/tw'); from k import keccak
R='https://rpc.moderato.tempo.xyz'; P='0x4d50500000000000000000000000000000000000'
def rpc(m,p):
  r=urllib.request.Request(R,json.dumps({"jsonrpc":"2.0","id":1,"method":m,"params":p}).encode(),{'content-type':'application/json','user-agent':'curl/8.4.0'})
  return json.load(urllib.request.urlopen(r,timeout=90))
EV=['ChannelOpened(bytes32,address,address,address,address,address,bytes32,bytes32,uint96)','Settled(bytes32,address,address,uint96,uint96,uint96)','CloseRequested(bytes32,address,address,uint256)','ChannelClosed(bytes32,address,address,uint96,uint96)','CloseRequestCancelled(bytes32,address,address)','TopUp(bytes32,address,address,uint96,uint96)']
T={'0x'+keccak(e.encode()):e.split('(')[0] for e in EV}
for t,n in T.items(): print(n,t)
head=int(rpc('eth_blockNumber',[])['result'],16); print('head',head)
for span in [1000,10000,100000,500000]:
  r=rpc('eth_getLogs',[{"address":P,"fromBlock":hex(head-span),"toBlock":hex(head)}])
  if 'error' in r: print(span,'ERR',r['error']); continue
  from collections import Counter
  c=Counter(T.get(L['topics'][0],L['topics'][0][:10]) for L in r['result']); print(span,len(r['result']),dict(c))
  if span==10000: sample=r['result']
# getChannelStatesBatch on some recent channel ids
cids=list({L['topics'][1] for L in sample})[:3]
from_sel='0x'+keccak(b'getChannelStatesBatch(bytes32[])')[:8]
data=from_sel+'%064x'%32+'%064x'%len(cids)+''.join(c[2:] for c in cids)
print('sel',from_sel, rpc('eth_call',[{"to":P,"data":data},"latest"]))
print(json.dumps(sample[0],indent=1)[:1500])
