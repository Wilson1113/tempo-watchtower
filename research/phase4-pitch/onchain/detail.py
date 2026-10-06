import json,sys,collections,urllib.request,datetime
sys.path.insert(0,'/tmp/tw'); from k import keccak
logs=json.load(open('/tmp/tw/logs.json')); rows=json.load(open('/tmp/tw/rows.json'))
TS='0x'+keccak(b'Settled(bytes32,address,address,uint96,uint96,uint96)')
TO='0x'+keccak(b'ChannelOpened(bytes32,address,address,address,address,address,bytes32,bytes32,uint96)')
settled_ch=collections.Counter(L['topics'][1] for L in logs if L['topics'][0]==TS)
opener={L['topics'][1]:L for L in logs if L['topics'][0]==TO}
forced=[r for r in rows if r[1]==r[2]]
payees=collections.Counter(r[3] for r in forced)
used=[r for r in forced if settled_ch[r[0]]>0 or r[4]>0]
print('forced withdraws',len(forced),'distinct payees affected',len(payees),payees.most_common(6))
print('forced withdraws on channels with >=1 prior settle (service provably used):',len(used),' refunded $',sum(r[5] for r in used)/1e6)
ops=[r for r in forced if int(opener[r[0]]['data'][2:66],16)!=0]
print('forced withdraws on channels that HAD an operator set:',len(ops))
RPC='https://rpc.tempo.xyz'
def ts(b):
  r=urllib.request.Request(RPC,json.dumps({"jsonrpc":"2.0","id":1,"method":"eth_getBlockByNumber","params":[hex(b),False]}).encode(),{'content-type':'application/json','user-agent':'curl/8.4.0'})
  return datetime.datetime.utcfromtimestamp(int(json.load(urllib.request.urlopen(r,timeout=60))['result']['timestamp'],16)).strftime('%Y-%m-%d %H:%M')
first=min(int(L['blockNumber'],16) for L in logs); last=max(int(L['blockNumber'],16) for L in logs)
print('first channel event',ts(first),' last',ts(last))
fts=sorted(r[6] for r in forced); f=lambda t:datetime.datetime.utcfromtimestamp(t).strftime('%Y-%m-%d')
print('forced withdraw dates: first',f(fts[0]),'last',f(fts[-1]))
# weekly opens excluding dominant payee
dom='0xca4e835f803cb0b7c428222b3a3b98518d4779fe'
bw=collections.Counter()
for L in logs:
  if L['topics'][0]==TO and '0x'+L['topics'][3][-40:]!=dom: bw[int(L['blockNumber'],16)//1_200_000]+=1
print('channels opened (excl. dominant payee) per ~1.2M-block bucket:',[bw[k] for k in sorted(bw)])
