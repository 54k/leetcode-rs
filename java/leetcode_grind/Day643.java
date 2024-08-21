package leetcode_grind;

public class Day643 {
    // https://leetcode.com/problems/strange-printer/description/?envType=daily-question&envId=2024-08-21
    static class Solution1 {
        public int strangePrinter(String s) {
            s = removeDuplicates(s);
            int n = s.length();
            Integer[][] memo = new Integer[n][n];
            return minimumTurns(0, n - 1, s, memo);
        }

        int minimumTurns(int start, int end, String s, Integer[][] memo) {
            if (start > end) {
                return 0;
            }
            if (memo[start][end] != null) {
                return memo[start][end];
            }
            int minTurns = 1 + minimumTurns(start + 1, end, s, memo);

            for (int k = start + 1; k <= end; k++) {
                if (s.charAt(k) == s.charAt(start)) {
                    int turnsWithMatch = minimumTurns(start, k - 1, s, memo) +
                            minimumTurns(k + 1, end, s, memo);
                    minTurns = Math.min(minTurns, turnsWithMatch);
                }
            }

            return memo[start][end] = minTurns;
        }

        String removeDuplicates(String s) {
            StringBuilder uniqueChars = new StringBuilder();
            int i = 0;
            while (i < s.length()) {
                char currentChar = s.charAt(i);
                uniqueChars.append(currentChar);
                while (i < s.length() && s.charAt(i) == currentChar) {
                    i++;
                }
            }
            return uniqueChars.toString();
        }
    }

    static class Solution2 {
        public int strangePrinter(String s) {
            s = removeDuplicates(s);
            int n = s.length();
            int[][] minTurns = new int[n][n];
            for (int i = 0; i < n; i++) {
                minTurns[i][i] = 1;
            }
            for (int length = 2; length <= n; length++) {
                for (int start = 0; start + length - 1 < n; start++) {
                    int end = start + length - 1;

                    minTurns[start][end] = length;

                    for (int split = 0; split < length - 1; split++) {
                        int totalTurns = minTurns[start][start + split] +
                                minTurns[start + split + 1][end];

                        if (s.charAt(start + split) == s.charAt(end)) {
                            totalTurns--;
                        }

                        minTurns[start][end] = Math.min(
                                minTurns[start][end],
                                totalTurns);
                    }
                }
            }
            return minTurns[0][n - 1];
        }

        String removeDuplicates(String s) {
            StringBuilder uniqueChars = new StringBuilder();
            int i = 0;
            while (i < s.length()) {
                char currentChar = s.charAt(i);
                uniqueChars.append(currentChar);
                while (i < s.length() && s.charAt(i) == currentChar) {
                    i++;
                }
            }
            return uniqueChars.toString();
        }
    }
}
