package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day1109 {
    // https://leetcode.com/problems/count-number-of-trapezoids-i/description/?envType=daily-question&envId=2025-12-02
    static class Solution1 {
        public int countTrapezoids(int[][] points) {
            Map<Integer, Integer> pointNum = new HashMap<>();
            final int mod = 1_000_000_007;
            long ans = 0;
            long sum = 0;
            for (int[] point : points) {
                pointNum.put(point[1], pointNum.getOrDefault(point[1], 0) + 1);
            }
            for (int pNum : pointNum.values()) {
                long edge = ((long) pNum * (pNum - 1)) / 2;
                ans = (ans + edge * sum) % mod;
                sum = (sum + edge) % mod;
            }
            return (int) ans;
        }
    }
}
