package leetcode_grind;

import java.util.PriorityQueue;

public class Day576 {
    // https://leetcode.com/problems/maximum-product-after-k-increments/description/
    static class Solution1 {
        public int maximumProduct(int[] nums, int k) {
            var pq = new PriorityQueue<Integer>();
            var mod = (int) 1e9 + 7;
            for (var n : nums) {
                pq.add(n);
            }
            while (k-- > 0) {
                pq.add(pq.remove() + 1);
            }
            long ans = 1;
            while (!pq.isEmpty()) {
                ans *= pq.remove() % mod;
                ans %= mod;
            }
            return (int) ans;
        }
    }
}