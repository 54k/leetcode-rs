n = int(input())
adj = [[] for _ in range(n)]
for i in range(n):
    row = list(map(int, input().split()))
    for j in range(n):
        if row[j]:
            adj[i].append(j+1)

for r in adj:
    print(len(r), " ".join(map(str, sorted(r))))