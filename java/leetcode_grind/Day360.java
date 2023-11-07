package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Day360 {
    // https://leetcode.com/problems/unique-paths/description
    static class Solution1 {
        public int uniquePaths(int m, int n) {
            var dp = new int[n];
            Arrays.fill(dp, 1);
            for (var i = 1; i < m; i++) {
                var next = new int[n];
                next[0] = 1;
                for (var j = 1; j < n; j++) {
                    next[j] = next[j - 1] + dp[j];
                }
                dp = next;
            }
            return dp[n - 1];
        }
    }

    // https://leetcode.com/problems/minimum-path-sum/description
    static class Solution2 {
        public int minPathSum(int[][] grid) {
            var m = grid.length;
            var n = grid[0].length;

            for (int i = m - 1; i >= 0; i--) {
                for (int j = n - 1; j >= 0; j--) {
                    if (i == m - 1 && j != n - 1) {
                        grid[i][j] = grid[i][j] + grid[i][j + 1];
                    } else if (j == n - 1 && i != m - 1) {
                        grid[i][j] = grid[i][j] + grid[i + 1][j];
                    } else if (j != n - 1 && i != m - 1) {
                        grid[i][j] = grid[i][j] + Math.min(grid[i + 1][j], grid[i][j + 1]);
                    }
                }
            }

            return grid[0][0];
        }
    }

    // https://leetcode.com/problems/palindrome-partitioning/description/
    static class Solution3 {
        public List<List<String>> partition(String s) {
            var result = new ArrayList<List<String>>();
            var current = new ArrayList<String>();
            var dp = new boolean[s.length()][s.length()];

            var backtrack = new Object() {
                void apply(int start) {
                    if (start >= s.length()) {
                        result.add(new ArrayList<>(current));
                        return;
                    }

                    for (int end = start; end < s.length(); end++) {
                        if (s.charAt(start) == s.charAt(end) && (end - start <= 2 || dp[start + 1][end - 1])) {
                            dp[start][end] = true;
                            current.add(s.substring(start, end + 1));
                            apply(end + 1);
                            current.remove(current.size() - 1);
                        }
                    }
                }
            };

            backtrack.apply(0);
            return result;
        }
    }

    // https://leetcode.com/problems/palindrome-partitioning-ii/description/
    static class Solution4 {
        public int minCutBrute(String s) {
            var isPalindrome = new Object() {
                boolean apply(int start, int end) {
                    while (start < end) {
                        if (s.charAt(start++) != s.charAt(end--)) {
                            return false;
                        }
                    }
                    return true;
                }
            };

            var findMinCut = new Object() {
                int apply(int start, int end, int minimumCut) {
                    if (start == end || isPalindrome.apply(start, end)) {
                        return 0;
                    }

                    for (int currentEndIndex = start; currentEndIndex <= end; currentEndIndex++) {
                        if (isPalindrome.apply(start, currentEndIndex)) {
                            minimumCut = Math.min(
                                    minimumCut,
                                    1 + apply(currentEndIndex + 1, end, minimumCut));
                        }
                    }
                    return minimumCut;
                }
            };

            return findMinCut.apply(0, s.length() - 1, s.length() - 1);
        }

        public int minCutTopDown(String s) {
            var memoCuts = new Integer[s.length()][s.length()];
            var memoPalindrome = new Boolean[s.length()][s.length()];

            var isPalindrome = new Object() {
                boolean apply(int start, int end) {
                    if (start >= end) {
                        return true;
                    }

                    if (memoPalindrome[start][end] != null) {
                        return memoPalindrome[start][end];
                    }

                    memoPalindrome[start][end] = s.charAt(start) == s.charAt(end) &&
                            apply(start + 1, end - 1);
                    return memoPalindrome[start][end];
                }
            };

            var findMinCut = new Object() {
                int apply(int start, int end, int minimumCut) {
                    if (start == end || isPalindrome.apply(start, end)) {
                        return 0;
                    }

                    if (memoCuts[start][end] != null) {
                        return memoCuts[start][end];
                    }

                    for (int currentEndIndex = start; currentEndIndex <= end; currentEndIndex++) {
                        if (isPalindrome.apply(start, currentEndIndex)) {
                            minimumCut = Math.min(
                                    minimumCut,
                                    1 + apply(currentEndIndex + 1, end, minimumCut));
                        }
                    }

                    memoCuts[start][end] = minimumCut;
                    return minimumCut;
                }
            };

            return findMinCut.apply(0, s.length() - 1, s.length() - 1);
        }

        public int minCutTopDownOptimized(String s) {
            var memoCuts = new Integer[s.length()];
            var memoPalindrome = new Boolean[s.length()][s.length()];

            var isPalindrome = new Object() {
                boolean apply(int start, int end) {
                    if (start >= end) {
                        return true;
                    }

                    if (memoPalindrome[start][end] != null) {
                        return memoPalindrome[start][end];
                    }

                    memoPalindrome[start][end] = s.charAt(start) == s.charAt(end) &&
                            apply(start + 1, end - 1);
                    return memoPalindrome[start][end];
                }
            };

            var findMinCut = new Object() {
                int apply(int start, int end, int minimumCut) {
                    if (start == end || isPalindrome.apply(start, end)) {
                        return 0;
                    }

                    if (memoCuts[start] != null) {
                        return memoCuts[start];
                    }

                    for (int currentEndIndex = start; currentEndIndex <= end; currentEndIndex++) {
                        if (isPalindrome.apply(start, currentEndIndex)) {
                            minimumCut = Math.min(
                                    minimumCut,
                                    1 + apply(currentEndIndex + 1, end, minimumCut));
                        }
                    }

                    memoCuts[start] = minimumCut;
                    return minimumCut;
                }
            };

            return findMinCut.apply(0, s.length() - 1, s.length() - 1);
        }

        public int minCutBottomUp(String s) {
            var cutsDp = new Integer[s.length()];

            var palindromeDp = new boolean[s.length()][s.length()];
            for (int end = 0; end < s.length(); end++) {
                for (int start = 0; start <= end; start++) {
                    if (s.charAt(start) == s.charAt(end) && (end - start <= 2 || palindromeDp[start + 1][end - 1])) {
                        palindromeDp[start][end] = true;
                    }
                }
            }

            for (int end = 0; end < s.length(); end++) {
                int minimumCut = end;
                for (int start = 0; start <= end; start++) {
                    if (palindromeDp[start][end]) {
                        minimumCut = start == 0 ? 0 : Math.min(minimumCut, cutsDp[start - 1] + 1);
                    }
                }
                cutsDp[end] = minimumCut;
            }

            return cutsDp[s.length() - 1];
        }

        public int minCutBottomUpOptimized(String s) {
            var cutsDp = new Integer[s.length()];
            var palindromeDp = new boolean[s.length()][s.length()];

            for (int end = 0; end < s.length(); end++) {
                int minimumCut = end;
                for (int start = 0; start <= end; start++) {
                    if (s.charAt(start) == s.charAt(end) && (end - start <= 2 || palindromeDp[start + 1][end - 1])) {
                        palindromeDp[start][end] = true;
                        minimumCut = start == 0 ? 0 : Math.min(minimumCut, cutsDp[start - 1] + 1);
                    }
                }
                cutsDp[end] = minimumCut;
            }

            return cutsDp[s.length() - 1];
        }
    }

    // https://leetcode.com/problems/palindrome-partitioning-iv/description/
    static class Solution5 {
        public boolean checkPartitioning(String s) {
            var end = s.length();
            var palindromeDp = new boolean[s.length()][s.length()];

            for (int i = 0; i < end; i++) {
                for (int j = 0; j <= i; j++) {
                    if (s.charAt(i) == s.charAt(j) && (i - j <= 2 || palindromeDp[j + 1][i - 1])) {
                        palindromeDp[j][i] = true;
                    }
                }
            }

            var backtrack = new Object() {
                boolean solve(int start, int cuts) {
                    if (start == end) {
                        return false;
                    }

                    if (cuts == 1) {
                        return palindromeDp[start][end - 1];
                    }

                    for (int i = start; i < end; i++) {
                        if (palindromeDp[start][i]) {
                            if (solve(i + 1, cuts - 1)) {
                                return true;
                            }
                        }
                    }
                    return false;
                }
            };

            return backtrack.solve(0, 3);
        }
    }
}
