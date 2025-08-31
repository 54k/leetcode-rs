package leetcode_grind;

public class Day1016 {
    // https://leetcode.com/problems/sudoku-solver/description/?envType=daily-question&envId=2025-08-31
    static class Solution1 {
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
