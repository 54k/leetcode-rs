n = int(input())
# 1
def generate(n, prefix):
    if n == 0:
        print(" ".join(map(str, prefix)))
        return
    if len(prefix) < 2 or prefix[-2:] == [0, 1] or prefix[-2:] == [1, 0]:
        generate(n-1, prefix + [0])
        generate(n-1, prefix + [1])
    elif prefix[-2:] == [1, 1]:
        generate(n-1, prefix + [0])
    else:
        generate(n-1, prefix + [1])

generate(n, [])

# 2
def generate2(n, prefix):
    if n == 0:
        print(" ".join(prefix))
    else:
        if len(prefix) < 2 or prefix[-2:] != "00":
            generate2(n-1, prefix+"0")
        if len(prefix) < 2 or prefix[-2:] != "11":
            generate(n-1, prefix+"1")

generate2(n, "")