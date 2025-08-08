package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day993 {
    // https://leetcode.com/problems/soup-servings/description/?envType=daily-question&envId=2025-08-08
    static class Solution1 {
        public double soupServings(int n) {
            int m = (int) Math.ceil(n / 25.0);
            Map<Integer, Map<Integer, Double>> dp = new HashMap<>();
            dp.put(0, new HashMap<>());
            dp.get(0).put(0, 0.5);

            for (int k = 1; k <= m; k++) {
                dp.put(k, new HashMap<>());
                dp.get(0).put(k, 1.0);
                dp.get(k).put(0, 0.0);
                for (int j = 1; j <= k; j++) {
                    dp.get(j).put(k, calculateDP(j, k, dp));
                    dp.get(k).put(j, calculateDP(k, j, dp));
                }

                if (dp.get(k).get(k) > 1 - 1e-5) {
                    return 1;
                }
            }

            return dp.get(m).get(m);
        }

        private double calculateDP(int i, int j, Map<Integer, Map<Integer, Double>> dp) {
            return (dp.get(Math.max(0, i - 4)).get(j)
                    + dp.get(Math.max(0, i - 3)).get(j - 1)
                    + dp.get(Math.max(0, i - 2)).get(Math.max(0, j - 2))
                    + dp.get(i - 1).get(Math.max(0, j - 3))) / 4;
        }
    }

    static class Solution2 {
        public double soupServings(int n) {
            int m = (int) Math.ceil(n / 25.0);
            Map<Integer, Map<Integer, Double>> dp = new HashMap<>();

            for (int k = 1; k <= m; k++) {
                if (calculateDP(k, k, dp) > 1 - 1e-5) {
                    return 1.0;
                }
            }
            return calculateDP(m, m, dp);
        }

        private double calculateDP(int i, int j, Map<Integer, Map<Integer, Double>> dp) {
            if (i <= 0 && j <= 0) {
                return 0.5;
            }
            if (i <= 0) {
                return 1.0;
            }
            if (j <= 0) {
                return 0.0;
            }
            if (dp.containsKey(i) && dp.get(i).containsKey(j)) {
                return dp.get(i).get(j);
            }
            double result = (calculateDP(i - 4, j, dp) + calculateDP(i - 3, j - 1, dp) +
                    calculateDP(i - 2, j - 2, dp) + calculateDP(i - 1, j - 3, dp)) / 4.0;
            dp.computeIfAbsent(i, k -> new HashMap<>()).put(j, result);
            return result;
        }
    }
}
