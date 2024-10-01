n = int(input())
curr = 1
ok = False
i = 1

while i <= n:
    if curr % n == 0:
        ok = True
        break
    curr = (curr * 10 + 1)%n
    i += 1

if ok:
    print("".join(["1"] * i))
else:
    print("NO")

