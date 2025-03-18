package leetcode_grind;

public class Day850 {
    // https://leetcode.com/problems/longest-nice-subarray/description/?envType=daily-question&envId=2025-03-18
    static class Solution1 {
        public int longestNiceSubarray(int[] nums) {
            int left = 0, right = nums.length;
            int result = 1;

            while (left <= right) {
                int length = left + (right - left) / 2;
                if (canFormNiceSubarray(length, nums)) {
                    result = length;
                    left = length + 1;
                } else {
                    right = length - 1;
                }
            }

            return result;
        }

        boolean canFormNiceSubarray(int length, int[] nums) {
            if (length <= 1)
                return true;

            for (int start = 0; start <= nums.length - length; ++start) {
                int bitMask = 0;
                boolean isNice = true;

                for (int pos = start; pos < start + length; ++pos) {
                    if ((bitMask & nums[pos]) != 0) {
                        isNice = false;
                        break;
                    }
                    bitMask |= nums[pos];
                }
                if (isNice)
                    return true;
            }
            return false;
        }
    }

    static class Solution2 {
        public int longestNiceSubarray(int[] nums) {
            int usedBits = 0;
            int windowStart = 0;
            int maxLength = 0;

            for (int windowEnd = 0; windowEnd < nums.length; ++windowEnd) {
                while ((usedBits & nums[windowEnd]) != 0) {
                    usedBits ^= nums[windowStart++];
                }
                usedBits |= nums[windowEnd];
                maxLength = Math.max(maxLength, windowEnd - windowStart + 1);
            }

            return maxLength;
        }
    }
}
