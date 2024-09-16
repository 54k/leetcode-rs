n = int(input())
isok = "YES" if (n % 4 == 0  and n % 100 != 0) or (n % 400 == 0) else "NO"
print(isok)
    