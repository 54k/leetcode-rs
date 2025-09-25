package leetcode_grind;

import java.util.List;

public class Day1041 {
    // https://leetcode.com/problems/triangle/description/?envType=daily-question&envId=2025-09-25
    static class Solution1 {
        public int minimumTotal(List<List<Integer>> triangle) {
            for (int r = triangle.size() - 2; r >= 0; r--) {
                for (int c = 0; c <= r; c++) {
                    int bestRow = Math.min(
                            triangle.get(r + 1).get(c),
                            triangle.get(r + 1).get(c + 1));
                    triangle.get(r).set(c, bestRow + triangle.get(r).get(c));
                }
            }
            return triangle.get(0).get(0);
        }
    }
}
