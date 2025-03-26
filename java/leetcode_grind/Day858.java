package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class Day858 {

    // https://leetcode.com/problems/minimum-operations-to-make-a-uni-value-grid/description/
    static class Solution1 {
        public int minOperations(int[][] grid, int x) {
            List<Integer> numsArray = new ArrayList<>();
            int result = 0;
            for (int row = 0; row < grid.length; row++) {
                for (int col = 0; col < grid[0].length; col++) {
                    numsArray.add(grid[row][col]);
                }
            }

            Collections.sort(numsArray);
            int length = numsArray.size();
            int finalCommonNumber = numsArray.get(length / 2);

            for (int number : numsArray) {
                if (number % x != finalCommonNumber % x) {
                    return -1;
                }
                result += Math.abs(finalCommonNumber - number) / x;
            }
            return result;
        }
    }

    static class Solution2 {
        public int minOperations(int[][] grid, int x) {
            List<Integer> numsArray = new ArrayList<>();
            int result = 0;

            for (int row = 0; row < grid.length; row++) {
                for (int col = 0; col < grid[0].length; col++) {
                    if (grid[row][col] % x != grid[0][0] % x)
                        return -1;
                    numsArray.add(grid[row][col]);
                }
            }

            Collections.sort(numsArray);
            int length = numsArray.size();
            int prefixIndex = 0;
            int suffixIndex = length - 1;

            while (prefixIndex < suffixIndex) {
                if (prefixIndex < length - suffixIndex - 1) {
                    int prefixOperations = ((prefixIndex + 1)
                            * (numsArray.get(prefixIndex + 1) - numsArray.get(prefixIndex))) / x;
                    result += prefixOperations;
                    prefixIndex++;
                } else {
                    int suffixOperations = ((length - suffixIndex)
                            * (numsArray.get(suffixIndex) - numsArray.get(suffixIndex - 1))) / x;
                    result += suffixOperations;
                    suffixIndex--;
                }
            }

            return result;
        }
    }
}
