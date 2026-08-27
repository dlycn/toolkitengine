import sys
strnum=sys.stdin.readline().strip()
strlist= sys.stdin.readline().strip()
num = int(strnum)
t:list = list(map(int,strlist.split()))
lt = [t[0] for _ in range(num)]
rt = [0 for _ in range(num)]
pl = [0 for _ in range(num)]
nl = lt[0]
pr  
for i in range(num-1):
    i+=1
    rt[i] = t[i] if t[i]>rt[i] else rt[i-1]
    lt[i] = t[i] if t[i]>lt[i] else lt[i-1]
    op[i] = n
    if rt[i]>n:
        n = rt[i]
print(rt,lt,op)