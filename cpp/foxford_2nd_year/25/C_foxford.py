# Решение задачи:

# Решим задачу при помощи рекурсивного перебора, модифицировав алгоритм построения всех возрастающих последовательностей длины
# из чисел от 1 до . Будем перебирать следующее число, которое можно добавить к уже построенному началу последовательности prefix.
# Его минимальное значение равно , а максимальное — на 1 меньше последнего элемента prefix (или

# , если длина prefix равна 0).

def generate(n, k, prefix):
    if k == 0:
        print(" ".join(map(str, prefix)))
    else:
        if len(prefix) == 0:
            max = n
        else:
            max = prefix[-1] - 1
        for next in range(k, max + 1):
            generate(n, k - 1, prefix + [next])
n, k = map(int, input().split())
generate(n, k, [])