# Даны три массива a , b , c и число x .
# Посчитать число троек  (i, j, k) , таких что
# a[i] + b[j] + c[k] == x
# Массивы можно модифицировать inplace.

#  sorted arrays
#  mem const, complexibility = n^2

a = [1, 2, 3, 4, 5, 6, 7]
b = [2, 3, 5, 7, 11]
c = [1, 1, 2, 3, 5, 7]
a_len = len(a)
b_len = len(b)
c_len = len(c)

x = 15

for k in range(c_len):
    # Find s - c[k] in top-to-tail merged a and b
    # находим s - c[k] в отсортированных массивах
    i = 0
    j = b_len - 1

    while i < a_len and j >= 0:
        if a[i] + b[j] + c[k] < x:
            # Move forward a
            i += 1
        elif a[i] + b[j] + c[k] > x:
            # Move back b
            j -= 1
        else:
            # Found
            print(a[i] + b[j] + c[k], "=", a[i], "+", b[j], "+", c[k])
            i += 1
            j -= 1
