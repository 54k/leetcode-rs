package leetcode_grind;

public class Day664 {
    // https://leetcode.com/problems/minimum-bit-flips-to-convert-number/description/?envType=daily-question&envId=2024-09-11
    static class Solution1 {
        public int minBitFlips(int start, int goal) {
            int count = 0;
            while (start > 0 || goal > 0) {
                if ((start & 1) != (goal & 1)) {
                    count++;
                }
                start >>= 1;
                goal >>= 1;
            }
            return count;
        }
    }

    static class Solution2 {
        public int minBitFlips(int start, int goal) {
            if (start == 0 && goal == 0)
                return 0;
            int flip = (start & 1) != (goal & 1) ? 1 : 0;
            return flip + minBitFlips(start >> 1, goal >> 1);
        }
    }

    
}
