package leetcode_grind;

public class Day1086 {
    // https://leetcode.com/problems/count-operations-to-obtain-zero/description/?envType=daily-question&envId=2025-11-09
    static class Solution1 {
        public int countOperations(int num1, int num2) {
            int res = 0;
            while (num1 != 0 && num2 != 0) {
                res += num1 / num2;
                num1 %= num2;
                int temp = num1;
                num1 = num2;
                num2 = temp;
            }
            return res;
        }
    }
}