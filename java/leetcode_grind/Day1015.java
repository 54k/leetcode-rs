package leetcode_grind;

import java.util.HashSet;
import java.util.Set;

public class Day1015 {
    // https://leetcode.com/problems/valid-sudoku/description/?envType=daily-question&envId=2025-08-30
    static class Solution1 {
        public boolean isValidSudoku(char[][] board) {
            int N = 9;

            Set<Character>[] rows = new HashSet[N];
            Set<Character>[] cols = new HashSet[N];
            Set<Character>[] boxes = new HashSet[N];

            for (int r = 0; r < N; r++) {
                rows[r] = new HashSet<>();
                cols[r] = new HashSet<>();
                boxes[r] = new HashSet<>();
            }

            for (int r = 0; r < N; r++) {
                for (int c = 0; c < N; c++) {
                    char val = board[r][c];

                    if (val == '.') {
                        continue;
                    }

                    if (rows[r].contains(val)) {
                        return false;
                    }
                    rows[r].add(val);

                    if (cols[c].contains(val)) {
                        return false;
                    }
                    cols[c].add(val);

                    int idx = (r / 3) * 3 + c / 3;
                    if (boxes[idx].contains(val)) {
                        return false;
                    }
                    boxes[idx].add(val);
                }
            }
            return true;
        }
    }

    // https://leetcode.com/problems/sudoku-solver/description/
    static class Solution2 {
        int n = 3;
        int N = n * n;

        int[][] rows = new int[N][N + 1];
        int[][] columns = new int[N][N + 1];
        int[][] boxes = new int[N][N + 1];

        char[][] board;

        boolean sudokuSolved = false;

        boolean couldPlace(int d, int row, int col) {
            int idx = (row / n) * n + col / n;
            return rows[row][d] + columns[col][d] + boxes[idx][d] == 0;
        }

        void placeNumber(int d, int row, int col) {
            int idx = (row / n) * n + col / n;
            rows[row][d]++;
            columns[col][d]++;
            boxes[idx][d]++;
            board[row][col] = (char) (d + '0');
        }

        void removeNumber(int d, int row, int col) {
            int idx = (row / n) * n + col / n;
            rows[row][d]--;
            columns[col][d]--;
            boxes[idx][d]--;
            board[row][col] = '.';
        }

        void placeNextNumbers(int row, int col) {
            if ((col == N - 1) && (row == N - 1)) {
                sudokuSolved = true;
            } else {
                if (col == N - 1)
                    backtrack(row + 1, 0);
                else
                    backtrack(row, col + 1);
            }
        }

        void backtrack(int row, int col) {
            if (board[row][col] == '.') {
                for (int d = 1; d < 10; d++) {
                    if (couldPlace(d, row, col)) {
                        placeNumber(d, row, col);
                        placeNextNumbers(row, col);
                        if (!sudokuSolved)
                            removeNumber(d, row, col);
                    }
                }
            } else
                placeNextNumbers(row, col);
        }

        public void solveSudoku(char[][] board) {
            this.board = board;
            for (int i = 0; i < N; i++) {
                for (int j = 0; j < N; j++) {
                    char num = board[i][j];
                    if (num != '.') {
                        int d = Character.getNumericValue(num);
                        placeNumber(d, i, j);
                    }
                }
            }
            backtrack(0, 0);
        }
    }
}
