package leetcode_grind;

public class Day630 {
    // https://leetcode.com/problems/spiral-matrix-iii/description/?envType=daily-question&envId=2024-08-08
    static class Solution1 {
        public int[][] spiralMatrixIII(int rows, int cols, int rStart, int cStart) {
            var d = 0;
            var dir = new int[][] { { 0, 1 }, { 1, 0 }, { 0, -1 }, { -1, 0 } };
            var vis = rows * cols - 1;
            var len = 1;
            var x = rStart;
            var y = cStart;
            var path = new int[rows * cols][2];
            var pidx = 1;
            path[0] = new int[] { rStart, cStart };
            while (vis > 0) {
                for (int $ = 0; $ < 2; $++) {
                    for (int i = 0; i < len; i++) {
                        x += dir[d][0];
                        y += dir[d][1];
                        if (0 <= x && x < rows && 0 <= y && y < cols) {
                            path[pidx++] = new int[] { x, y };
                            vis--;
                        }
                    }
                    d = (d + 1) % dir.length;
                }
                len++;
            }
            return path;
        }
    }

    static class Solution2 {
        public int[][] spiralMatrixIII(int rows, int cols, int rStart, int cStart) {
            int[][] dir = new int[][] { { 0, 1 }, { 1, 0 }, { 0, -1 }, { -1, 0 } };
            int[][] traversed = new int[rows * cols][2];
            int idx = 0;
            for (int step = 1, direction = 0; idx < rows * cols;) {
                for (int i = 0; i < 2; i++) {
                    for (int j = 0; j < step; ++j) {
                        if (rStart >= 0 && rStart < rows && cStart >= 0 && cStart < cols) {
                            traversed[idx][0] = rStart;
                            traversed[idx][1] = cStart;
                            ++idx;
                        }
                        rStart += dir[direction][0];
                        cStart += dir[direction][1];
                    }
                    direction = (direction + 1) % 4;
                }
                ++step;
            }
            return traversed;
        }
    }

    // https://leetcode.com/problems/android-unlock-patterns/description/?envType=weekly-question&envId=2024-08-08
    static class Solution3 {
        private static final int[][] SINGLE_STEP_MOVES = {
                { 0, 1 },
                { 0, -1 },
                { 1, 0 },
                { -1, 0 },
                { 1, 1 },
                { -1, 1 },
                { 1, -1 },
                { -1, -1 },
                { -2, 1 },
                { -2, -1 },
                { 2, 1 },
                { 2, -1 },
                { 1, -2 },
                { -1, -2 },
                { 1, 2 },
                { -1, 2 },
        };

        private static final int[][] SKIP_DOT_MOVES = {
                { 0, 2 },
                { 0, -2 },
                { 2, 0 },
                { -2, 0 },
                { -2, -2 },
                { 2, 2 },
                { 2, -2 },
                { -2, 2 },
        };

        public int numberOfPatterns(int m, int n) {
            int totalPatterns = 0;
            for (int row = 0; row < 3; row++) {
                for (int col = 0; col < 3; col++) {
                    boolean[][] visitedDots = new boolean[3][3];
                    totalPatterns += countPatternsFromDot(m, n, 1, row, col, visitedDots);
                }
            }
            return totalPatterns;
        }

        private int countPatternsFromDot(int m, int n, int currentLength, int currentRow, int currentCol,
                boolean[][] visitedDots) {
            if (currentLength > n) {
                return 0;
            }
            int validPatterns = 0;
            if (currentLength >= m) {
                validPatterns++;
            }
            visitedDots[currentRow][currentCol] = true;

            for (int[] move : SINGLE_STEP_MOVES) {
                int newRow = currentRow + move[0];
                int newCol = currentCol + move[1];
                if (isValidMove(newRow, newCol, visitedDots)) {
                    validPatterns += countPatternsFromDot(m, n, currentLength + 1, newRow, newCol, visitedDots);
                }
            }

            for (int[] move : SKIP_DOT_MOVES) {
                int newRow = currentRow + move[0];
                int newCol = currentCol + move[1];
                if (isValidMove(newRow, newCol, visitedDots)) {
                    int middleRow = currentRow + move[0] / 2;
                    int middleCol = currentCol + move[1] / 2;
                    if (visitedDots[middleRow][middleCol]) {
                        validPatterns += countPatternsFromDot(m, n, currentLength + 1, newRow, newCol, visitedDots);
                    }
                }
            }

            visitedDots[currentRow][currentCol] = false;
            return validPatterns;
        }

        boolean isValidMove(int row, int col, boolean[][] visitedDots) {
            return row >= 0 && col >= 0 && row < 3 && col < 3 && !visitedDots[row][col];
        }
    }

    static class Solution4 {
        public int numberOfPatterns(int m, int n) {
            int[][] jump = new int[10][10];
            jump[1][3] = jump[3][1] = 2;
            jump[4][6] = jump[6][4] = 5;
            jump[7][9] = jump[9][7] = 8;
            jump[1][7] = jump[7][1] = 4;
            jump[2][8] = jump[8][2] = 5;
            jump[3][9] = jump[9][3] = 6;
            jump[1][9] = jump[9][1] = jump[3][7] = jump[7][3] = 5;

            boolean[] visitedNumbers = new boolean[10];
            int totalPatterns = 0;

            totalPatterns += countPatternsFromNumber(1, 1, m, n, jump, visitedNumbers) * 4;
            totalPatterns += countPatternsFromNumber(2, 1, m, n, jump, visitedNumbers) * 4;
            totalPatterns += countPatternsFromNumber(5, 1, m, n, jump, visitedNumbers);
            return totalPatterns;
        }

        int countPatternsFromNumber(int currentNumber, int currentLength, int minLength, int maxLength, int[][] jump,
                boolean[] visitedNumbers) {
            if (currentLength > maxLength)
                return 0;
            int validPatterns = 0;
            if (currentLength >= minLength) {
                validPatterns++;
            }
            visitedNumbers[currentNumber] = true;
            for (int nextNumber = 1; nextNumber <= 9; nextNumber++) {
                int jumpOverNumber = jump[currentNumber][nextNumber];
                if (!visitedNumbers[nextNumber] && (jumpOverNumber == 0 || visitedNumbers[jumpOverNumber])) {
                    validPatterns += countPatternsFromNumber(nextNumber, currentLength + 1, minLength, maxLength, jump,
                            visitedNumbers);
                }
            }
            visitedNumbers[currentNumber] = false;
            return validPatterns;
        }
    }

    static class Solution5 {

        public int numberOfPatterns(int m, int n) {
            int[][] jump = new int[10][10];

            // Initialize the jump over numbers for all valid jumps
            jump[1][3] = jump[3][1] = 2;
            jump[4][6] = jump[6][4] = 5;
            jump[7][9] = jump[9][7] = 8;
            jump[1][7] = jump[7][1] = 4;
            jump[2][8] = jump[8][2] = 5;
            jump[3][9] = jump[9][3] = 6;
            jump[1][9] = jump[9][1] = jump[3][7] = jump[7][3] = 5;

            int visitedNumbers = 0;
            int totalPatterns = 0;
            Integer[][] dp = new Integer[10][1 << 10];

            // Count patterns starting from corner numbers (1, 3, 7, 9) and multiply by 4
            // due to symmetry
            totalPatterns += countPatternsFromNumber(1, 1, m, n, jump, visitedNumbers, dp) * 4;

            // Count patterns starting from edge numbers (2, 4, 6, 8) and multiply by 4 due
            // to symmetry
            totalPatterns += countPatternsFromNumber(2, 1, m, n, jump, visitedNumbers, dp) * 4;

            // Count patterns starting from the center number (5)
            totalPatterns += countPatternsFromNumber(5, 1, m, n, jump, visitedNumbers, dp);

            return totalPatterns;
        }

        private int countPatternsFromNumber(
                int currentNumber,
                int currentLength,
                int minLength,
                int maxLength,
                int[][] jump,
                int visitedNumbers,
                Integer[][] dp) {
            // Base case: if current pattern length exceeds maxLength, stop exploring
            if (currentLength > maxLength)
                return 0;

            if (dp[currentNumber][visitedNumbers] != null)
                return dp[currentNumber][visitedNumbers];

            int validPatterns = 0;
            // If current pattern length is within the valid range, count it
            if (currentLength >= minLength) {
                validPatterns++;
            }

            visitedNumbers = setBit(visitedNumbers, currentNumber);

            // Explore all possible next numbers
            for (int nextNumber = 1; nextNumber <= 9; nextNumber++) {
                int jumpOverNumber = jump[currentNumber][nextNumber];
                // Check if the next number is unvisited and either:
                // 1. There's no number to jump over, or
                // 2. The number to jump over has been visited
                if (!isSet(visitedNumbers, nextNumber) &&
                        (jumpOverNumber == 0 || isSet(visitedNumbers, jumpOverNumber))) {
                    validPatterns += countPatternsFromNumber(
                            nextNumber,
                            currentLength + 1,
                            minLength,
                            maxLength,
                            jump,
                            visitedNumbers,
                            dp);
                }
            }

            // Backtrack: unmark the current number before returning
            visitedNumbers = clearBit(visitedNumbers, currentNumber);

            return dp[currentNumber][visitedNumbers] = validPatterns;
        }

        private int setBit(int num, int position) {
            num |= 1 << (position - 1);
            return num;
        }

        private int clearBit(int num, int position) {
            num ^= 1 << (position - 1);
            return num;
        }

        private boolean isSet(int num, int position) {
            int bitAtPosition = (num >> (position - 1)) & 1;
            return bitAtPosition == 1;
        }
    }
}
