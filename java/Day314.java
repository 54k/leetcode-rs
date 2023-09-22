import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;

public class Day314 {
    // https://leetcode.com/problems/median-of-a-row-wise-sorted-matrix/description/
    static class Solution1 {
        public int matrixMedian(int[][] grid) {
            var binSearch = new Object() {
                int apply(int[] cols, int target) {
                    var left = 0;
                    var right = cols.length - 1;

                    while (left <= right) {
                        var mid = (left + right) / 2;
                        if (cols[mid] >= target) {
                            right = mid - 1;
                        } else {
                            left = mid + 1;
                        }
                    }
                    return left;
                }
            };

            var m = grid.length;
            var n = grid[0].length;

            var half = (m * n) / 2;
            var left = 1;
            var right = (int) 10e6;

            while (left + 1 < right) {
                var mid = (left + right) / 2;

                var sum = 0;
                for (var col : grid) {
                    sum += binSearch.apply(col, mid);
                }

                if (sum > half) {
                    right = mid;
                } else {
                    left = mid;
                }
            }

            return left;
        }
    }

    // https://leetcode.com/problems/is-subsequence/description
    class Solution2 {
        public boolean isSubsequence(String s, String t) {
            var indices = new HashMap<Character, List<Integer>>();
            var i = 0;
            for (char c : t.toCharArray()) {
                indices.putIfAbsent(c, new ArrayList<>());
                indices.get(c).add(i++);
            }

            var currMatchIndex = -1;
            for (char c : s.toCharArray()) {
                if (!indices.containsKey(c)) {
                    return false;
                }

                var isMatched = false;
                for (var matchIndex : indices.get(c)) {
                    if (currMatchIndex < matchIndex) {
                        currMatchIndex = matchIndex;
                        isMatched = true;
                        break;
                    }
                }

                if (!isMatched) {
                    return false;
                }
            }

            return true;
        }
    }
}
