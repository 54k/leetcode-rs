package leetcode_grind;

import java.util.*;

public class Day725 {
    // https://leetcode.com/problems/prime-subtraction-operation/description/?envType=daily-question&envId=2024-11-11
    static class Solution1 {
        boolean checkPrime(int x) {
            for (int i = 2; i <= Math.sqrt(x); i++) {
                if (x % i == 0) {
                    return false;
                }
            }
            return true;
        }

        public boolean primeSubOperation(int[] nums) {
            for (int i = 0; i < nums.length; i++) {
                int bound;
                if (i == 0) {
                    bound = nums[0];
                } else {
                    bound = nums[i] - nums[i - 1];
                }

                if (bound <= 0) {
                    return false;
                }

                int largestPrime = 0;
                for (int j = bound - 1; j >= 2; j--) {
                    if (checkPrime(j)) {
                        largestPrime = j;
                        break;
                    }
                }

                nums[i] = nums[i] - largestPrime;
            }
            return true;
        }
    }

    static class Solution2 {

        boolean isPrime(int n) {
            for (int i = 2; i * i <= n; i++) {
                if (n % i == 0) {
                    return false;
                }
            }
            return true;
        }

        public boolean primeSubOperation(int[] nums) {
            int maxElement = Integer.MIN_VALUE;
            for (int num : nums) {
                maxElement = Math.max(maxElement, num);
            }

            int[] previousPrime = new int[maxElement + 1];

            for (int i = 2; i <= maxElement; i++) {
                if (isPrime(i)) {
                    previousPrime[i] = i;
                } else {
                    previousPrime[i] = previousPrime[i - 1];
                }
            }

            for (int i = 0; i < nums.length; i++) {
                int bound;
                if (i == 0) {
                    bound = nums[0];
                } else {
                    bound = nums[i] - nums[i - 1];
                }

                if (bound <= 0) {
                    return false;
                }

                int largestPrime = previousPrime[bound - 1];
                nums[i] -= largestPrime;
            }
            return true;
        }
    }

    static class Solution3 {
        public boolean primeSubOperation(int[] nums) {
            int maxElement = getMaxElement(nums);

            boolean[] sieve = new boolean[maxElement + 1];
            fill(sieve, true);
            sieve[1] = false;

            for (int i = 2; i <= Math.sqrt(maxElement + 1); i++) {
                if (sieve[i]) {
                    for (int j = i * i; j <= maxElement; j += i) {
                        sieve[j] = false;
                    }
                }
            }

            int currValue = 1;
            int i = 0;
            while (i < nums.length) {
                int difference = nums[i] - currValue;
                if (difference < 0) {
                    return false;
                }

                if (sieve[difference] == true || difference == 0) {
                    i++;
                    currValue++;
                } else {
                    currValue++;
                }
            }

            return true;
        }

        int getMaxElement(int[] nums) {
            int max = -1;
            for (int num : nums) {
                if (num > max) {
                    max = num;
                }
            }
            return max;
        }

        void fill(boolean[] arr, boolean value) {
            for (int i = 0; i < arr.length; i++) {
                arr[i] = value;
            }
        }
    }
}
