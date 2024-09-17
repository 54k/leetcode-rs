s = int(input())
n = int(input())
s1 = 0
s2 = 0
for i in range(n):
    a = int(input())
    if s1 + a <= s:
        s1 += a
    else:
        s2 += a
print(s1, s2)