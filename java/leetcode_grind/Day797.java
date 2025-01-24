package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.LinkedList;
import java.util.List;
import java.util.Queue;

public class Day797 {
    // https://leetcode.com/problems/find-eventual-safe-states/description/?envType=daily-question&envId=2025-01-24
    static class Solution1 {
        public List<Integer> eventualSafeNodes(int[][] graph) {
            int n = graph.length;
            int[] indegree = new int[n];
            List<List<Integer>> adj = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                adj.add(new ArrayList<>());
            }
            for (int i = 0; i < n; i++) {
                for (int node : graph[i]) {
                    adj.get(node).add(i);
                    indegree[i]++;
                }
            }

            Queue<Integer> q = new LinkedList<>();
            for (int i = 0; i < n; i++) {
                if (indegree[i] == 0) {
                    q.add(i);
                }
            }

            boolean[] safe = new boolean[n];
            while (!q.isEmpty()) {
                int node = q.poll();
                safe[node] = true;

                for (int neighbor : adj.get(node)) {
                    indegree[neighbor]--;
                    if (indegree[neighbor] == 0) {
                        q.add(neighbor);
                    }
                }
            }

            List<Integer> safeNodes = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                if (safe[i]) {
                    safeNodes.add(i);
                }
            }
            return safeNodes;
        }
    }

    static class Solution2 {
        boolean dfs(int node, int[][] adj, boolean[] visit, boolean[] inStack) {
            if (inStack[node]) {
                return true;
            }
            if (visit[node]) {
                return false;
            }
            visit[node] = true;
            inStack[node] = true;
            for (int neighbor : adj[node]) {
                if (dfs(neighbor, adj, visit, inStack)) {
                    return true;
                }
            }

            inStack[node] = false;
            return false;
        }

        public List<Integer> eventualSafeNodes(int[][] graph) {
            int n = graph.length;
            boolean[] visit = new boolean[n];
            boolean[] inStack = new boolean[n];
            for (int i = 0; i < n; i++) {
                dfs(i, graph, visit, inStack);
            }

            List<Integer> safeNodes = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                if (!inStack[i]) {
                    safeNodes.add(i);
                }
            }
            return safeNodes;
        }
    }

    // https://leetcode.com/problems/coin-change-ii/description/
    static class Solution3 {
        int[][] memo;
        int[] coins;

        int numberOfWays(int i, int amount) {
            if (amount == 0) {
                return 1;
            }
            if (i == coins.length) {
                return 0;
            }
            if (memo[i][amount] != -1) {
                return memo[i][amount];
            }
            if (coins[i] > amount) {
                return memo[i][amount] = numberOfWays(i + 1, amount);
            }
            memo[i][amount] = numberOfWays(i, amount - coins[i]) + numberOfWays(i + 1, amount);
            return memo[i][amount];
        }

        public int change(int amount, int[] coins) {
            this.coins = coins;
            memo = new int[coins.length][amount + 1];
            for (int[] row : memo) {
                Arrays.fill(row, -1);
            }
            return numberOfWays(0, amount);
        }
    }

    static class Solution4 {
        public int change(int amount, int[] coins) {
            int n = coins.length;
            long[][] dp = new long[n + 1][amount + 1];

            for (int i = 0; i <= n; i++) {
                dp[i][0] = 1;
            }

            for (int i = 1; i <= amount; i++) {
                dp[n][i] = 0;
            }

            for (int i = n - 1; i >= 0; i--) {
                for (int j = 1; j <= amount; j++) {
                    if (coins[i] > j) {
                        dp[i][j] = dp[i + 1][j];
                    } else {
                        dp[i][j] = dp[i + 1][j] + dp[i][j - coins[i]];
                    }
                }
            }

            return dp[0][amount] <= Integer.MAX_VALUE ? (int) dp[0][amount] : -1;
        }
    }

    static class Solution5 {
        public int change(int amount, int[] coins) {
            int n = coins.length;
            long[] dp = new long[amount + 1];
            dp[0] = 1;

            for (int i = n - 1; i >= 0; i--) {
                for (int j = coins[i]; j <= amount; j++) {
                    dp[j] += dp[j - coins[i]];
                }
            }
            return dp[amount] <= Integer.MAX_VALUE ? (int) dp[amount] : -1;
        }
    }

    // https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters/description/
    static class Solution6 {
        public int longestSubstring(String s, int k) {
            if (s == null || s.isEmpty() || k > s.length()) {
                return 0;
            }
            int[] countMap = new int[26];
            int n = s.length();
            int result = 0;
            for (int start = 0; start < n; start++) {
                Arrays.fill(countMap, 0);
                for (int end = start; end < n; end++) {
                    countMap[s.charAt(end) - 'a']++;
                    if (isValid(s, start, end, k, countMap)) {
                        result = Math.max(result, end - start + 1);
                    }
                }
            }
            return result;
        }

        boolean isValid(String s, int start, int end, int k, int[] countMap) {
            int countLetters = 0, countAtLeastK = 0;
            for (int freq : countMap) {
                if (freq > 0)
                    countLetters++;
                if (freq >= k)
                    countAtLeastK++;
            }
            return countAtLeastK == countLetters;
        }
    }

    static class Solution7 {
        public int longestSubstring(String s, int k) {
            return longestSubstringUtil(s, 0, s.length(), k);
        }

        int longestSubstringUtil(String s, int start, int end, int k) {
            if (end < k)
                return 0;
            int[] countMap = new int[26];
            for (int i = start; i < end; i++) {
                countMap[s.charAt(i) - 'a']++;
            }
            for (int mid = start; mid < end; mid++) {
                if (countMap[s.charAt(mid) - 'a'] >= k)
                    continue;
                int midNext = mid + 1;
                while (midNext < end && countMap[s.charAt(midNext) - 'a'] < k)
                    midNext++;
                return Math.max(longestSubstringUtil(s, start, mid, k), longestSubstringUtil(s, midNext, end, k));
            }
            return end - start;
        }
    }

    static class Solution8 {
        public int longestSubstring(String s, int k) {
            char[] str = s.toCharArray();
            int[] countMap = new int[26];
            int maxUnique = getMaxUniqueLetters(s);
            int result = 0;

            for (int currUnique = 1; currUnique <= maxUnique; currUnique++) {
                Arrays.fill(countMap, 0);
                int windowStart = 0, windowEnd = 0, idx = 0, unique = 0, countAtLeastK = 0;
                while (windowEnd < str.length) {
                    if (unique <= currUnique) {
                        idx = str[windowEnd] - 'a';
                        if (countMap[idx] == 0)
                            unique++;
                        countMap[idx]++;
                        if (countMap[idx] == k)
                            countAtLeastK++;
                        windowEnd++;
                    } else {
                        idx = str[windowStart] - 'a';
                        if (countMap[idx] == k)
                            countAtLeastK--;
                        countMap[idx]--;
                        if (countMap[idx] == 0)
                            unique--;
                        windowStart++;
                    }
                    if (unique == currUnique && unique == countAtLeastK) {
                        result = Math.max(windowEnd - windowStart, result);
                    }
                }
            }
            return result;
        }

        int getMaxUniqueLetters(String s) {
            boolean map[] = new boolean[26];
            int maxUnique = 0;
            for (int i = 0; i < s.length(); i++) {
                if (!map[s.charAt(i) - 'a']) {
                    maxUnique++;
                    map[s.charAt(i) - 'a'] = true;
                }
            }
            return maxUnique;
        }
    }
}
