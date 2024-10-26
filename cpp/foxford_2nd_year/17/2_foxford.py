M, N = map(int, input().split())
T = [0] * N
Z = [0] * N
Y = [0] * N
for i in range(N):
    T[i], Z[i], Y[i] = map(int, input().split())
def check(t):
    cnt = 0
    for i in range(N):
        cnt += (t // (T[i] * Z[i] + Y[i])) * Z[i] + min((t % (T[i] * Z[i] + Y[i]) // T[i]), Z[i])
    return cnt >= M
left = -1
right = 10 ** 6
while right - left > 1:
    t = (left + right) // 2
    if check(t):
        right = t
    else:
        left = t
print(right)