package leetcode_grind;

import java.util.PriorityQueue;

public class Day351 {
    // https://leetcode.com/problems/eliminate-maximum-number-of-monsters/description/
    static class Solution1 {
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

    // https://leetcode.com/problems/minimum-health-to-beat-game/
    static class Solution2 {
        public long minimumHealth(int[] damage, int armor) {
            long healthLost = 0;
            var maxDamage = 0;
            for (var d : damage) {
                maxDamage = Math.max(maxDamage, d);
                healthLost += d;
            }

            return healthLost + 1 - Math.min(armor, maxDamage);
        }
    }
}
