package leetcode_grind;

import java.util.ArrayDeque;
import java.util.Comparator;
import java.util.PriorityQueue;
import java.util.TreeMap;

public class Day586 {
    // https://leetcode.com/problems/longest-continuous-subarray-with-absolute-diff-less-than-or-equal-to-limit/description/
    static class Solution1 {
        public int longestSubarray(int[] nums, int limit) {
            var inc = new ArrayDeque<Integer>();
            var dec = new ArrayDeque<Integer>();
            var ans = 0;
            for (int i = 0, j = 0; i < nums.length; i++) {
                while (!inc.isEmpty() && nums[inc.peekLast()] >= nums[i]) {
                    inc.removeLast();
                }
                inc.addLast(i);
                while (!dec.isEmpty() && nums[dec.peekLast()] <= nums[i]) {
                    dec.removeLast();
                }
                dec.addLast(i);
                while (Math.abs(nums[inc.peekFirst()] - nums[dec.peekFirst()]) > limit) {
                    if (inc.peekFirst() <= j)
                        inc.removeFirst();
                    if (dec.peekFirst() <= j)
                        dec.removeFirst();
                    j++;
                }
                ans = Math.max(ans, i - j + 1);
            }
            return ans;
        }
    }

    static class Solution2 {
        public int longestSubarray(int[] nums, int limit) {
            PriorityQueue<int[]> maxHeap = new PriorityQueue<>((a, b) -> b[0] - a[0]);
            PriorityQueue<int[]> minHeap = new PriorityQueue<>(Comparator.comparingInt(a -> a[0]));
            int left = 0, maxLength = 0;

            for (int right = 0; right < nums.length; ++right) {
                maxHeap.offer(new int[] { nums[right], right });
                minHeap.offer(new int[] { nums[right], right });
                while (maxHeap.peek()[0] - minHeap.peek()[0] > limit) {
                    left = Math.min(maxHeap.peek()[1], minHeap.peek()[1]) + 1;
                    while (maxHeap.peek()[1] < left) {
                        maxHeap.poll();
                    }
                    while (minHeap.peek()[1] < left) {
                        minHeap.poll();
                    }
                }
                maxLength = Math.max(maxLength, right - left + 1);
            }
            return maxLength;
        }
    }

    static class Solution3 {
        public int longestSubarray(int[] nums, int limit) {
            TreeMap<Integer, Integer> window = new TreeMap<>();
            int left = 0, maxLength = 0;

            for (int right = 0; right < nums.length; ++right) {
                window.put(nums[right], window.getOrDefault(nums[right], 0) + 1);

                while (window.lastKey() - window.firstKey() > limit) {
                    window.put(nums[left], window.get(nums[left]) - 1);
                    if (window.get(nums[left]) == 0) {
                        window.remove(nums[left]);
                    }
                    ++left;
                }
                maxLength = Math.max(maxLength, right - left + 1);
            }
            return maxLength;
        }
    }
}
