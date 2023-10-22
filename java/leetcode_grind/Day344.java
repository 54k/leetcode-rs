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

    // https://leetcode.com/problems/maximum-score-of-a-good-subarray/description/
    static class Solution3 {
        public int maximumScoreBinarySearch(int[] nums, int k) {
            var binSearch = new Object() {
                int apply(int[] nums, int target) {
                    var lo = 0;
                    var hi = nums.length;
                    while (lo < hi) {
                        var mid = (lo + hi) / 2;
                        if (nums[mid] < target) {
                            lo = mid + 1;
                        } else {
                            hi = mid;
                        }
                    }
                    return lo;
                }
            };

            var solve = new Object() {
                int apply(int[] nums, int k) {
                    var left = new int[k];
                    var currMin = Integer.MAX_VALUE;
                    for (int i = k - 1; i >= 0; i--) {
                        currMin = Math.min(currMin, nums[i]);
                        left[i] = currMin;
                    }

                    var right = new int[nums.length - k];
                    currMin = Integer.MAX_VALUE;
                    for (int i = 0; i < right.length; i++) {
                        currMin = Math.min(currMin, nums[i + k]);
                        right[i] = currMin;
                    }

                    var ans = 0;
                    for (int j = 0; j < right.length; j++) {
                        currMin = right[j];
                        var i = binSearch.apply(left, currMin);
                        ans = Math.max(ans, currMin * (k + j - i + 1));
                    }
                    return ans;
                }
            };

            var ans = solve.apply(nums, k);
            for (int i = 0; i < nums.length / 2; i++) {
                var temp = nums[i];
                nums[i] = nums[nums.length - 1 - i];
                nums[nums.length - 1 - i] = temp;
            }
            return Math.max(ans, solve.apply(nums, nums.length - k - 1));
        }

        public int maximumScoreMonotonicStack(int[] nums, int k) {
            var n = nums.length;
            var left = new int[n];
            Arrays.fill(left, -1);
            var stack = new Stack<Integer>();

            for (int i = n - 1; i >= 0; i--) {
                while (!stack.isEmpty() && nums[stack.peek()] > nums[i]) {
                    left[stack.pop()] = i;
                }
                stack.push(i);
            }

            stack.clear();
            var right = new int[n];
            Arrays.fill(right, n);

            for (int i = 0; i < n; i++) {
                while (!stack.isEmpty() && nums[stack.peek()] > nums[i]) {
                    right[stack.pop()] = i;
                }
                stack.push(i);
            }

            var ans = 0;
            for (int i = 0; i < n; i++) {
                if (left[i] < k && right[i] > k) {
                    ans = Math.max(nums[i] * (right[i] - left[i] - 1), ans);
                }
            }
            return ans;
        }

        public int maximumScoreGreedy(int[] nums, int k) {
            var n = nums.length;
            var ans = nums[k];
            var currMin = nums[k];
            var left = k;
            var right = k;
            while (left > 0 || right < n - 1) {
                if ((left > 0 ? nums[left - 1] : 0) < (right < n - 1 ? nums[right + 1] : 0)) {
                    right++;
                    currMin = Math.min(currMin, nums[right]);
                } else {
                    left--;
                    currMin = Math.min(currMin, nums[left]);
                }
                ans = Math.max(ans, currMin * (right - left + 1));
            }
            return ans;
        }
    }
}
