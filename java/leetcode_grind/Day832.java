package leetcode_grind;

public class Day832 {
    // https://leetcode.com/problems/shortest-common-supersequence/description/?envType=daily-question&envId=2025-02-28
    static class Solution1 {
        public String shortestCommonSupersequence(String str1, String str2) {
            int str1Length = str1.length();
            int str2Length = str2.length();

            String[] prevRow = new String[str2Length + 1];
            for (int col = 0; col <= str2Length; col++) {
                prevRow[col] = str2.substring(0, col);
            }

            for (int row = 1; row <= str1Length; row++) {
                String[] currRow = new String[str2Length + 1];
                currRow[0] = str1.substring(0, row);

                for (int col = 1; col <= str2Length; col++) {
                    if (str1.charAt(row - 1) == str2.charAt(col - 1)) {
                        currRow[col] = prevRow[col - 1] + str1.charAt(row - 1);
                    } else {
                        String pickS1 = prevRow[col];
                        String pickS2 = currRow[col - 1];

                        currRow[col] = (pickS1.length() < pickS2.length()) ? pickS1 + str1.charAt(row - 1)
                                : pickS2 + str2.charAt(col - 1);
                    }
                }

                prevRow = currRow;
            }

            return prevRow[str2Length];
        }
    }
}
