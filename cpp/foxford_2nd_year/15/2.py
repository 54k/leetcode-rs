n = int(input())
max=0
for i in range(n):
    a = int(input())
    if a % 10 == 7 and a > max:
        max = a
print(max)