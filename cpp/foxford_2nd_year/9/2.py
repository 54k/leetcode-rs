def extended_gcd(a, b):
    if b == 0:
        return a, 1, 0
    gcd, x1, y1 = extended_gcd(b, a % b)
    x = y1
    y = x1 - (a // b) * y1
    return gcd, x, y

def solve(a, b, c):
    gcd, x, y = extended_gcd(a, b)
    
    # Проверка, существует ли решение
    if c % gcd != 0:
        print("Impossible")
    else:
        # Масштабируем коэффициенты
        x *= c // gcd
        y *= c // gcd
        print(gcd, x, y)

# Чтение данных
a, b, c = map(int, input().split())
solve(a, b, c)


