import json,sys,urllib.request,concurrent.futures as cf,collections
sys.path.insert(0,'/tmp/tw'); from k import keccak
RPC='https://rpc.tempo.xyz'; P='0x4d50500000000000000000000000000000000000'
EV={'ChannelOpened':'ChannelOpened(bytes32,address,address,address,address,address,bytes32,bytes32,uint96)',
    'Settled':'Settled(bytes32,address,address,uint96,uint96,uint96)',
    'CloseRequested':'CloseRequested(bytes32,address,address,uint256)',
    'ChannelClosed':'ChannelClosed(bytes32,address,address,uint96,uint96)',
    'CloseRequestCancelled':'CloseRequestCancelled(bytes32,address,address)',
    'TopUp':'TopUp(bytes32,address,address,uint96,uint96)'}
T={'0x'+keccak(v.encode()):k for k,v in EV.items()}
def rpc(m,p):
  r=urllib.request.Request(RPC,json.dumps({"jsonrpc":"2.0","id":1,"method":m,"params":p}).encode(),{'content-type':'application/json','user-agent':'curl/8.4.0'})
  for _ in range(4):
    try: return json.load(urllib.request.urlopen(r,timeout=90))
    except Exception as e: err=e
  raise err
head=int(rpc('eth_blockNumber',[])['result'],16)
STEP=100000
def chunk(s):
  e=min(s+STEP-1,head); r=rpc('eth_getLogs',[{"address":P,"fromBlock":hex(s),"toBlock":hex(e),"topics":[list(T)]}])
  if 'error' in r: return ('ERR',s,r['error'])
  return r['result']
logs=[];errs=[]
with cf.ThreadPoolExecutor(8) as ex:
  for res in ex.map(chunk,range(0,head+1,STEP)):
    if isinstance(res,tuple): errs.append(res)
    else: logs+=res
json.dump(logs,open('/tmp/tw/logs.json','w'))
print('head',head,'logs',len(logs),'errors',len(errs), errs[:2])
