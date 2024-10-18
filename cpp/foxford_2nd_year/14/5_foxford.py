A = list(map(int, input().split()))
S = []
i = 0
while i < len(A):
    S.append(A[i])
    if len(S) >= 3 and S[-1] == S[-2] == S[-3]:
        color = S[-1]
        S.pop()
        S.pop()
        S.pop()
        while i < len(A) and A[i] == color:
            i += 1
    else:
        i += 1
print(len(A) - len(S))