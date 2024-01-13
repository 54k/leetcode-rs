# Author: Boris Korotaev
# task 6
def count_strings(strs: list[str]) -> dict[str, int]:
    ret = {}
    for s in [x.lower() for x in strs]:
        ret[s] = ret.get(s, 0) + 1
    return ret

if __name__ == "__main__":
    # 6
    print(count_strings(["one", "TWO", "two"]))
    print(count_strings(["three", "THRee", "THREE"]))
