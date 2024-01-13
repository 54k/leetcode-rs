# Author: Boris Korotaev
# tast 5
def double_values_i(m: dict[int, int]) -> dict[int, int]:
    ret = {}
    for k, v in m.items():
        if type(k) is int or type(k) is float:
            ret[k * 2] = v
        else:
            ret[k] = v
    return ret

# tast 5
def double_values_ii(m: dict[int, int]) -> dict[int, int]:
    ret = {}
    for k, v in m.items():
        if type(v) is int or type(v) is float:
            ret[k] = 2 * v
        else:
            ret[k] = v
    return ret

if __name__ == "__main__":
    # 5
    print("Double map keys:")
    print(double_values_i({1: 1, 2: 2, 3: "3", "4": "4"}))
    print(double_values_i({"4": "4"}))
    print(double_values_i({1.02: 1.02}))

    print("Double map values:")
    print(double_values_ii({1: 1, 2: "2", 3: 3, "4": "4"}))
    print(double_values_ii({"4": "4"}))
    print(double_values_ii({1.02: 1.02}))
