package leetcode_grind;

import java.util.HashMap;
import java.util.Stack;

public class Day794 {
    // https://leetcode.com/problems/grid-game/description/?envType=daily-question&envId=2025-01-21
    static class Solution1 {
        public long gridGame(int[][] grid) {
            long firstRowSum = 0;
            for (int num : grid[0]) {
                firstRowSum += num;
            }
            long secondRowSum = 0;
            long minimumSum = Long.MAX_VALUE;
            for (int turnIndex = 0; turnIndex < grid[0].length; ++turnIndex) {
                firstRowSum -= grid[0][turnIndex];
                minimumSum = Math.min(minimumSum, Math.max(firstRowSum, secondRowSum));
                secondRowSum += grid[1][turnIndex];
            }
            return minimumSum;
        }
    }

    // https://leetcode.com/problems/minimum-penalty-for-a-shop/description/
    static class Solution2 {
        public int bestClosingTime(String customers) {
            int curPenalty = 0;
            for (int i = 0; i < customers.length(); i++) {
                if (customers.charAt(i) == 'Y') {
                    curPenalty++;
                }
            }

            // Start with closing at hour 0, the penalty equals all 'Y' in closed hours.
            int minPenalty = curPenalty;
            int earliestHour = 0;

            for (int i = 0; i < customers.length(); i++) {
                char ch = customers.charAt(i);

                // If status in hour i is 'Y', moving it to open hours decrement
                // penalty by 1. Otherwise, moving 'N' to open hours increment
                // penatly by 1.
                if (ch == 'Y') {
                    curPenalty--;
                } else {
                    curPenalty++;
                }

                // Update earliestHour if a smaller penatly is encountered.
                if (curPenalty < minPenalty) {
                    earliestHour = i + 1;
                    minPenalty = curPenalty;
                }
            }

            return earliestHour;
        }
    }

    static class Solution3 {
        public int bestClosingTime(String customers) {
            int minPenalty = 0, curPenalty = 0;
            int earliestHour = 0;

            for (int i = 0; i < customers.length(); i++) {
                char ch = customers.charAt(i);

                if (ch == 'Y') {
                    curPenalty--;
                } else {
                    curPenalty++;
                }

                if (curPenalty < minPenalty) {
                    earliestHour = i + 1;
                    minPenalty = curPenalty;
                }
            }
            return earliestHour;
        }
    }

    // https://leetcode.com/problems/design-excel-sum-formula/description/
    class Excel {
        Formula[][] formulas;

        static class Formula {
            HashMap<String, Integer> cells;
            int val;

            Formula(HashMap<String, Integer> c, int v) {
                val = v;
                cells = c;
            }
        }

        Stack<int[]> stack = new Stack<>();

        public Excel(int height, char width) {
            formulas = new Formula[height][(width - 'A') + 1];
        }

        public void set(int row, char column, int val) {
            formulas[row - 1][column - 'A'] = new Formula(new HashMap<>(), val);
            topologicalSort(row - 1, column - 'A');
            executeStack();
        }

        public int get(int row, char column) {
            if (formulas[row - 1][column - 'A'] == null) {
                return 0;
            }
            return formulas[row - 1][column - 'A'].val;
        }

        public int sum(int row, char column, String[] numbers) {
            HashMap<String, Integer> cells = convert(numbers);
            int summ = calculateSum(row - 1, column - 'A', cells);
            set(row, column, summ);
            formulas[row - 1][column - 'A'] = new Formula(cells, summ);
            return summ;
        }

        void topologicalSort(int r, int c) {
            for (int i = 0; i < formulas.length; i++) {
                for (int j = 0; j < formulas[0].length; j++) {
                    if (formulas[i][j] != null && formulas[i][j].cells.containsKey("" + (char) ('A' + c) + (r + 1))) {
                        topologicalSort(i, j);
                    }
                }
            }
            stack.push(new int[] { r, c });
        }

        void executeStack() {
            while (!stack.isEmpty()) {
                int[] top = stack.pop();
                if (formulas[top[0]][top[1]].cells.size() > 0) {
                    calculateSum(top[0], top[1], formulas[top[0]][top[1]].cells);
                }
            }
        }

        HashMap<String, Integer> convert(String[] strs) {
            HashMap<String, Integer> res = new HashMap<>();
            for (String st : strs) {
                if (st.indexOf(":") < 0) {
                    res.put(st, res.getOrDefault(st, 0) + 1);
                } else {
                    String[] cells = st.split(":");
                    int si = Integer.parseInt(cells[0].substring(1)), ei = Integer.parseInt(cells[1].substring(1));
                    char sj = cells[0].charAt(0), ej = cells[1].charAt(0);
                    for (int i = si; i <= ei; i++) {
                        for (char j = sj; j <= ej; j++) {
                            res.put("" + j + i, res.getOrDefault("" + j + i, 0) + 1);
                        }
                    }
                }
            }
            return res;
        }

        int calculateSum(int r, int c, HashMap<String, Integer> cells) {
            int sum = 0;
            for (String s : cells.keySet()) {
                int x = Integer.parseInt(s.substring(1)) - 1, y = s.charAt(0) - 'A';
                sum += (formulas[x][y] != null ? formulas[x][y].val : 0) * cells.get(s);
            }
            formulas[r][c] = new Formula(cells, sum);
            return sum;
        }
    }

    /**
     * Your Excel object will be instantiated and called as such:
     * Excel obj = new Excel(height, width);
     * obj.set(row,column,val);
     * int param_2 = obj.get(row,column);
     * int param_3 = obj.sum(row,column,numbers);
     */
}
