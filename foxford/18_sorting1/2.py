input()
jobs = list(map(int, input().split()))
idx = [i for j, i in reversed(sorted([(job, i) for i, job in enumerate(jobs)], key=lambda x: x[0]))]
s = 0
for i, t in enumerate(idx):
    s += jobs[t] * (i + 1)

print(s)
for i in idx:
    print(i+1, end=" ")