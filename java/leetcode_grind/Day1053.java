package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Queue;
import java.util.Set;

public class Day1053 {
    // https://leetcode.com/problems/pacific-atlantic-water-flow/description/?envType=daily-question&envId=2025-10-07
    static class Solution1 {
        private static final int[][] DIRECTIONS = new int[][] { { 0, 1 }, { 1, 0 }, { -1, 0 }, { 0, -1 } };
        private int numRows;
        private int numCols;
        private int[][] landHeights;

        public List<List<Integer>> pacificAtlantic(int[][] matrix) {
            if (matrix.length == 0 || matrix[0].length == 0) {
                return new ArrayList<>();
            }

            numRows = matrix.length;
            numCols = matrix[0].length;
            landHeights = matrix;

            Queue<int[]> pacificQueue = new LinkedList<>();
            Queue<int[]> atlanticQueue = new LinkedList<>();
            for (int i = 0; i < numRows; i++) {
                pacificQueue.offer(new int[] { i, 0 });
                atlanticQueue.offer(new int[] { i, numCols - 1 });
            }
            for (int i = 0; i < numCols; i++) {
                pacificQueue.offer(new int[] { 0, i });
                atlanticQueue.offer(new int[] { numRows - 1, i });
            }

            boolean[][] pacificReachable = bfs(pacificQueue);
            boolean[][] atlanticReachable = bfs(atlanticQueue);

            List<List<Integer>> commonCells = new ArrayList<>();
            for (int i = 0; i < numRows; i++) {
                for (int j = 0; j < numCols; j++) {
                    if (pacificReachable[i][j] && atlanticReachable[i][j]) {
                        commonCells.add(List.of(i, j));
                    }
                }
            }
            return commonCells;
        }

        private boolean[][] bfs(Queue<int[]> queue) {
            boolean[][] reachable = new boolean[numRows][numCols];

            while (!queue.isEmpty()) {
                int[] cell = queue.poll();
                reachable[cell[0]][cell[1]] = true;

                for (int[] dir : DIRECTIONS) {
                    int newRow = cell[0] + dir[0];
                    int newCol = cell[1] + dir[1];

                    if (newRow < 0 || newRow >= numRows || newCol < 0 || newCol >= numCols) {
                        continue;
                    }

                    if (reachable[newRow][newCol]) {
                        continue;
                    }

                    if (landHeights[newRow][newCol] < landHeights[cell[0]][cell[1]]) {
                        continue;
                    }

                    queue.offer(new int[] { newRow, newCol });
                }
            }

            return reachable;
        }
    }

    static class Solution2 {
        private static final int[][] DIRECTIONS = new int[][] { { 0, 1 }, { 1, 0 }, { -1, 0 }, { 0, -1 } };
        private int numRows;
        private int numCols;
        private int[][] landHeights;

        public List<List<Integer>> pacificAtlantic(int[][] matrix) {
            if (matrix.length == 0 || matrix[0].length == 0) {
                return new ArrayList<>();
            }

            numRows = matrix.length;
            numCols = matrix[0].length;
            landHeights = matrix;

            boolean[][] pacificReachable = new boolean[numRows][numCols];
            boolean[][] atlanticReachable = new boolean[numRows][numCols];

            for (int i = 0; i < numRows; i++) {
                dfs(i, 0, pacificReachable);
                dfs(i, numCols - 1, atlanticReachable);
            }

            for (int i = 0; i < numCols; i++) {
                dfs(0, i, pacificReachable);
                dfs(numRows - 1, i, atlanticReachable);
            }

            List<List<Integer>> commonCells = new ArrayList<>();
            for (int i = 0; i < numRows; i++) {
                for (int j = 0; j < numCols; j++) {
                    if (pacificReachable[i][j] && atlanticReachable[i][j]) {
                        commonCells.add(List.of(i, j));
                    }
                }
            }
            return commonCells;
        }

        private void dfs(int row, int col, boolean[][] reachable) {
            reachable[row][col] = true;

            for (int[] dir : DIRECTIONS) {
                int newRow = row + dir[0];
                int newCol = col + dir[1];

                if (newRow < 0 || newRow >= numRows || newCol < 0 || newCol >= numCols) {
                    continue;
                }

                if (reachable[newRow][newCol]) {
                    continue;
                }

                if (landHeights[newRow][newCol] < landHeights[row][col]) {
                    continue;
                }

                dfs(newRow, newCol, reachable);
            }
        }
    }

    // https://leetcode.com/problems/line-reflection/description/?envType=company&envId=yandex&favoriteSlug=yandex-three-months
    static class Solution3 {
        public boolean isReflected(int[][] points) {
            int min = Integer.MAX_VALUE;
            int max = Integer.MIN_VALUE;

            Map<Integer, Set<Integer>> coords = new HashMap<>();
            for (int[] point : points) {
                coords.computeIfAbsent(point[0], $ -> new HashSet<>()).add(point[1]);
                min = Math.min(min, point[0]);
                max = Math.max(max, point[0]);
            }

            int midx = min + max;
            for (int[] point : points) {
                int reflected = midx - point[0];
                if (!coords.containsKey(reflected) || !coords.get(reflected).contains(point[1])) {
                    return false;
                }
            }
            return true;
        }
    }

    // https://leetcode.com/problems/find-all-anagrams-in-a-string/description/?envType=company&envId=yandex&favoriteSlug=yandex-three-months
    static class Solution4 {
        public List<Integer> findAnagrams(String s, String p) {
            int ns = s.length(), np = p.length();
            if (ns < np)
                return new ArrayList<>();

            Map<Character, Integer> pCount = new HashMap<>();
            Map<Character, Integer> sCount = new HashMap<>();

            for (char ch : p.toCharArray()) {
                if (pCount.containsKey(ch)) {
                    pCount.put(ch, pCount.get(ch) + 1);
                } else {
                    pCount.put(ch, 1);
                }
            }

            List<Integer> output = new ArrayList<>();

            for (int i = 0; i < ns; i++) {
                char ch = s.charAt(i);
                if (sCount.containsKey(ch)) {
                    sCount.put(ch, sCount.get(ch) + 1);
                } else {
                    sCount.put(ch, 1);
                }

                if (i >= np) {
                    ch = s.charAt(i - np);
                    if (sCount.get(ch) == 1) {
                        sCount.remove(ch);
                    } else {
                        sCount.put(ch, sCount.get(ch) - 1);
                    }
                }

                if (pCount.equals(sCount)) {
                    output.add(i - np + 1);
                }
            }

            return output;
        }
    }

    static class Solution5 {
        public List<Integer> findAnagrams(String s, String p) {
            var res = new ArrayList<Integer>();
            if (s.length() < p.length()) {
                return res;
            }

            int[] m = new int[26];
            for (char ch : p.toCharArray()) {
                m[ch - 'a']++;
            }

            int left = 0, right = 0, count = p.length();
            while (right < s.length()) {
                if (m[s.charAt(right++) - 'a']-- >= 1)
                    count--;

                if (count == 0)
                    res.add(left);

                if (right - left + 1 > p.length() && m[s.charAt(left++) - 'a']++ >= 0)
                    count++;
            }

            return res;
        }
    }

    // https://leetcode.com/problems/permutation-in-string/description/
    static class Solution6 {
        public boolean checkInclusion(String s1, String s2) {
            if (s1.length() > s2.length()) {
                return false;
            }

            int[] s1arr = new int[26];
            int[] s2arr = new int[26];
            for (int i = 0; i < s1.length(); i++) {
                s1arr[s1.charAt(i) - 'a']++;
                s2arr[s2.charAt(i) - 'a']++;
            }

            int count = 0;
            for (int i = 0; i < 26; i++) {
                if (s1arr[i] == s2arr[i]) {
                    count++;
                }
            }

            for (int i = 0; i < s2.length() - s1.length(); i++) {
                int r = s2.charAt(i + s1.length()) - 'a', l = s2.charAt(i) - 'a';
                if (count == 26) {
                    return true;
                }

                s2arr[r]++;
                if (s2arr[r] == s1arr[r]) {
                    count++;
                } else if (s2arr[r] == s1arr[r] + 1) {
                    count--;
                }

                s2arr[l]--;
                if (s2arr[l] == s1arr[l]) {
                    count++;
                } else if (s2arr[l] == s1arr[l] - 1) {
                    count--;
                }
            }

            return count == 26;
        }
    }
}
