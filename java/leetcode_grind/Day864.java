package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Queue;

public class Day864 {
    // https://leetcode.com/problems/solving-questions-with-brainpower/description/
    static class Solution1 {
        public long mostPoints(int[][] questions) {
            int n = questions.length;
            long[] dp = new long[n];
            dp[n - 1] = questions[n - 1][0];

            for (int i = n - 2; i >= 0; i--) {
                dp[i] = questions[i][0];
                int skip = questions[i][1];
                if (i + skip + 1 < n) {
                    dp[i] += dp[i + skip + 1];
                }

                dp[i] = Math.max(dp[i], dp[i + 1]);
            }
            return dp[0];
        }
    }

    static class Solution2 {
        long dp[];

        long dfs(int[][] questions, int i) {
            if (i >= questions.length) {
                return 0;
            }
            if (dp[i] != 0) {
                return dp[i];
            }
            long points = questions[i][0];
            int skip = questions[i][1];
            dp[i] = Math.max(points + dfs(questions, i + skip + 1), dfs(questions, i + 1));
            return dp[i];
        }

        public long mostPoints(int[][] questions) {
            int n = questions.length;
            dp = new long[n];
            return dfs(questions, 0);
        }
    }

    // https://leetcode.com/problems/shortest-distance-from-all-buildings/description/
    static class Solution3 {

        int bfs(int[][] grid, int row, int col, int totalHouses) {
            int[][] dirs = { { 1, 0 }, { -1, 0 }, { 0, 1 }, { 0, -1 } };
            int rows = grid.length;
            int cols = grid[0].length;
            int distanceSum = 0;
            int housesReached = 0;

            Queue<int[]> q = new LinkedList<>();
            q.offer(new int[] { row, col });

            boolean[][] vis = new boolean[rows][cols];
            vis[row][col] = true;

            int steps = 0;
            while (!q.isEmpty() && housesReached != totalHouses) {
                for (int i = q.size(); i > 0; --i) {
                    int[] curr = q.poll();
                    row = curr[0];
                    col = curr[1];

                    if (grid[row][col] == 1) {
                        distanceSum += steps;
                        housesReached++;
                        continue;
                    }

                    for (int[] dir : dirs) {
                        int nextRow = row + dir[0];
                        int nextCol = col + dir[1];

                        if (nextRow >= 0 && nextCol >= 0 && nextRow < rows && nextCol < cols) {
                            if (!vis[nextRow][nextCol] && grid[nextRow][nextCol] != 2) {
                                vis[nextRow][nextCol] = true;
                                q.offer(new int[] { nextRow, nextCol });
                            }
                        }
                    }
                }
                steps++;
            }

            if (housesReached != totalHouses) {
                for (row = 0; row < rows; row++) {
                    for (col = 0; col < cols; col++) {
                        if (grid[row][col] == 0 && vis[row][col]) {
                            grid[row][col] = 2;
                        }
                    }
                }
                return Integer.MAX_VALUE;
            }

            return distanceSum;
        }

        public int shortestDistance(int[][] grid) {
            int minDistance = Integer.MAX_VALUE;
            int rows = grid.length;
            int cols = grid[0].length;

            int totalHouses = 0;

            for (int row = 0; row < rows; ++row) {
                for (int col = 0; col < cols; ++col) {
                    if (grid[row][col] == 1) {
                        totalHouses++;
                    }
                }
            }

            for (int row = 0; row < rows; row++) {
                for (int col = 0; col < cols; ++col) {
                    if (grid[row][col] == 0) {
                        minDistance = Math.min(minDistance, bfs(grid, row, col, totalHouses));
                    }
                }
            }

            if (minDistance == Integer.MAX_VALUE) {
                return -1;
            }

            return minDistance;
        }
    }

    static class Solution4 {
        public int shortestDistance(int[][] grid) {
            int[][] dirs = { { 1, 0 }, { -1, 0 }, { 0, 1 }, { 0, -1 } };
            int rows = grid.length;
            int cols = grid[0].length;

            int[][] total = new int[rows][cols];
            int emptyLandValue = 0;
            int minDist = Integer.MAX_VALUE;

            for (int row = 0; row < rows; ++row) {
                for (int col = 0; col < cols; ++col) {
                    if (grid[row][col] == 1) {
                        minDist = Integer.MAX_VALUE;

                        Queue<int[]> q = new LinkedList<>();
                        q.offer(new int[] { row, col });
                        int steps = 0;

                        while (!q.isEmpty()) {
                            steps++;

                            for (int level = q.size(); level > 0; --level) {
                                int[] curr = q.poll();

                                for (int[] dir : dirs) {
                                    int nextRow = curr[0] + dir[0];
                                    int nextCol = curr[1] + dir[1];

                                    if (nextRow >= 0 && nextRow < rows && nextCol >= 0 && nextCol < cols &&
                                            grid[nextRow][nextCol] == emptyLandValue) {
                                        grid[nextRow][nextCol]--;
                                        total[nextRow][nextCol] += steps;
                                        q.offer(new int[] { nextRow, nextCol });
                                        minDist = Math.min(minDist, total[nextRow][nextCol]);
                                    }
                                }
                            }
                        }
                        emptyLandValue--;
                    }
                }
            }

            return minDist == Integer.MAX_VALUE ? -1 : minDist;
        }
    }

    static class Solution5 {
        void bfs(int[][] grid, int[][][] distances, int row, int col) {
            int[][] dirs = { { 1, 0 }, { -1, 0 }, { 0, 1 }, { 0, -1 } };
            int rows = grid.length;
            int cols = grid[0].length;

            Queue<int[]> q = new LinkedList<>();
            q.offer(new int[] { row, col });

            boolean[][] vis = new boolean[rows][cols];
            vis[row][col] = true;

            int steps = 0;

            while (!q.isEmpty()) {
                for (int i = q.size(); i > 0; --i) {
                    int[] curr = q.poll();
                    row = curr[0];
                    col = curr[1];

                    if (grid[row][col] == 0) {
                        distances[row][col][0] += steps;
                        distances[row][col][1] += 1;
                    }

                    for (int[] dir : dirs) {
                        int nextRow = row + dir[0];
                        int nextCol = col + dir[1];

                        if (nextRow >= 0 && nextCol >= 0 && nextRow < rows && nextCol < cols) {
                            if (!vis[nextRow][nextCol] && grid[nextRow][nextCol] == 0) {
                                vis[nextRow][nextCol] = true;
                                q.offer(new int[] { nextRow, nextCol });
                            }
                        }
                    }
                }

                steps++;
            }
        }

        public int shortestDistance(int[][] grid) {
            int minDistance = Integer.MAX_VALUE;
            int rows = grid.length;
            int cols = grid[0].length;
            int totalHouses = 0;

            int[][][] distances = new int[rows][cols][2];

            for (int row = 0; row < rows; ++row) {
                for (int col = 0; col < cols; ++col) {
                    if (grid[row][col] == 1) {
                        totalHouses++;
                        bfs(grid, distances, row, col);
                    }
                }
            }

            for (int row = 0; row < rows; ++row) {
                for (int col = 0; col < cols; ++col) {
                    if (distances[row][col][1] == totalHouses) {
                        minDistance = Math.min(minDistance, distances[row][col][0]);
                    }
                }
            }

            if (minDistance == Integer.MAX_VALUE) {
                return -1;
            }
            return minDistance;
        }
    }

    // https://leetcode.com/problems/minimum-window-substring/description/
    static class Solution6 {
        public String minWindow(String s, String t) {
            if (s.length() == 0 || t.length() == 0) {
                return "";
            }

            Map<Character, Integer> dictT = new HashMap<>();
            for (int i = 0; i < t.length(); i++) {
                int count = dictT.getOrDefault(t.charAt(i), 0);
                dictT.put(t.charAt(i), count + 1);
            }

            int required = dictT.size();

            List<Pair<Integer, Character>> filteredS = new ArrayList<>();
            for (int i = 0; i < s.length(); i++) {
                char c = s.charAt(i);
                if (dictT.containsKey(c)) {
                    filteredS.add(new Pair<>(i, c));
                }
            }

            int l = 0, r = 0, formed = 0;
            Map<Character, Integer> windowCounts = new HashMap<>();
            int[] ans = { -1, 0, 0 };

            while (r < filteredS.size()) {
                char c = filteredS.get(r).getValue();
                int count = windowCounts.getOrDefault(c, 0);
                windowCounts.put(c, count + 1);

                if (dictT.containsKey(c) && windowCounts.get(c).intValue() == dictT.get(c).intValue()) {
                    formed++;
                }

                while (l <= r && formed == required) {
                    c = filteredS.get(l).getValue();

                    int end = filteredS.get(r).getKey();
                    int start = filteredS.get(l).getKey();
                    if (ans[0] == -1 || end - start + 1 < ans[0]) {
                        ans[0] = end - start + 1;
                        ans[1] = start;
                        ans[2] = end;
                    }

                    windowCounts.put(c, windowCounts.get(c) - 1);
                    if (dictT.containsKey(c) && windowCounts.get(c).intValue() < dictT.get(c).intValue()) {
                        formed--;
                    }
                    l++;
                }
                r++;
            }

            return ans[0] == -1 ? "" : s.substring(ans[1], ans[2] + 1);
        }
    }

    static class Solution7 {
        public String minWindow(String s, String t) {
            if (s.length() == 0 || t.length() == 0) {
                return "";
            }

            Map<Character, Integer> dictT = new HashMap<>();
            for (int i = 0; i < t.length(); i++) {
                int count = dictT.getOrDefault(t.charAt(i), 0);
                dictT.put(t.charAt(i), count + 1);
            }

            int required = dictT.size();

            List<Pair<Integer, Character>> filteredS = new ArrayList<>();
            for (int i = 0; i < s.length(); i++) {
                char c = s.charAt(i);
                if (dictT.containsKey(c)) {
                    filteredS.add(new Pair<>(i, c));
                }
            }

            int l = 0, r = 0, formed = 0;
            Map<Character, Integer> windowCounts = new HashMap<>();
            int[] ans = { -1, 0, 0 };

            while (r < filteredS.size()) {
                char c = filteredS.get(r).getValue();
                int count = windowCounts.getOrDefault(c, 0);
                windowCounts.put(c, count + 1);

                if (dictT.containsKey(c) && windowCounts.get(c).intValue() == dictT.get(c).intValue()) {
                    formed++;
                }

                while (l <= r && formed == required) {
                    c = filteredS.get(l).getValue();

                    int end = filteredS.get(r).getKey();
                    int start = filteredS.get(l).getKey();
                    if (ans[0] == -1 || end - start + 1 < ans[0]) {
                        ans[0] = end - start + 1;
                        ans[1] = start;
                        ans[2] = end;
                    }

                    windowCounts.put(c, windowCounts.get(c) - 1);
                    if (dictT.containsKey(c) && windowCounts.get(c).intValue() < dictT.get(c).intValue()) {
                        formed--;
                    }
                    l++;
                }
                r++;
            }

            return ans[0] == -1 ? "" : s.substring(ans[1], ans[2] + 1);
        }
    }

}
