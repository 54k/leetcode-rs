package leetcode_grind;

import java.util.*;

public class Day698 {
    // https://leetcode.com/problems/maximal-score-after-applying-k-operations/description/?envType=daily-question&envId=2024-10-14
    static class Solution1 {
        public long maxKelements(int[] nums, int k) {
            var pq = new PriorityQueue<Integer>((a, b) -> b - a);
            for (var num : nums) {
                pq.add(num);
            }
            var ans = 0l;
            while (k-- > 0) {
                var p = pq.poll();
                ans += p;
                pq.add((int) Math.ceil((double) p / 3.0));
            }
            return ans;
        }
    }
}
