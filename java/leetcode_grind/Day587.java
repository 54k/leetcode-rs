package leetcode_grind;

import java.util.ArrayDeque;

public class Day587 {
    // https://leetcode.com/problems/minimum-number-of-k-consecutive-bit-flips/description/
    static class Solution {
        public int minKBitFlips(int[] nums, int k) {
            var deq = new ArrayDeque<Integer>();
            var flips = 0;
            for (int index = 0; index < nums.length; index++) {
                if (!deq.isEmpty() && deq.peekFirst() < index)
                    deq.removeFirst();

                if ((nums[index] + deq.size()) % 2 == 0) {
                    if (index + k - 1 >= nums.length)
                        return -1;
                    deq.addLast(index + k - 1);
                    flips++;
                }
            }
            return flips;
        }
    }
}
