package leetcode_grind;

import java.util.*;

public class Day726 {
    // https://leetcode.com/problems/most-beautiful-item-for-each-query/description/?envType=daily-question&envId=2024-11-12
    static class Solution1 {
        public int[] maximumBeauty(int[][] items, int[] queries) {
            Arrays.sort(items, (a, b) -> a[0] - b[0]);
            int max = items[0][1];
            for (int i = 0; i < items.length; i++) {
                max = Math.max(max, items[i][1]);
                items[i][1] = max;
            }

            var ans = new int[queries.length];
            var idx = 0;
            for (var q : queries) {
                var lo = -1;
                var hi = items.length;
                while (lo + 1 < hi) {
                    var mid = (lo + hi) / 2;
                    if (items[mid][0] <= q) {
                        lo = mid;
                    } else {
                        hi = mid;
                    }
                }
                ans[idx++] = lo > -1 ? items[lo][1] : 0;
            }
            return ans;
        }
    }
}
