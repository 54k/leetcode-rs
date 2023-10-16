package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day338 {
    // https://leetcode.com/problems/pascals-triangle/description/
    static class Solution1 {
        public List<List<Integer>> generate(int numRows) {
            var triangle = new ArrayList<List<Integer>>();
            triangle.add(List.of(1));
            if (numRows == 0) {
                return triangle;
            }
            for (int i = 0; i < numRows - 1; i++) {
                var next = new ArrayList<Integer>();
                next.add(1);
                for (int j = 1; j <= triangle.get(triangle.size() - 1).size() - 1; j++) {
                    next.add(triangle.get(triangle.size() - 1).get(j - 1) + triangle.get(triangle.size() - 1).get(j));
                }
                next.add(1);
                triangle.add(next);
            }
            return triangle;
        }
    }

    // https://leetcode.com/problems/pascals-triangle-ii/description
    static class Solution2 {
        public List<Integer> getRowDP(int rowIndex) {
            var prev = new ArrayList<Integer>();
            prev.add(1);

            for (int i = 1; i <= rowIndex; i++) {
                var curr = new ArrayList<Integer>();
                curr.add(1);

                for (int j = 1; j < i; j++) {
                    curr.add(prev.get(j - 1) + prev.get(j));
                }
                curr.add(1);
                prev = curr;
            }

            return prev;
        }

        public List<Integer> getRowMath(int rowIndex) {
            var row = new ArrayList<Integer>() {
                {
                    add(1);
                }
            };
            for (int k = 1; k <= rowIndex; k++) {
                row.add((int) ((row.get(row.size() - 1) * (long) (rowIndex - k + 1)) / k));
            }
            return row;
        }
    }
}
