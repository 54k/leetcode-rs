# Author: Boris Korotaev
# task 4
def fibonacci(n: int) -> list[int]:
    assert n > 0
    if n == 1:
        return [0]

    ret = [0] * n
    ret[1] = 1
    if n == 2:
        return ret

    i = 2
    while i < n:
        ret[i] = ret[i - 1] + ret[i - 2]
        i += 1
    return ret

if __name__ == "__main__":
    # 4
    print(fibonacci(1))
    print(fibonacci(2))
    print(fibonacci(3))
    print(fibonacci(10))
