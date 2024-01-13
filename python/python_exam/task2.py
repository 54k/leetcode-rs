# Author: Boris Korotaev
# task 2
def check_conditions(x: int, y: int, z: int) -> bool:
    return x == y or x > z or y <= z

if __name__ == "__main__":
    # 2
    print(check_conditions(1, 2, 3))
    print(check_conditions(1, 3, 2))
    print(check_conditions(2, 3, 1))
    print(check_conditions(2, 2, 1))
