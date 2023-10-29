package leetcode_grind;

import java.util.PriorityQueue;

public class Day351 {
    // https://leetcode.com/problems/eliminate-maximum-number-of-monsters/description/
    static class Solution {
        public int eliminateMaximum(int[] dist, int[] speed) {
            var heap = new PriorityQueue<Double>();
            for (int i = 0; i < speed.length; i++) {
                heap.add((double) dist[i] / speed[i]);
            }
            var ans = 0;
            while (heap.size() > 0) {
                if (heap.poll() <= ans) {
                    break;
                }
                ans++;
            }
            return ans;
        }
    }
}
