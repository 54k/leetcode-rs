for _ in range(int(input())):
    s = [int(x) for x in input()]
    zeroes = sum([1 for x in s if x == 0])
    cnt = [0, 0]
    ans = 0
    for c in s:
        cnt[c]+=1
        if c == 0:
            ans += 1 if cnt[1] > 0 else 0
        else:
            ans += (zeroes - cnt[0])
    print(ans)