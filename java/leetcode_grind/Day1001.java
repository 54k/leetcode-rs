package leetcode_grind;

public class Day1001 {
    // https://leetcode.com/problems/maximum-69-number/description/?envType=daily-question&envId=2025-08-16
    static class Solution1 {
        public int maximum69Number(int num) {
            int numCopy = num;
            int indexSix = -1;
            int currDigit = 0;

            while (numCopy > 0) {
                if (numCopy % 10 == 6) {
                    indexSix = currDigit;
                }
                numCopy /= 10;
                currDigit++;
            }

            return indexSix == -1 ? num : num + 3 * (int) Math.pow(10, indexSix);
        }
    }
}