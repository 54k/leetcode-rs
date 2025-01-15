package leetcode_grind;

public class Day788 {
    // https://leetcode.com/problems/minimize-xor/description/?envType=daily-question&envId=2025-01-15
    static class Solution1 {
        public int minimizeXor(int num1, int num2) {
            int result = 0;

            int targetSetBitsCount = Integer.bitCount(num2);
            int setBitsCount = 0;
            int currentBit = 31;

            while (setBitsCount < targetSetBitsCount) {
                if (isSet(num1, currentBit) || (targetSetBitsCount - setBitsCount > currentBit)) {
                    result = setBit(result, currentBit);
                    setBitsCount++;
                }
                currentBit--;
            }

            return result;
        }

        boolean isSet(int x, int bit) {
            return (x & (1 << bit)) != 0;
        }

        int setBit(int x, int bit) {
            return x | (1 << bit);
        }
    }

    static class Solution2 {
        public int minimizeXor(int num1, int num2) {
            int result = num1;
            int targetSetBitsCount = Integer.bitCount(num2);
            int setBitsCount = Integer.bitCount(result);

            int currentBit = 0;

            while (setBitsCount < targetSetBitsCount) {
                if (!isSet(result, currentBit)) {
                    result = setBit(result, currentBit);
                    setBitsCount++;
                }
                currentBit++;
            }

            while (setBitsCount > targetSetBitsCount) {
                if (isSet(result, currentBit)) {
                    result = unsetBit(result, currentBit);
                    setBitsCount--;
                }
                currentBit++;
            }

            return result;
        }

        boolean isSet(int x, int bit) {
            return (x & (1 << bit)) != 0;
        }

        int setBit(int x, int bit) {
            return x | (1 << bit);
        }

        int unsetBit(int x, int bit) {
            return x & ~(1 << bit);
        }
    }

}
