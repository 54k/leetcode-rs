package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class Day836 {
    // https://leetcode.com/problems/check-if-number-is-a-sum-of-powers-of-three/description/
    static class Solution1 {
        public boolean checkPowersOfThree(int n) {
            return backtrack(0, n);
        }

        boolean backtrack(int pow, int n) {
            if (n == 0)
                return true;
            if (Math.pow(3, pow) > n)
                return false;

            boolean take = backtrack(pow + 1, n - (int) Math.pow(3, pow));
            boolean noTake = backtrack(pow + 1, n);
            return take || noTake;
        }
    }

    static class Solution2 {
        public boolean checkPowersOfThree(int n) {
            int power = 0;
            while (Math.pow(3, power) <= n) {
                power++;
            }
            while (n > 0) {
                if (n >= Math.pow(3, power)) {
                    n -= (int) Math.pow(3, power);
                }
                if (n >= Math.pow(3, power)) {
                    return false;
                }
                power--;
            }
            return true;
        }
    }

    static class Solution3 {
        public boolean checkPowersOfThree(int n) {
            while (n > 0) {
                if (n % 3 == 2) {
                    return false;
                }
                n /= 3;
            }
            return true;
        }
    }

    // https://leetcode.com/problems/sort-transformed-array/description/
    static class Solution4 {
        int transform(int x, int a, int b, int c) {
            return (a * x * x) + (b * x) + c;
        }

        public int[] sortTransformedArray(int[] nums, int a, int b, int c) {
            int[] answer = new int[nums.length];
            int left = 0, right = nums.length - 1;
            int index = 0;

            if (a < 0) {
                while (left <= right) {
                    int leftTransformedVal = transform(nums[left], a, b, c);
                    int rightTransformedVal = transform(nums[right], a, b, c);
                    if (leftTransformedVal < rightTransformedVal) {
                        answer[index++] = leftTransformedVal;
                        left++;
                    } else {
                        answer[index++] = rightTransformedVal;
                        right--;
                    }
                }
            } else {
                while (left <= right) {
                    int leftTransformedVal = transform(nums[left], a, b, c);
                    int rightTransformedVal = transform(nums[right], a, b, c);
                    if (leftTransformedVal > rightTransformedVal) {
                        answer[index++] = leftTransformedVal;
                        left++;
                    } else {
                        answer[index++] = rightTransformedVal;
                        right--;
                    }
                }

                for (int i = 0; i < answer.length / 2; i++) {
                    int temp = answer[i];
                    answer[i] = answer[answer.length - i - 1];
                    answer[answer.length - i - 1] = temp;
                }
            }
            return answer;
        }
    }

    static class Solution5 {
        public int[] sortTransformedArray(int[] nums, int a, int b, int c) {
            int n = nums.length;
            int[] answer = new int[n];

            for (int i = 0; i < n; i++) {
                int num = nums[i];
                answer[i] = (a * num * num) + (b * num) + c;
            }

            int maxElement = nums[0];
            for (int num : answer) {
                maxElement = Math.max(Math.abs(num), maxElement);
            }
            int maxDigits = 0;
            while (maxElement > 0) {
                maxDigits += 1;
                maxElement /= 10;
            }

            int placeValue = 1;
            for (int digit = 0; digit < maxDigits; ++digit) {
                sort(answer, placeValue);
                placeValue *= 10;
            }

            ArrayList<Integer> negatives = new ArrayList<>();
            ArrayList<Integer> positives = new ArrayList<>();

            for (int num : answer) {
                if (num < 0) {
                    negatives.add(num);
                } else {
                    positives.add(num);
                }
            }
            Collections.reverse(negatives);

            int index = 0;
            for (int num : negatives) {
                answer[index] = num;
                index++;
            }
            for (int num : positives) {
                answer[index] = num;
                index++;
            }
            return answer;
        }

        void sort(int[] array, int placeValue) {
            ArrayList<List<Integer>> mapDigits = new ArrayList<>(10);
            for (int digit = 0; digit < 10; ++digit) {
                mapDigits.add(digit, new ArrayList<>());
            }

            for (int num : array) {
                int digit = Math.abs(num) / placeValue;
                digit = digit % 10;
                mapDigits.get(digit).add(num);
            }

            int index = 0;
            for (int digit = 0; digit < 10; ++digit) {
                for (int num : mapDigits.get(digit)) {
                    array[index] = num;
                    index++;
                }
            }
        }
    }
}