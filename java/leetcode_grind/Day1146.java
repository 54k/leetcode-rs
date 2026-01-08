package leetcode_grind;

import java.util.LinkedList;
import java.util.Queue;

public class Day1146 {
    // https://leetcode.com/problems/max-dot-product-of-two-subsequences/description/?envType=daily-question&envId=2026-01-08
    static class Solution1 {
        int[][] memo;

        int dp(int i, int j, int[] nums1, int[] nums2) {
            if (i == nums1.length || j == nums2.length) {
                return 0;
            }

            if (memo[i][j] != 0) {
                return memo[i][j];
            }

            int use = nums1[i] * nums2[j] + dp(i + 1, j + 1, nums1, nums2);
            memo[i][j] = Math.max(use, Math.max(dp(i + 1, j, nums1, nums2), dp(i, j + 1, nums1, nums2)));
            return memo[i][j];
        }

        public int maxDotProduct(int[] nums1, int[] nums2) {
            int firstMax = Integer.MIN_VALUE;
            int secondMax = Integer.MIN_VALUE;
            int firstMin = Integer.MAX_VALUE;
            int secondMin = Integer.MAX_VALUE;

            for (int num : nums1) {
                firstMax = Math.max(firstMax, num);
                firstMin = Math.min(firstMin, num);
            }

            for (int num : nums2) {
                secondMax = Math.max(secondMax, num);
                secondMin = Math.min(secondMin, num);
            }

            if (firstMax < 0 && secondMin > 0) {
                return firstMax * secondMin;
            }

            if (firstMin > 0 && secondMax < 0) {
                return firstMin * secondMax;
            }

            memo = new int[nums1.length][nums2.length];
            return dp(0, 0, nums1, nums2);
        }
    }

    static class Solution2 {
        public int maxDotProduct(int[] nums1, int[] nums2) {
            int firstMax = Integer.MIN_VALUE;
            int secondMax = Integer.MIN_VALUE;
            int firstMin = Integer.MAX_VALUE;
            int secondMin = Integer.MAX_VALUE;

            for (int num : nums1) {
                firstMax = Math.max(firstMax, num);
                firstMin = Math.min(firstMin, num);
            }

            for (int num : nums2) {
                secondMax = Math.max(secondMax, num);
                secondMin = Math.min(secondMin, num);
            }

            if (firstMax < 0 && secondMin > 0) {
                return firstMax * secondMin;
            }

            if (firstMin > 0 && secondMax < 0) {
                return firstMin * secondMax;
            }

            int[][] dp = new int[nums1.length + 1][nums2.length + 1];
            for (int i = nums1.length - 1; i >= 0; i--) {
                for (int j = nums2.length - 1; j >= 0; j--) {
                    int use = nums1[i] * nums2[j] + dp[i + 1][j + 1];
                    dp[i][j] = Math.max(use, Math.max(dp[i + 1][j], dp[i][j + 1]));
                }
            }
            return dp[0][0];
        }
    }

    static class Solution3 {
        public int maxDotProduct(int[] nums1, int[] nums2) {
            int firstMax = Integer.MIN_VALUE;
            int secondMax = Integer.MIN_VALUE;
            int firstMin = Integer.MAX_VALUE;
            int secondMin = Integer.MAX_VALUE;

            for (int num : nums1) {
                firstMax = Math.max(firstMax, num);
                firstMin = Math.min(firstMin, num);
            }

            for (int num : nums2) {
                secondMax = Math.max(secondMax, num);
                secondMin = Math.min(secondMin, num);
            }

            if (firstMax < 0 && secondMin > 0) {
                return firstMax * secondMin;
            }

            if (firstMin > 0 && secondMax < 0) {
                return firstMin * secondMax;
            }

            int m = nums2.length + 1;
            int[] dp = new int[m];
            int[] prevDp = new int[m];

            for (int i = nums1.length - 1; i >= 0; i--) {
                dp = new int[m];
                for (int j = nums2.length - 1; j >= 0; j--) {
                    int use = nums1[i] * nums2[j] + prevDp[j + 1];
                    dp[j] = Math.max(use, Math.max(prevDp[j], dp[j + 1]));
                }
                prevDp = dp;
            }

            return dp[0];
        }
    }

    // https://leetcode.com/problems/the-maze/description/?envType=weekly-question&envId=2026-01-08
    static class Solution4 {
        boolean dfs(int m, int n, int[][] maze, int[] curr, int[] destination, boolean[][] visit) {
            if (visit[curr[0]][curr[1]]) {
                return false;
            }
            if (curr[0] == destination[0] && curr[1] == destination[1]) {
                return true;
            }

            visit[curr[0]][curr[1]] = true;
            int[] dirX = { 0, 1, 0, -1 };
            int[] dirY = { -1, 0, 1, 0 };

            for (int i = 0; i < 4; i++) {
                int r = curr[0], c = curr[1];
                while (r >= 0 && r < m && c >= 0 && c < n && maze[r][c] == 0) {
                    r += dirX[i];
                    c += dirY[i];
                }
                if (dfs(m, n, maze, new int[] { r - dirX[i], c - dirY[i] }, destination, visit)) {
                    return true;
                }
            }
            return false;
        }

        public boolean hasPath(int[][] maze, int[] start, int[] destination) {
            int m = maze.length;
            int n = maze[0].length;
            boolean[][] visit = new boolean[m][n];
            return dfs(m, n, maze, start, destination, visit);
        }
    }

    static class Solution5 {
        public boolean hasPath(int[][] maze, int[] start, int[] destination) {
            int m = maze.length;
            int n = maze[0].length;
            boolean[][] visit = new boolean[m][n];
            int[] dirX = { 0, 1, 0, -1 };
            int[] dirY = { -1, 0, 1, 0 };

            Queue<int[]> queue = new LinkedList<>();
            queue.offer(start);
            visit[start[0]][start[1]] = true;

            while (!queue.isEmpty()) {
                int[] curr = queue.poll();
                if (curr[0] == destination[0] && curr[1] == destination[1]) {
                    return true;
                }

                for (int i = 0; i < 4; i++) {
                    int r = curr[0];
                    int c = curr[1];

                    while (r >= 0 && r < m && c >= 0 && c < n && maze[r][c] == 0) {
                        r += dirX[i];
                        c += dirY[i];
                    }

                    r -= dirX[i];
                    c -= dirY[i];

                    if (!visit[r][c]) {
                        queue.offer(new int[] { r, c });
                        visit[r][c] = true;
                    }
                }
            }
            return false;
        }
    }
}
