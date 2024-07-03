package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.PriorityQueue;
import java.util.Random;

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

    // https://leetcode.com/problems/kth-largest-element-in-an-array/description/
    static class Solution3 {
        public int findKthLargest(int[] nums, int k) {
            List<Integer> list = new ArrayList<>();
            for (int num : nums) {
                list.add(num);
            }
            return quickSelect(list, k);
        }

        int quickSelect(List<Integer> nums, int k) {
            int pivotIndex = new Random().nextInt(nums.size());
            int pivot = nums.get(pivotIndex);

            List<Integer> left = new ArrayList<>();
            List<Integer> mid = new ArrayList<>();
            List<Integer> right = new ArrayList<>();

            for (int num : nums) {
                if (num > pivot) {
                    left.add(num);
                } else if (num < pivot) {
                    right.add(num);
                } else {
                    mid.add(num);
                }
            }

            if (k <= left.size()) {
                return quickSelect(left, k);
            }
            if (left.size() + mid.size() < k) {
                return quickSelect(right, k - left.size() - mid.size());
            }
            return pivot;
        }
    }
}
