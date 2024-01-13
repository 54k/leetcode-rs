# Author: Boris Korotaev
# tast 5
def double_values(m: dict[int, int]) -> dict[int, int]:
    ret = {}
    for k, v in m.items():
        if type(k) is int or type(k) is float:
            ret[k * 2] = v
        else:
            ret[k] = v
    return ret

if __name__ == "__main__":
    # 5
    print(double_values({1: "1", 2: "2", 3: "3", "4": "4"}))
    print(double_values({"4": "4"}))
    print(double_values({1.02: "1.02"}))
