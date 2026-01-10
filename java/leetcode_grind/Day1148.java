package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day1148 {
    // static class Solution1 {
    // Map<Pair<Integer, Integer>, Integer> savedResult = new HashMap<>();

    // public int minimumDeleteSum(String s1, String s2) {
    // return computeCost(s1, s2, s1.length() - 1, s2.length() - 1);
    // }

    // int computeCost(String s1, String s2, int i, int j) {
    // if (i < 0 && j < 0) {
    // return 0;
    // }

    // Pair<Integer, Integer> key = new Pair<>(i, j);
    // if (savedResult.containsKey(key)) {
    // return savedResult.get(key);
    // }

    // if (i < 0) {
    // savedResult.put(key, s2.charAt(j) + computeCost(s1, s2, i, j - 1));
    // return savedResult.get(key);
    // }

    // if (j < 0) {
    // savedResult.put(key, s1.charAt(i) + computeCost(s1, s2, i - 1, j));
    // return savedResult.get(key);
    // }

    // if (s1.charAt(i) == s2.charAt(j)) {
    // savedResult.put(key, computeCost(s1, s2, i - 1, j - 1));
    // } else {
    // savedResult.put(key, Math.min(s1.charAt(i) + computeCost(s1, s2, i - 1, j),
    // s2.charAt(j) + computeCost(s1, s2, i, j - 1)));
    // }

    // return savedResult.get(key);
    // }
    // }

    // https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/description/?envType=daily-question&envId=2026-01-10
    static class Solution1 {
        public int minimumDeleteSum(String s1, String s2) {
            int m = s1.length(), n = s2.length();
            int[][] computeCost = new int[m + 1][n + 1];

            for (int i = 1; i <= m; i++) {
                computeCost[i][0] = computeCost[i - 1][0] + s1.charAt(i - 1);
            }
            for (int j = 1; j <= n; j++) {
                computeCost[0][j] = computeCost[0][j - 1] + s2.charAt(j - 1);
            }

            for (int i = 1; i <= m; i++) {
                for (int j = 1; j <= n; j++) {
                    if (s1.charAt(i - 1) == s2.charAt(j - 1)) {
                        computeCost[i][j] = computeCost[i - 1][j - 1];
                    } else {
                        computeCost[i][j] = Math.min(s1.charAt(i - 1) + computeCost[i - 1][j],
                                s2.charAt(j - 1) + computeCost[i][j - 1]);
                    }
                }
            }

            return computeCost[m][n];
        }
    }
}
