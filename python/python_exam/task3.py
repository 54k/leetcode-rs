# Author: Boris Korotaev
# task 3
def sum_even_numbers(arr: list[int]):
    return sum([x for x in arr if not x % 2])

if __name__ == "__main__":
    # 3
    print(sum_even_numbers([1, 2, 3, 4]))
    print(sum_even_numbers([1, 1, 1, 1]))
