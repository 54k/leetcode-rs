package leetcode_grind;

public class Day335 {
    // https://leetcode.com/problems/min-cost-climbing-stairs/description
    static class Solution1 {
        public int minCostClimbingStairs(int[] cost) {
            var a = cost[0];
            var b = cost[1];
            for (var i = 2; i < cost.length; i++) {
                var c = Math.min(a + cost[i], b + cost[i]);
                a = b;
                b = c;
            }
            return Math.min(a, b);
        }
    }
}
