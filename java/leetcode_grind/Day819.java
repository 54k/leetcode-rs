package leetcode_grind;

public class Day819 {
    // https://leetcode.com/problems/find-the-punishment-number-of-an-integer/description/?envType=daily-question&envId=2025-02-15
    static class Solution1 {
        public boolean canPartition(String stringNum, int target) {
            // Valid Partition Found
            if (stringNum.isEmpty() && target == 0) {
                return true;
            }

            // Invalid Partition Found
            if (target < 0) {
                return false;
            }

            // Recursively check all partitions for a valid partition
            for (int index = 0; index < stringNum.length(); index++) {
                String left = stringNum.substring(0, index + 1);
                String right = stringNum.substring(index + 1);
                int leftNum = Integer.parseInt(left);

                if (canPartition(right, target - leftNum)) {
                    return true;
                }
            }
            return false;
        }

        public int punishmentNumber(int n) {
            int punishmentNum = 0;

            // Iterate through numbers in range [1, n]
            for (int currentNum = 1; currentNum <= n; currentNum++) {
                int squareNum = currentNum * currentNum;

                // Check if valid partition can be found and add squared number if so
                if (canPartition(Integer.toString(squareNum), currentNum)) {
                    punishmentNum += squareNum;
                }
            }
            return punishmentNum;
        }
    }
}
