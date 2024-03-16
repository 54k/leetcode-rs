package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;

public class Day489 {
    // https://leetcode.com/problems/contiguous-array/
    static class Solution1 {
        public int findMaxLength(int[] nums) {
            var hm = new HashMap<Integer, Integer>();
            hm.put(0, -1);
            int maxlen = 0, count = 0;
            for (int i = 0; i < nums.length; i++) {
                count = count + (nums[i] == 1 ? 1 : -1);
                if (hm.containsKey(count)) {
                    maxlen = Math.max(maxlen, i - hm.get(count));
                } else {
                    hm.put(count, i);
                }
            }
            return maxlen;
        }
    }

    // https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/
    static class Solution2 {
        public int findMinArrowShots(int[][] points) {
            Arrays.sort(points, (a, b) -> a[1] == b[1] ? 0 : (a[1] < b[1] ? -1 : 1));
            var pe = points[0][1];
            var ans = 1;
            for (int i = 1; i < points.length; i++) {
                if (pe < points[i][0]) {
                    ans++;
                    pe = points[i][1];
                }
            }
            return ans;
        }
    }
}
