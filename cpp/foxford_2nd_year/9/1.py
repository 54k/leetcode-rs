a, b = map(int, input().split())
def gcd(a, b):
    if b == 0: return a
    return gcd(b, a % b)
lcm = a*b//gcd(a,b)
print(lcm)