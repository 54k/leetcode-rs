package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.PriorityQueue;

public class Day595 {
    // https://leetcode.com/problems/minimum-difference-between-largest-and-smallest-value-in-three-moves/description/?envType=daily-question&envId=2024-07-03
    static class Solution1 {
        public int minDifference(int[] nums) {
            int numsSize = nums.length;
            if (numsSize <= 4)
                return 0;
            Arrays.sort(nums);
            int minDiff = Integer.MAX_VALUE;
            for (int left = 0, right = numsSize - 4; left < 4; left++, right++) {
                minDiff = Math.min(minDiff, nums[right] - nums[left]);
            }
            return minDiff;
        }
    }

    static class Solution2 {
        public int minDifference(int[] nums) {
            int numsSize = nums.length;
            if (numsSize <= 4)
                return 0;
            PriorityQueue<Integer> maxHeap = new PriorityQueue<>(Collections.reverseOrder());

            for (int num : nums) {
                maxHeap.offer(num);
                if (maxHeap.size() > 4) {
                    maxHeap.poll();
                }
            }

            List<Integer> smallestFour = new ArrayList<>(maxHeap);
            Collections.sort(smallestFour);

            PriorityQueue<Integer> minHeap = new PriorityQueue<>();
            for (int num : nums) {
                minHeap.offer(num);
                if (minHeap.size() > 4) {
                    minHeap.poll();
                }
            }

            List<Integer> largestFour = new ArrayList<>(minHeap);
            Collections.sort(largestFour);
            int minDiff = Integer.MAX_VALUE;
            for (int i = 0; i < 4; i++) {
                minDiff = Math.min(minDiff, largestFour.get(i) - smallestFour.get(i));
            }
            return minDiff;
        }
    }
}
