n, a, b, w, h = map(int, input().split())
left = 0
right = min(w, h)
while left + 1 < right:
    d = (left + right) // 2
    x = max((w // (a + 2 * d)) * (h // (b + 2 * d)), (h // (a + 2 * d)) * (w // (b + 2 * d)))
    if x >= n:
        left = d
    else:
        right = d
print(left)