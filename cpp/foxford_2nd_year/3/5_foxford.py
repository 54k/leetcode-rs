x1 = int(input())
y1 = int(input())
x2 = int(input())
y2 = int(input())
x3 = int(input())
y3 = int(input())
x4 = int(input())
y4 = int(input())

left = max(x1, x3)
right = min(x2, x4)
bottom = max(y1, y3)
top = min(y2, y4)
if left < right and bottom < top:
    print((right - left) * (top - bottom))
else:
   print(0)