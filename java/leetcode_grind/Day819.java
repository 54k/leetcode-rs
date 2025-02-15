package leetcode_grind;

import java.util.Arrays;

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

    static class Solution2 {
        boolean findPartitions(
                int startIndex,
                int sum,
                String stringNum,
                int target,
                int[][] memo) {
            if (startIndex == stringNum.length()) {
                return sum == target;
            }

            if (sum > target)
                return false;

            if (memo[startIndex][sum] != -1)
                return memo[startIndex][sum] == 1;

            boolean partitionFound = false;
            for (int currentIndex = startIndex; currentIndex < stringNum.length(); currentIndex++) {
                String currentString = stringNum.substring(startIndex, currentIndex + 1);
                int addend = Integer.parseInt(currentString);
                partitionFound |= findPartitions(currentIndex + 1, sum + addend, stringNum, target, memo);
                if (partitionFound) {
                    memo[startIndex][sum] = 1;
                    return true;
                }
            }
            memo[startIndex][sum] = 0;
            return false;
        }

        public int punishmentNumber(int n) {
            int punishmentNum = 0;
            for (int currentNum = 1; currentNum <= n; currentNum++) {
                int squareNum = currentNum * currentNum;
                String stringNum = Integer.toString(squareNum);
                int[][] memoArray = new int[stringNum.length()][currentNum + 1];
                for (int[] row : memoArray) {
                    Arrays.fill(row, -1);
                }
                if (findPartitions(0, 0, stringNum, currentNum, memoArray)) {
                    punishmentNum += squareNum;
                }
            }
            return punishmentNum;
        }
    }

    static class Solution3 {
        boolean canPartition(int num, int target) {
            if (target < 0 || num < target) {
                return false;
            }
            if (num == target) {
                return true;
            }
            return (canPartition(num / 10, target - (num % 10)) ||
                    canPartition(num / 100, target - (num % 100)) ||
                    canPartition(num / 1000, target - (num % 1000)));
        }

        public int punishmentNumber(int n) {
            int result = 0;
            for (int curr = 1; curr <= n; curr++) {
                int square = curr * curr;
                if (canPartition(square, curr)) {
                    result += square;
                }
            }
            return result;
        }
    }
}
