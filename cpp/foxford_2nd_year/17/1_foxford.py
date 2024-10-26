w, h, n = map(int, input().split())
left = 0
right = n * max(w, h)
while right - left > 1:
    L = (right + left) // 2
    if (L // w) * (L // h) >= n:
        right = L
    else:
        left = L
print(right)