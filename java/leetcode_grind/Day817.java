package leetcode_grind;

import java.util.PriorityQueue;

public class Day817 {
    // https://leetcode.com/problems/minimum-operations-to-exceed-threshold-value-ii/description/?envType=daily-question&envId=2025-02-13
    static class Solution1 {
        public int minOperations(int[] nums, int k) {
            var heap = new PriorityQueue<Long>();
            for (var num : nums) {
                heap.add((long) num);
            }
            var ans = 0;
            while (heap.size() >= 2) {
                if (heap.peek() >= k) {
                    break;
                }
                var a = heap.poll();
                var b = heap.poll();
                heap.add(Math.min(a, b) * 2 + Math.max(a, b));
                ans++;
            }
            return ans;
        }
    }

}
