import json,sys,collections
sys.path.insert(0,'/tmp/tw'); from k import keccak
EV={'ChannelOpened':'ChannelOpened(bytes32,address,address,address,address,address,bytes32,bytes32,uint96)',
    'Settled':'Settled(bytes32,address,address,uint96,uint96,uint96)',
    'CloseRequested':'CloseRequested(bytes32,address,address,uint256)',
    'ChannelClosed':'ChannelClosed(bytes32,address,address,uint96,uint96)',
    'CloseRequestCancelled':'CloseRequestCancelled(bytes32,address,address)',
    'TopUp':'TopUp(bytes32,address,address,uint96,uint96)'}
T={'0x'+keccak(v.encode()):k for k,v in EV.items()}
logs=json.load(open('/tmp/tw/logs.json'))
w=lambda d,i:int(d[2+64*i:2+64*(i+1)],16)
a=lambda t:'0x'+t[-40:]
cnt=collections.Counter(); payees=set(); payers=set(); ops=collections.Counter(); tokens=collections.Counter()
dep=collections.Counter(); paid=collections.Counter(); chan_token={}; opened=set()
cr=collections.defaultdict(list); closed={}; cancelled=collections.Counter(); blocks=[]
for L in logs:
  k=T.get(L['topics'][0]); cnt[k]+=1; b=int(L['blockNumber'],16); blocks.append(b)
  cid=L['topics'][1]; d=L['data']
  if k=='ChannelOpened':
    opened.add(cid); payers.add(a(L['topics'][2])); payees.add(a(L['topics'][3]))
    op='0x'+hex(w(d,0))[2:].zfill(40); tok='0x'+hex(w(d,1))[2:].zfill(40)
    if int(op,16): ops[op]+=1
    tokens[tok]+=1; chan_token[cid]=tok; dep[tok]+=w(d,5)
  elif k=='TopUp': dep[chan_token.get(cid,'?')]+=w(d,0)
  elif k=='Settled': paid[chan_token.get(cid,'?')]+=w(d,1)
  elif k=='CloseRequested': cr[cid].append((b,w(d,0),L['transactionHash']))
  elif k=='CloseRequestCancelled': cancelled[cid]+=1
  elif k=='ChannelClosed': closed[cid]=(b,w(d,0),w(d,1),L['transactionHash'],a(L['topics'][2]),a(L['topics'][3]))
print('events',dict(cnt)); print('block range',min(blocks),max(blocks))
print('channels opened',len(opened),'unique payees',len(payees),'unique payers',len(payers))
print('channels with non-zero operator',sum(ops.values()),'unique operators',len(ops), ops.most_common(3))
print('tokens',tokens.most_common(5))
for t,v in dep.most_common(5): print('deposits',t,v/1e6)
for t,v in paid.most_common(5): print('settled deltaPaid',t,v/1e6)
print('channels with close requests',len(cr),'cancelled',sum(cancelled.values()))
# closes after a close request -> candidates for forced withdraw
cand=[(cid,closed[cid]) for cid in cr if cid in closed]
print('closed channels that had close request',len(cand))
json.dump({'cand':[[c,list(v)] for c,v in cand],'cr':{c:v for c,v in cr.items()}},open('/tmp/tw/cand.json','w'))
# payee concentration
pc=collections.Counter(a(L['topics'][3]) for L in logs if T.get(L['topics'][0])=='ChannelOpened')
print('top payees by channels',[(p,n) for p,n in pc.most_common(8)])
