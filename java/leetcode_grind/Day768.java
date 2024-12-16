package leetcode_grind;

import java.util.PriorityQueue;

public class Day768 {
    // https://leetcode.com/problems/final-array-state-after-k-multiplication-operations-i/description/?envType=daily-question&envId=2024-12-16
    static class Solution1 {
        public int[] getFinalState(int[] nums, int k, int multiplier) {
            var heap = new PriorityQueue<int[]>(
                    (a, b) -> {
                        int valueComparison = Integer.compare(a[0], b[0]);
                        if (valueComparison == 0) {
                            return Integer.compare(a[1], b[1]);
                        }
                        return valueComparison;
                    });

            for (int i = 0; i < nums.length; i++) {
                heap.offer(new int[] { nums[i], i });
            }

            while (k-- > 0) {
                int[] smallest = heap.poll();
                int index = smallest[1];
                nums[index] *= multiplier;
                heap.offer(new int[] { nums[index], index });
            }

            return nums;
        }
    }
}
