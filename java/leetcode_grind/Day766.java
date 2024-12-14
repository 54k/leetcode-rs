package leetcode_grind;

import java.util.ArrayDeque;
import java.util.Deque;

public class Day766 {
    // https://leetcode.com/problems/continuous-subarrays/
    static class Solution1 {
        public long continuousSubarrays(int[] nums) {
            Deque<Integer> maxQ = new ArrayDeque<>();
            Deque<Integer> minQ = new ArrayDeque<>();
            int left = 0;
            long count = 0;

            for (int right = 0; right < nums.length; right++) {
                while (!maxQ.isEmpty() && nums[maxQ.peekLast()] < nums[right]) {
                    maxQ.pollLast();
                }
                maxQ.offerLast(right);

                while (!minQ.isEmpty() && nums[minQ.peekLast()] > nums[right]) {
                    minQ.pollLast();
                }
                minQ.offerLast(right);

                while (!maxQ.isEmpty() && !minQ.isEmpty() && nums[maxQ.peekFirst()] - nums[minQ.peekFirst()] > 2) {
                    if (maxQ.peekFirst() < minQ.peekFirst()) {
                        left = maxQ.peekFirst() + 1;
                        maxQ.pollFirst();
                    } else {
                        left = minQ.peekFirst() + 1;
                        minQ.pollFirst();
                    }
                }

                count += right - left + 1;
            }
            return count;
        }
    }

}
