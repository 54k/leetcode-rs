package leetcode_grind;

import java.util.Deque;
import java.util.LinkedList;

public class Day458 {
    // https://leetcode.com/problems/shortest-subarray-with-sum-at-least-k/
    static class Solution1 {
        public int shortestSubarray(int[] nums, int k) {
            int N = nums.length;
            long[] P = new long[N + 1];
            for (int i = 0; i < N; i++) {
                P[i + 1] = P[i] + nums[i];
            }

            int ans = N + 1;
            Deque<Integer> monoq = new LinkedList<>();

            for (int y = 0; y < P.length; ++y) {
                while (!monoq.isEmpty() && P[y] <= P[monoq.getLast()]) {
                    monoq.removeLast();
                }
                while (!monoq.isEmpty() && P[y] >= P[monoq.getFirst()] + k) {
                    ans = Math.min(ans, y - monoq.removeFirst());
                }
                monoq.addLast(y);
            }

            return ans < N + 1 ? ans : -1;
        }
    }
}
