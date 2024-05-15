ans = 0

def generate(n, y):
    global ans
    if len(y) == n:
        ans += 1
    for next_y in range(n):
        for i in range(len(y)):
            if y[i] == next_y or abs(y[i] - next_y) == len(y) - i:
                break
            else:
                generate(n, y + [next_y])

generate(int(input()), [])
print(ans)