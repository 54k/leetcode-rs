def main():
    A, K, B, M, X = map(int, input().split())

    lo = 0
    hi = 10 ** 18

    while lo + 1 < hi:
        mid = (lo + hi) // 2
        cnt = A * (mid - mid // K) + B * (mid - mid // M)
        if cnt < X:
            lo = mid
        else:
            hi = mid
    print(hi)

if __name__ == '__main__':
    main()
