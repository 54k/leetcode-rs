package leetcode_grind;

import java.util.List;
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

    // https://leetcode.com/problems/brick-wall/description/
    static class Solution3 {
        public int leastBricks(List<List<Integer>> wall) {
            int[] pos = new int[wall.size()];
            int c = 0, width = 0, res = Integer.MAX_VALUE;
            for (var brick : wall.get(0)) {
                width += brick;
            }

            while (width != 0) {
                int count = 0;
                for (int i = 0; i < wall.size(); i++) {
                    List<Integer> row = wall.get(i);
                    if (row.get(pos[i]) != 0) {
                        count++;
                    } else {
                        pos[i]++;
                    }
                    row.set(pos[i], row.get(pos[i]) - 1);
                }
                width--;
                res = Math.min(res, count);
            }
            return res;
        }
    }
}
