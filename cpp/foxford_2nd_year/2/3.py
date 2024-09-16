n = int(input())
delims = []
for i in range(1, n+1):
    if n % i == 0:
        delims.append(i)
print(" ".join([str(x) for x in delims]))