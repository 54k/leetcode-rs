package leetcode_grind;

public class Day816 {
    // https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits/description/?envType=daily-question&envId=2025-02-12
    // static class Solution1 {
    //     public int maximumSum(int[] nums) {
    //         Pair<Integer, Integer>[] digitSumPairs = new Pair[nums.length];
    //         for (int i = 0; i < nums.length; i++) {
    //             int digitSum = dsum(nums[i]);
    //             digitSumPairs[i] = new Pair<>(digitSum, nums[i]);
    //         }

    //         Arrays.sort(digitSumPairs, Comparator.comparing(Pair<Integer, Integer>::getKey).thenComparing(
    //                 Pair<Integer, Integer>::getValue));

    //         int maxPairSum = -1;

    //         for (int index = 1; index < digitSumPairs.length; index++) {
    //             int currentDigitSum = digitSumPairs[index].getKey();
    //             int previousDigitSum = digitSumPairs[index - 1].getKey();

    //             if (currentDigitSum == previousDigitSum) {
    //                 int currentSum = digitSumPairs[index].getValue() + digitSumPairs[index - 1].getValue();
    //                 maxPairSum = Math.max(maxPairSum, currentSum);
    //             }
    //         }
    //         return maxPairSum;
    //     }

    //     int dsum(int num) {
    //         var res = 0;
    //         while (num != 0) {
    //             res += num % 10;
    //             num /= 10;
    //         }
    //         return res;
    //     }
    // }

}
