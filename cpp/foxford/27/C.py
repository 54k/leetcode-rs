n, k = map(int, input().split())

# 1
cur = []
def rec():
    if len(cur) == n:
        print(" ".join(map(str, cur)))
        return
    
    for i in range(0, k):
        cur.append(i)
        rec()
        cur.pop()

rec()

# 2
def generate(n, k, prefix):
    if n == 0:
        print(prefix)
    else:
        for next in range(k):
            generate(n - 1, k, prefix + str(next) + " ")

n, k = map(int, input().split())
generate(n, k, "")
