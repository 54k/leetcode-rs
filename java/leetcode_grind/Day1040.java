package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day1040 {
    // https://leetcode.com/problems/fraction-to-recurring-decimal/description/?envType=daily-question&envId=2025-09-24
    static class Solution1 {
        public String fractionToDecimal(int numerator, int denominator) {
            if (numerator == 0) {
                return "0";
            }

            StringBuilder fraction = new StringBuilder();
            if ((numerator < 0) ^ (denominator < 0)) {
                fraction.append("-");
            }

            long dividend = Math.abs(Long.valueOf(numerator));
            long divisor = Math.abs(Long.valueOf(denominator));

            fraction.append(String.valueOf(dividend / divisor));
            long remainder = dividend % divisor;
            if (remainder == 0) {
                return fraction.toString();
            }
            fraction.append(".");

            Map<Long, Integer> map = new HashMap<>();
            while (remainder != 0) {
                if (map.containsKey(remainder)) {
                    fraction.insert(map.get(remainder), "(");
                    fraction.append(")");
                    break;
                }
                map.put(remainder, fraction.length());
                remainder *= 10;
                fraction.append(String.valueOf(remainder / divisor));
                remainder %= divisor;
            }
            return fraction.toString();
        }
    }

    static class Solution2 {
        int[] numsArray;

        public int maxSubArray(int[] nums) {
            numsArray = nums;
            return findBestSubarray(0, numsArray.length - 1);
        }

        int findBestSubarray(int left, int right) {
            if (left > right) {
                return Integer.MIN_VALUE;
            }

            int mid = Math.floorDiv(left + right, 2);
            int curr = 0;
            int bestLeftSum = 0;
            int bestRightSum = 0;

            for (int i = mid - 1; i >= left; i--) {
                curr += numsArray[i];
                bestLeftSum = Math.max(bestLeftSum, curr);
            }

            curr = 0;
            for (int i = mid + 1; i <= right; i++) {
                curr += numsArray[i];
                bestRightSum = Math.max(bestRightSum, curr);
            }

            int bestCombinedSum = numsArray[mid] + bestLeftSum + bestRightSum;

            int leftHalf = findBestSubarray(left, mid - 1);
            int rightHalf = findBestSubarray(mid + 1, right);

            return Math.max(bestCombinedSum, Math.max(leftHalf, rightHalf));
        }
    }
}
