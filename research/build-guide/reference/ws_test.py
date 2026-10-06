import asyncio, json, websockets, sys, time
async def probe(url, sub_params, wait=8):
    try:
        async with websockets.connect(url, open_timeout=10, user_agent_header="curl/8.4.0") as ws:
            await ws.send(json.dumps({"jsonrpc":"2.0","id":1,"method":"eth_chainId","params":[]}))
            print(url, "chainId:", await asyncio.wait_for(ws.recv(), 10))
            await ws.send(json.dumps({"jsonrpc":"2.0","id":2,"method":"eth_subscribe","params":sub_params}))
            print(" subscribe:", await asyncio.wait_for(ws.recv(), 10))
            t0=time.time(); n=0
            while time.time()-t0 < wait:
                try:
                    m = await asyncio.wait_for(ws.recv(), wait)
                    n+=1
                    if n<=2: print(" msg:", m[:300])
                except asyncio.TimeoutError: break
            print(" messages in", wait, "s:", n)
    except Exception as e:
        print(url, "ERR", type(e).__name__, e)
async def main():
    for u in ["wss://rpc.moderato.tempo.xyz", "wss://rpc.moderato.tempo.xyz/ws", "wss://rpc.tempo.xyz"]:
        await probe(u, ["newHeads"], 4)
    # logs on the precompile (mainnet has traffic)
    await probe("wss://rpc.tempo.xyz", ["logs", {"address":"0x4d50500000000000000000000000000000000000"}], 30)
    await probe("wss://rpc.moderato.tempo.xyz", ["logs", {"address":"0x4d50500000000000000000000000000000000000"}], 20)
asyncio.run(main())
