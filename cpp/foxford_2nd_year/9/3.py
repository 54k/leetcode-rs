def sieve_of_eratosthenes(n):
    primes = [True] * (n + 1)
    primes[0] = primes[1] = False
    for p in range(2, int(n ** 0.5) + 1):
        if primes[p]:
            for multiple in range(p * p, n + 1, p):
                primes[multiple] = False
    prime_numbers = [i for i in range(2, n+1) if primes[i]]
    return prime_numbers

n = int(input())
primes = sieve_of_eratosthenes(n)
print(" ".join([str(i) for i in primes]))
