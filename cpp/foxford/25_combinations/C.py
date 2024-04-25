import sys

sys.setrecursionlimit(100010)

def generate(n, k, prefix=[]):
    if k == 0:
        print(" ".join(map(str, prefix)))
    else:
        if len(prefix) == 0:
            next = n
        else:
            next = prefix[-1] - 1

        for i in range(k, next+1):
            generate(n,k-1,prefix+[i])


n, k = map(int, input().split())
generate(n, k)