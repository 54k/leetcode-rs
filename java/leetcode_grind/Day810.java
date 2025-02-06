package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day810 {
    // https://leetcode.com/problems/tuple-with-same-product/description/
    static class Solution1 {
        public int tupleSameProduct(int[] nums) {
            int numsLength = nums.length;
            Map<Integer, Integer> pairProductsFrequency = new HashMap<>();
            int totalNumberOfTuples = 0;
            for (int firstIndex = 0; firstIndex < numsLength; firstIndex++) {
                for (int secondIndex = firstIndex + 1; secondIndex < numsLength; secondIndex++) {
                    pairProductsFrequency.put(
                            nums[firstIndex] * nums[secondIndex],
                            pairProductsFrequency.getOrDefault(
                                    nums[firstIndex] * nums[secondIndex],
                                    0) + 1);
                }
            }

            for (int productValue : pairProductsFrequency.keySet()) {
                int productFrequency = pairProductsFrequency.get(productValue);
                int pairOfEqualProduct = ((productFrequency - 1) * productFrequency) / 2;
                totalNumberOfTuples += 8 * pairOfEqualProduct;
            }
            return totalNumberOfTuples;
        }
    }

    // https://leetcode.com/problems/fraction-to-recurring-decimal/description/
    static class Solution2 {
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

}
