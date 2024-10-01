x, n, p = map(int, input().split())

result = 1
while n:
    if n % 2 == 1:
        result = (result * x) % p
        n -= 1
    x = (x * x) % p
    n //= 2

print(result)
