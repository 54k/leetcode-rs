package leetcode_grind;

import java.util.HashMap;
import java.util.Iterator;
import java.util.Map;
import java.util.PriorityQueue;

public class Day1140 {
    // https://leetcode.com/problems/n-repeated-element-in-size-2n-array/description/?envType=daily-question&envId=2026-01-02
    static class Solution1 {
        public int repeatedNTimes(int[] nums) {
            for (int k = 1; k <= 3; k++) {
                for (int i = 0; i < nums.length - k; ++i) {
                    if (nums[i] == nums[i + k]) {
                        return nums[i];
                    }
                }
            }
            throw null;
        }
    }

    // https://leetcode.com/problems/design-a-leaderboard/description/?envType=weekly-question&envId=2026-01-01
    static class Leaderboard1 {
        Map<Integer, Integer> scores;

        public Leaderboard1() {
            scores = new HashMap<>();
        }

        public void addScore(int playerId, int score) {
            if (!this.scores.containsKey(playerId)) {
                this.scores.put(playerId, 0);
            }
            this.scores.put(playerId, this.scores.get(playerId) + score);
        }

        public int top(int K) {
            PriorityQueue<Map.Entry<Integer, Integer>> heap = new PriorityQueue<>(
                    (a, b) -> a.getValue() - b.getValue());
            for (Map.Entry<Integer, Integer> entry : this.scores.entrySet()) {
                heap.offer(entry);
                if (heap.size() > K) {
                    heap.poll();
                }
            }

            int total = 0;
            Iterator<Map.Entry<Integer, Integer>> value = heap.iterator();
            while (value.hasNext()) {
                total += value.next().getValue();
            }
            return total;
        }

        public void reset(int playerId) {
            this.scores.put(playerId, 0);
        }
    }

}
