package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day986 {
    // https://leetcode.com/problems/pascals-triangle/description/?envType=daily-question&envId=2025-08-01
    static class Solution1 {
        public List<List<Integer>> generate(int numRows) {
            List<List<Integer>> triangle = new ArrayList<>();
            triangle.add(new ArrayList<>());
            triangle.get(0).add(1);

            for (int rowNum = 1; rowNum < numRows; rowNum++) {
                List<Integer> row = new ArrayList<>();
                List<Integer> prevRow = triangle.get(rowNum - 1);

                row.add(1);

                for (int j = 1; j < rowNum; j++) {
                    row.add(prevRow.get(j - 1) + prevRow.get(j));
                }

                row.add(1);

                triangle.add(row);
            }

            return triangle;
        }
    }

}
