import math

def max_divisor(n):
    max_d = 1
    for i in range(1, int(math.sqrt(n)) + 1):
        if n % i == 0:
            # i — делитель
            if i != n:
                max_d = max(max_d, i)
            # n // i — другой делитель
            if n // i != n:
                max_d = max(max_d, n // i)
    return max_d

# Чтение данных
n = int(input())
print(max_divisor(n))
