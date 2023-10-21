package leetcode_grind;

import java.util.ArrayDeque;
import java.util.Deque;
import java.util.PriorityQueue;
import java.util.TreeMap;

public class Day343 {
    // https://leetcode.com/problems/constrained-subsequence-sum/description
    static class Solution {
        public int constrainedSubsetSumHeap(int[] nums, int k) {
            var heap = new PriorityQueue<int[]>((a, b) -> {
                return b[0] - a[0];
            });

            heap.add(new int[] { nums[0], 0 });
            var ans = nums[0];

            for (var i = 1; i < nums.length; i++) {
                while (i - heap.peek()[1] > k) {
                    heap.remove();
                }

                var curr = Math.max(0, heap.peek()[0]) + nums[i];
                ans = Math.max(ans, curr);
                heap.add(new int[] { curr, i });
            }

            return ans;
        }

        public int constrainedSubsetSumTreeMap(int[] nums, int k) {
            TreeMap<Integer, Integer> window = new TreeMap<>();
            window.put(0, 0);

            int[] dp = new int[nums.length];

            for (int i = 0; i < nums.length; i++) {
                dp[i] = nums[i] + window.lastKey();
                window.put(dp[i], window.getOrDefault(dp[i], 0) + 1);

                if (i >= k) {
                    window.put(dp[i - k], window.get(dp[i - k]) - 1);
                    if (window.get(dp[i - k]) == 0) {
                        window.remove(dp[i - k]);
                    }
                }
            }

            int ans = Integer.MIN_VALUE;
            for (int num : dp) {
                ans = Math.max(ans, num);
            }
            return ans;
        }

        public int constrainedSubsetSumDeque(int[] nums, int k) {
            Deque<Integer> deque = new ArrayDeque<>();
            var dp = new int[nums.length];

            for (int i = 0; i < nums.length; i++) {
                if (!deque.isEmpty() && i - deque.peek() > k) {
                    deque.poll();
                }

                dp[i] = (deque.isEmpty() ? 0 : dp[deque.peek()]) + nums[i];

                while (!deque.isEmpty() && dp[deque.peekLast()] < dp[i]) {
                    deque.pollLast();
                }

                if (dp[i] > 0) {
                    deque.offer(i);
                }
            }

            var ans = Integer.MIN_VALUE;
            for (var v : dp) {
                ans = Math.max(ans, v);
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/maximum-subarray/description/
    static class Solution2 {
        public int maxSubArray(int[] nums) {
            var maxSubArray = nums[0];
            var currentSubarray = nums[0];

            for (int i = 1; i < nums.length; i++) {
                int num = nums[i];
                currentSubarray = Math.max(num, currentSubarray + num);
                maxSubArray = Math.max(maxSubArray, currentSubarray);
            }

            return maxSubArray;
        }
    }

    // https://leetcode.com/problems/remove-element/description/
    static class Solution3 {
        public int removeElementForward(int[] nums, int val) {
            var left = 0;
            for (var right = 0; right < nums.length; right++) {
                if (nums[right] == val) {
                    continue;
                }
                nums[left++] = nums[right];
            }
            return left;
        }

        public int removeElementBackwards(int[] nums, int val) {
            var i = 0;
            var n = nums.length;
            while (i < n) {
                if (nums[i] == val) {
                    nums[i] = nums[n - 1];
                    n--;
                } else {
                    i++;
                }
            }
            return i;
        }
    }

    // https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/
    static class Solution4 {
        public int removeDuplicates(int[] nums) {
            var left = 0;
            for (var right = 1; right < nums.length; right++) {
                if (nums[left] != nums[right]) {
                    nums[++left] = nums[right];
                }
            }
            return ++left;
        }

        public int removeDuplicatesLeetcode(int[] nums) {
            var insertIndex = 1;
            for (int i = 1; i < nums.length; i++) {
                if (nums[i - 1] != nums[i]) {
                    nums[insertIndex++] = nums[i];
                }
            }
            return insertIndex;
        }
    }

    // https://leetcode.com/problems/move-zeroes/description/
    static class Solution5 {
        public void moveZeroes(int[] nums) {
            var left = 0;
            for (var right = 0; right < nums.length; right++) {
                if (nums[right] != 0) {
                    var tmp = nums[right];
                    nums[right] = nums[left];
                    nums[left] = tmp;
                    left++;
                }
            }
        }
    }
}
