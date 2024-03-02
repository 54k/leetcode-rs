package leetcode_grind;

public class Day475 {
    // https://leetcode.com/problems/odd-even-jump/
    static class Solution1 {
        public int oddEvenJumps(int[] arr) {
            var n = arr.length;

            var dp = new int[n][2];
            dp[n - 1] = new int[] { 1, 1 };
            var map = new TreeMap<Integer, Integer>();
            for (int i = n - 1; i >= 0; i--) {
                if (!map.isEmpty()) {
                    if (map.ceilingEntry(arr[i]) != null) {
                        dp[i][1] += dp[map.ceilingEntry(arr[i]).getValue()][0];
                    }
                    if (map.floorEntry(arr[i]) != null)
                        dp[i][0] += dp[map.floorEntry(arr[i]).getValue()][1];
                }
                map.put(arr[i], i);
            }

            var ans = 0;
            for (int i = 0; i < n; i++) {
                ans += dp[i][1];
            }
            return ans;
        }
    }
}
