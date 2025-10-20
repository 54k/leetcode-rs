package leetcode_grind;

public class Day1066 {
    // https://leetcode.com/problems/final-value-of-variable-after-performing-operations/description/?envType=daily-question&envId=2025-10-20
    static class Solution1 {
        public int finalValueAfterOperations(String[] operations) {
            int ret = 0;
            for (String op : operations) {
                if (op.contains("-")) {
                    ret--;
                } else {
                    ret++;
                }
            }
            return ret;
        }
    }
}
