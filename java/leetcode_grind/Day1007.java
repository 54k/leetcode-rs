package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Queue;
import java.util.Set;

public class Day1007 {
    // https://leetcode.com/problems/find-the-minimum-area-to-cover-all-ones-i/description/?envType=daily-question&envId=2025-08-22
    static class Solution1 {
        public int minimumArea(int[][] grid) {
            int n = grid.length;
            int m = grid[0].length;

            int min_i = n;
            int max_i = 0;
            int min_j = m;
            int max_j = 0;

            for (int i = 0; i < n; i++) {
                for (int j = 0; j < m; j++) {
                    if (grid[i][j] == 1) {
                        min_i = Math.min(min_i, i);
                        max_i = Math.max(max_i, i);
                        min_j = Math.min(min_j, j);
                        max_j = Math.max(max_j, j);
                    }
                }
            }
            return (max_i - min_i + 1) * (max_j - min_j + 1);
        }
    }

    // https://leetcode.com/problems/before-and-after-puzzle/description/?envType=weekly-question&envId=2025-08-22
    static class Solution2 {
        public List<String> beforeAndAfterPuzzles(String[] phrases) {
            int n = phrases.length;
            String[][] sp = new String[n][2];
            for (int i = 0; i < n; i++) {
                String[] words = phrases[i].split(" ");
                sp[i][0] = words[0];
                sp[i][1] = words[words.length - 1];
            }

            Set<String> m = new HashSet<>();
            for (int i = 0; i < n; i++) {
                for (int j = 0; j < n; j++) {
                    if (i == j)
                        continue;
                    if (sp[i][0].equals(sp[j][1])) {
                        String combined = phrases[j] + phrases[i].substring(sp[i][0].length());
                        m.add(combined);
                    }
                }
            }

            List<String> ret = new ArrayList<>(m);
            Collections.sort(ret);
            return ret;
        }
    }

    // https://leetcode.com/problems/strobogrammatic-number-ii/description/
    static class Solution3 {
        char[][] reversiblePairs = {
                { '0', '0' }, { '1', '1' }, { '6', '9' }, { '8', '8' }, { '9', '6' }
        };

        List<String> generateStroboNumbers(int n, int finalLength) {
            if (n == 0) {
                return new ArrayList<>(List.of(""));
            }

            if (n == 1) {
                return new ArrayList<>(List.of("0", "1", "8"));
            }

            List<String> prevStroboNums = generateStroboNumbers(n - 2, finalLength);
            List<String> currStroboNums = new ArrayList<>();

            for (String prevStroboNum : prevStroboNums) {
                for (char[] pair : reversiblePairs) {
                    if (pair[0] != '0' || n != finalLength) {
                        currStroboNums.add(pair[0] + prevStroboNum + pair[1]);
                    }
                }
            }

            return currStroboNums;
        }

        public List<String> findStrobogrammatic(int n) {
            return generateStroboNumbers(n, n);
        }
    }

    static class Solution4 {
        char[][] reversiblePairs = {
                { '0', '0' }, { '1', '1' }, { '6', '9' }, { '8', '8' }, { '9', '6' }
        };

        public List<String> findStrobogrammatic(int n) {
            Queue<String> q = new LinkedList<>();
            int currStringsLength;

            if (n % 2 == 0) {
                currStringsLength = 0;
                q.add("");
            } else {
                currStringsLength = 1;
                q.add("0");
                q.add("1");
                q.add("8");
            }

            while (currStringsLength < n) {
                currStringsLength += 2;
                for (int i = q.size(); i > 0; --i) {
                    String number = q.poll();
                    for (char[] pair : reversiblePairs) {
                        if (currStringsLength != n || pair[0] != '0') {
                            q.add(pair[0] + number + pair[1]);
                        }
                    }
                }
            }

            List<String> stroboNums = new ArrayList<>();
            while (!q.isEmpty()) {
                stroboNums.add(q.poll());
            }
            return stroboNums;
        }
    }
}
