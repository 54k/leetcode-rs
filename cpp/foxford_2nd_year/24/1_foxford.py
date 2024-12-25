ans = 0

def move(n, start, finish):
     global ans
     if n > 0:
         temp = 6 - start - finish # Вспомогательный стержень
         move(n - 1, start, temp)
         ans += 1
         move(n - 1, temp, finish)

n = int(input())
move(n, 1, 2)
print(ans)