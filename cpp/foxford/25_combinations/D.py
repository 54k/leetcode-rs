n = int(input())
d = 2
while d * d <= n and n % d != 0:
    d += 1
if n % d == 0:
    print(n // d, n // d * (d - 1))
else:
    print(1, n - 1)