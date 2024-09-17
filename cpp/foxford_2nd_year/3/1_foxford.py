a = int(input())
b = int(input())
c = int(input())
k = min(a, b // 2, c)
a -= k
b -= 2 * k
c -= k
if a == 0:
    print(4 * k)
elif b == 0:
    print(4 * k + 1)
elif c == 0:
    print(4 * k + 2)
else:
    print(4 * k + 3)