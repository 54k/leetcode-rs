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

}
