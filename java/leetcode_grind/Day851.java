package leetcode_grind;

import java.util.ArrayDeque;
import java.util.Deque;
import java.util.LinkedList;

public class Day851 {
    // https://leetcode.com/problems/minimum-operations-to-make-binary-array-elements-equal-to-one-i/description/?envType=daily-question&envId=2025-03-19
    static class Solution1 {
        public int minOperations(int[] nums) {
            int ops = 0;
            for (int i = 0; i < nums.length; i++) {
                if (nums[i] == 0) {
                    if (i < nums.length - 2) {
                        nums[i] ^= 1;
                        nums[i + 1] ^= 1;
                        nums[i + 2] ^= 1;
                        ops++;
                    } else {
                        return -1;
                    }
                }
            }
            return ops;
        }
    }

    // https://leetcode.com/problems/minimum-operations-to-make-binary-array-elements-equal-to-one-i/description/?envType=daily-question&envId=2025-03-19
    static class Solution2 {
        public int minOperations(int[] nums) {
            Deque<Integer> flipQueue = new ArrayDeque<>();
            int count = 0;

            for (int i = 0; i < nums.length; i++) {
                while (!flipQueue.isEmpty() && i > flipQueue.peekFirst() + 2) {
                    flipQueue.pollFirst();
                }

                if ((nums[i] + flipQueue.size()) % 2 == 0) {
                    if (i + 2 >= nums.length) {
                        return -1;
                    }
                    count++;
                    flipQueue.offerLast(i);
                }
            }

            return count;
        }
    }

    // https://leetcode.com/problems/minimum-number-of-k-consecutive-bit-flips/description/
    static class Solution3 {
        public int minKBitFlips(int[] nums, int k) {
            boolean[] flipped = new boolean[nums.length];
            int validFlipsFromPastWindow = 0;
            int flipCount = 0;

            for (int i = 0; i < nums.length; i++) {
                if (i >= k) {
                    if (flipped[i - k]) {
                        validFlipsFromPastWindow--;
                    }
                }

                if (validFlipsFromPastWindow % 2 == nums[i]) {
                    if (i + k > nums.length) {
                        return -1;
                    }
                    validFlipsFromPastWindow++;
                    flipped[i] = true;
                    flipCount++;
                }
            }

            return flipCount;
        }
    }

    static class Solution4 {
        public int minKBitFlips(int[] nums, int k) {
            int n = nums.length;
            Deque<Integer> flipQueue = new LinkedList<>();

            int flipped = 0;
            int result = 0;

            for (int i = 0; i < n; i++) {
                if (i >= k) {
                    flipped ^= flipQueue.poll();
                }
                if (flipped == nums[i]) {
                    if (i + k > n) {
                        return -1;
                    }

                    flipQueue.offer(1);
                    flipped ^= 1;
                    result++;
                } else {
                    flipQueue.offer(0);
                }
            }
            return result;
        }
    }

    static class Solution5 {
        public int minKBitFlips(int[] nums, int k) {
            int currentFlips = 0;
            int totalFlips = 0;

            for (int i = 0; i < nums.length; i++) {
                if (i >= k && nums[i - k] == 2) {
                    currentFlips--;
                }

                if ((currentFlips % 2) == nums[i]) {
                    if (i + k > nums.length) {
                        return -1;
                    }
                    nums[i] = 2;
                    currentFlips++;
                    totalFlips++;
                }
            }
            return totalFlips;
        }
    }
}
