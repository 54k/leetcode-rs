package leetcode_grind;

import java.util.PriorityQueue;

public class Day462 {
    // https://leetcode.com/problems/furthest-building-you-can-reach/description
    static class Solution1 {
        public int furthestBuilding(int[] heights, int bricks, int ladders) {
            int i = 1;
            var pq = new PriorityQueue<Integer>();
            for (; i < heights.length; i++) {
                int diff = heights[i] - heights[i - 1];
                if (diff < 0) {
                    continue;
                }
                pq.add(diff);

                ladders -= 1;
                if (ladders >= 0) {
                    continue;
                }

                int sm = pq.poll();

                if (bricks < sm) {
                    return i - 1;
                }

                bricks -= sm;
                ladders += 1;
            }
            return i - 1;
        }
    }

}
