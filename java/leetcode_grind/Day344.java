package leetcode_grind;

import java.util.ArrayDeque;
import java.util.Arrays;
import java.util.HashMap;
import java.util.Stack;

public class Day344 {
    // https://leetcode.com/problems/next-greater-element-i/description/
    static class Solution1 {
        public int[] nextGreaterElementBruteForce(int[] nums1, int[] nums2) {
            var res = new int[nums1.length];
            int j;

            for (var i = 0; i < nums1.length; i++) {
                var found = false;
                for (j = 0; j < nums2.length; j++) {
                    if (found && nums2[j] > nums1[i]) {
                        res[i] = nums2[j];
                        break;
                    }

                    if (nums2[j] == nums1[i]) {
                        found = true;
                    }
                }
                if (j == nums2.length) {
                    res[i] = -1;
                }
            }

            return res;
        }

        public int[] nextGreaterElement(int[] nums1, int[] nums2) {
            var hash = new HashMap<Integer, Integer>();
            for (var i = 0; i < nums2.length; i++) {
                hash.put(nums2[i], i);
            }

            var ans = new int[nums1.length];
            int j;

            for (int i = 0; i < nums1.length; i++) {
                for (j = hash.get(nums1[i]) + 1; j < nums2.length; j++) {
                    if (nums1[i] < nums2[j]) {
                        ans[i] = nums2[j];
                        break;
                    }
                }
                if (j == nums2.length) {
                    ans[i] = -1;
                }
            }

            return ans;
        }

        public int[] nextGreaterElementMonotonicStack(int[] nums1, int[] nums2) {
            var idx = new HashMap<Integer, Integer>();
            for (var i = 0; i < nums2.length; i++) {
                idx.put(nums2[i], i);
            }

            var hash = new HashMap<Integer, Integer>();
            var stack = new Stack<Integer>();
            for (var i = 0; i < nums2.length; i++) {
                while (!stack.isEmpty() && nums2[stack.peek()] <= nums2[i]) {
                    hash.put(stack.pop(), nums2[i]);
                }
                stack.push(i);
            }

            var ans = new int[nums1.length];
            Arrays.fill(ans, -1);
            for (var i = 0; i < nums1.length; i++) {
                ans[i] = hash.getOrDefault(idx.get(nums1[i]), -1);
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/sliding-window-maximum/description/
    static class Solution2 {
        public int[] maxSlidingWindow(int[] nums, int k) {
            var deque = new ArrayDeque<Integer>();
            var ans = new int[nums.length - k + 1];
            for (int i = 0; i < nums.length; i++) {
                while (deque.size() > 0 && nums[deque.peekLast()] <= nums[i]) {
                    deque.removeLast();
                }

                deque.addLast(i);

                while (deque.size() > 0 && deque.peekFirst() < i - k + 1) {
                    deque.removeFirst();
                }

                if (i >= k - 1)
                    ans[i - k + 1] = nums[deque.peekFirst()];
            }
            return ans;
        }
    }
}
