package leetcode_grind;

import java.util.ArrayDeque;
import java.util.Deque;

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
}
