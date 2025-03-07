package leetcode_grind;

import java.util.Arrays;

public class Day839 {
    // https://leetcode.com/problems/closest-prime-numbers-in-range/description/?envType=daily-question&envId=2025-03-07
    static class Solution1 {
        public int[] closestPrimes(int left, int right) {
            var n = right + 1;
            var sieve = new boolean[n];
            Arrays.fill(sieve, true);
            sieve[1] = false;
            sieve[0] = false;
            for (int d = 2; d * d < n; d++) {
                if (sieve[d]) {
                    for (int j = d * d; j < n; j += d) {
                        sieve[j] = false;
                    }
                }
            }

            var ans = Integer.MAX_VALUE;
            var arr = new int[] { -1, -1 };
            var prev = 0;
            for (int i = left; i <= right; i++) {
                if (sieve[i]) {
                    if (prev != 0 && i - prev < ans) {
                        ans = i - prev;
                        arr[0] = prev;
                        arr[1] = i;
                    }
                    prev = i;
                }
            }
            return arr;
        }
    }

    static class Solution2 {
        public int[] closestPrimes(int left, int right) {
            int prevPrime = -1, closestA = -1, closestB = -1;
            int minDifference = (int) 1e6;

            for (int candidate = left; candidate <= right; candidate++) {
                if (isPrime(candidate)) {
                    if (prevPrime != -1) {
                        int difference = candidate - prevPrime;
                        if (difference < minDifference) {
                            minDifference = difference;
                            closestA = prevPrime;
                            closestB = candidate;
                        }
                        if (difference == 2 || difference == 1)
                            return new int[] {
                                    closestA,
                                    closestB
                            };
                    }
                    prevPrime = candidate;
                }
            }
            return new int[] { closestA, closestB };
        }

        boolean isPrime(int number) {
            if (number < 2)
                return false;
            if (number == 2 || number == 3)
                return true;
            if (number % 2 == 0)
                return false;
            for (int divisor = 3; divisor * divisor <= number; divisor += 2) {
                if (number % divisor == 0)
                    return false;
            }
            return true;
        }
    }
}
