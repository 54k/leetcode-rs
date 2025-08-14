package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeMap;

public class Day999 {
    // https://leetcode.com/problems/largest-3-same-digit-number-in-string/description/?envType=daily-question&envId=2025-08-14
    static class Solution1 {
        public String largestGoodInteger(String num) {
            // Assign 'maxDigit' to the NUL character (smallest ASCII value character)
            char maxDigit = '\0';

            // Iterate on characters of the num string.
            for (int index = 0; index <= num.length() - 3; ++index) {
                // If 3 consecutive characters are the same,
                // store the character in 'maxDigit' if it's bigger than what it already stores.
                if (num.charAt(index) == num.charAt(index + 1) && num.charAt(index) == num.charAt(index + 2)) {
                    maxDigit = (char) Math.max(maxDigit, num.charAt(index));
                }
            }

            // If 'maxDigit' is NUL, return an empty string; otherwise, return a string of
            // size 3 with 'maxDigit' characters.
            return maxDigit == '\0' ? "" : new String(new char[] { maxDigit, maxDigit, maxDigit });
        }
    }

    static class TimeMap1 {
        HashMap<String, TreeMap<Integer, String>> keyTimeMap;

        public TimeMap1() {
            keyTimeMap = new HashMap<>();
        }

        public void set(String key, String value, int timestamp) {
            if (!keyTimeMap.containsKey(key)) {
                keyTimeMap.put(key, new TreeMap<>());
            }
            keyTimeMap.get(key).put(timestamp, value);
        }

        public String get(String key, int timestamp) {
            if (!keyTimeMap.containsKey(key)) {
                return "";
            }

            Integer floorKey = keyTimeMap.get(key).floorKey(timestamp);
            if (floorKey != null) {
                return keyTimeMap.get(key).get(floorKey);
            }
            return "";
        }
    }

    static class TimeMap2 {
        HashMap<String, ArrayList<Pair<Integer, String>>> keyTimeMap;

        public TimeMap2() {
            keyTimeMap = new HashMap<>();
        }

        public void set(String key, String value, int timestamp) {
            if (!keyTimeMap.containsKey(key)) {
                keyTimeMap.put(key, new ArrayList<>());
            }
            keyTimeMap.get(key).add(new Pair<>(timestamp, value));
        }

        public String get(String key, int timestamp) {
            if (!keyTimeMap.containsKey(key)) {
                return "";
            }
            if (timestamp < keyTimeMap.get(key).get(0).getKey()) {
                return "";
            }

            int left = 0;
            int right = keyTimeMap.get(key).size();
            while (left < right) {
                int mid = (left + right) / 2;
                if (keyTimeMap.get(key).get(mid).getKey() <= timestamp) {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }

            if (right == 0) {
                return "";
            }

            return keyTimeMap.get(key).get(right - 1).getValue();
        }
    }

    private static class Pair<T0, T1> {
        T0 key;
        T1 value;

        public Pair(T0 key, T1 value) {
            this.key = key;
            this.value = value;
        }

        T0 getKey() {
            return key;
        }

        T1 getValue() {
            return value;
        }
    }

    // https://leetcode.com/problems/maximum-score-from-performing-multiplication-operations/description/
    static class Solution2 {

        int dp(Integer[][] memo, int[] nums, int[] multipliers, int op, int left) {
            int n = nums.length;
            if (op == multipliers.length) {
                return 0;
            }

            if (memo[op][left] != null) {
                return memo[op][left];
            }

            int l = nums[left] * multipliers[op] + dp(memo, nums, multipliers, op + 1, left + 1);
            int r = nums[(n - 1) - (op - left)] * multipliers[op] + dp(memo, nums, multipliers, op + 1, left);

            return memo[op][left] = Math.max(l, r);
        }

        public int maximumScore(int[] nums, int[] multipliers) {
            Integer[][] memo = new Integer[multipliers.length + 1][multipliers.length + 1];
            return dp(memo, nums, multipliers, 0, 0);
        }
    }

    static class Solution3 {
        public int maximumScore(int[] nums, int[] multipliers) {
            int n = nums.length;
            int m = multipliers.length;
            int[][] dp = new int[m + 1][n + 1];
            for (int i = 0; i <= m; i++) {
                Arrays.fill(dp[i], 0);
            }
            for (int op = m - 1; op >= 0; op--) {
                for (int left = op; left >= 0; left--) {
                    dp[op][left] = Math.max(multipliers[op] * nums[left] + dp[op + 1][left + 1],
                            multipliers[op] * nums[n - 1 - (op - left)] + dp[op + 1][left]);

                }
            }
            return dp[0][0];
        }
    }

    static class Solution4 {
        public int maximumScore(int[] nums, int[] multipliers) {
            int n = nums.length;
            int m = multipliers.length;
            int[] dp = new int[m + 1];

            for (int op = m - 1; op >= 0; op--) {
                for (int left = 0; left <= op; left++) {
                    dp[left] = Math.max(multipliers[op] * nums[left] + dp[left + 1],
                            multipliers[op] * nums[n - 1 - (op - left)] + dp[left]);
                }
            }
            return dp[0];
        }
    }

    // https://leetcode.com/problems/evaluate-division/description/
    static class Solution5 {
        public double[] calcEquation(List<List<String>> equations, double[] values, List<List<String>> queries) {
            Map<String, Map<String, Double>> graph = new HashMap<>();
            for (int i = 0; i < equations.size(); i++) {
                List<String> equation = equations.get(i);
                String dividend = equation.get(0), divisor = equation.get(1);
                double quotient = values[i];

                if (!graph.containsKey(dividend)) {
                    graph.put(dividend, new HashMap<String, Double>());
                }
                if (!graph.containsKey(divisor)) {
                    graph.put(divisor, new HashMap<String, Double>());
                }

                graph.get(dividend).put(divisor, quotient);
                graph.get(divisor).put(dividend, 1. / quotient);
            }

            double[] results = new double[queries.size()];
            for (int i = 0; i < queries.size(); i++) {
                List<String> query = queries.get(i);
                String dividend = query.get(0), divisor = query.get(1);

                if (!graph.containsKey(dividend) || !graph.containsKey(divisor)) {
                    results[i] = -1.0;
                } else if (dividend == divisor) {
                    results[i] = 1.0;
                } else {
                    Set<String> visited = new HashSet<>();
                    results[i] = backtrackEvaluate(graph, dividend, divisor, 1, visited);
                }
            }
            return results;
        }

        double backtrackEvaluate(Map<String, Map<String, Double>> graph, String currNode, String targetNode,
                double accProduct, Set<String> visited) {
            visited.add(currNode);
            double ret = -1.0;
            Map<String, Double> neighbors = graph.get(currNode);
            if (neighbors.containsKey(targetNode)) {
                ret = accProduct * neighbors.get(targetNode);
            } else {
                for (Map.Entry<String, Double> pair : neighbors.entrySet()) {
                    String nextNode = pair.getKey();
                    if (visited.contains(nextNode)) {
                        continue;
                    }
                    ret = backtrackEvaluate(graph, nextNode, targetNode, accProduct * pair.getValue(), visited);
                    if (ret != -1.0) {
                        break;
                    }
                }
            }
            visited.remove(currNode);
            return ret;
        }
    }
}
