RC=[0x1,0x8082,0x800000000000808a,0x8000000080008000,0x808b,0x80000001,0x8000000080008081,0x8000000000008009,0x8a,0x88,0x80008009,0x8000000a,0x8000808b,0x800000000000008b,0x8000000000008089,0x8000000000008003,0x8000000000008002,0x8000000000000080,0x800a,0x800000008000000a,0x8000000080008081,0x8000000000008080,0x80000001,0x8000000080008008]
R=[[0,36,3,41,18],[1,44,10,45,2],[62,6,43,15,61],[28,55,25,21,56],[27,20,39,8,14]]
M=(1<<64)-1
rot=lambda x,n:((x<<n)|(x>>(64-n)))&M if n else x
def _f(A):
  for rc in RC:
    C=[A[x][0]^A[x][1]^A[x][2]^A[x][3]^A[x][4] for x in range(5)]
    D=[C[(x-1)%5]^rot(C[(x+1)%5],1) for x in range(5)]
    A=[[A[x][y]^D[x] for y in range(5)] for x in range(5)]
    B=[[0]*5 for _ in range(5)]
    for x in range(5):
      for y in range(5): B[y][(2*x+3*y)%5]=rot(A[x][y],R[x][y])
    A=[[B[x][y]^((~B[(x+1)%5][y])&B[(x+2)%5][y]) for y in range(5)] for x in range(5)]
    A[0][0]^=rc
  return A
def keccak(m):
  r=136; m=bytearray(m)+b'\x01'; m+=b'\x00'*((-len(m))%r); m[-1]|=0x80
  A=[[0]*5 for _ in range(5)]
  for i in range(0,len(m),r):
    for j in range(r//8):
      A[j%5][j//5]^=int.from_bytes(m[i+8*j:i+8*j+8],'little')
    A=_f(A)
  return b''.join(A[j%5][j//5].to_bytes(8,'little') for j in range(4)).hex()
