package leetcode_grind;

public class Day829 {
    // https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum/description/
    static class Solution1 {
        public int numOfSubarrays(int[] arr) {
            int MOD = (int) Math.pow(10, 9) + 7;
            int n = arr.length;
            for (int num = 0; num < n; num++) {
                arr[num] %= 2;
            }

            int[] dpEven = new int[n], dpOdd = new int[n];

            if (arr[n - 1] == 0)
                dpEven[n - 1] = 1;
            else
                dpOdd[n - 1] = 1;

            for (int num = n - 2; num >= 0; num--) {
                if (arr[num] == 1) {
                    dpOdd[num] = (1 + dpEven[num + 1]) % MOD;
                    dpEven[num] = dpOdd[num + 1];
                } else {
                    dpEven[num] = (1 + dpEven[num + 1]) % MOD;
                    dpOdd[num] = dpOdd[num + 1];
                }
            }

            int count = 0;
            for (int oddCount : dpOdd) {
                count += oddCount;
                count %= MOD;
            }
            return count;
        }
    }

    static class Solution2 {
        public int numOfSubarrays(int[] arr) {
            final int MOD = 1_000_000_007;
            int count = 0, prefixSum = 0;
            int oddCount = 0, evenCount = 1;
            for (int num : arr) {
                prefixSum += num;
                if (prefixSum % 2 == 0) {
                    count += oddCount;
                    evenCount++;
                } else {
                    count += evenCount;
                    oddCount++;
                }
                count %= MOD;
            }
            return count;
        }
    }
}
