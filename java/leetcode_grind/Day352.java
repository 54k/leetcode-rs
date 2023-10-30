package leetcode_grind;

import java.util.ArrayList;

public class Day352 {
    // https://leetcode.com/problems/zigzag-conversion/description/
    static class Solution1 {
        public String convert(String s, int numRows) {
            return "";
        }
    }

    // https://leetcode.com/problems/interval-list-intersections/description/
    static class Solution2 {
        public int[][] intervalIntersection(int[][] firstList, int[][] secondList) {
            var ans = new ArrayList<int[]>();
            int i = 0, j = 0;
            while (i < firstList.length && j < secondList.length) {
                var lo = Math.max(firstList[i][0], secondList[j][0]);
                var hi = Math.min(firstList[i][1], secondList[j][1]);
                if (lo <= hi) {
                    ans.add(new int[] { lo, hi });
                }

                if (firstList[i][1] < secondList[j][1]) {
                    i++;
                } else {
                    j++;
                }
            }
            return ans.toArray(new int[][] {});
        }
    }
}
