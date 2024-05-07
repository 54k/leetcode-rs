for _ in range(int(input())):
    n = int(input())
    p = [int(x)-1 for x in input().split()]
    ans = 3
    for i in range(n):
        if p[p[i]] == i:
            ans = 2
    print(ans)