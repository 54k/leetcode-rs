def generate(n, k, prefix=""):
    if n == 0:
        print(prefix[1:])
    else:
        generate(n-1, k, prefix + " 0")
        if k > 0:
            generate(n-1, k-1, prefix + " 1")

n, k = map(int,input().split())
generate(n, k)
