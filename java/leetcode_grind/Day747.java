public class Day747 {
    // https://leetcode.com/problems/make-string-a-subsequence-using-cyclic-increments/description/?envType=daily-question&envId=2024-12-04
    static class Solution1 {

        public boolean canMakeSubsequence(String str1, String str2) {
            int lengthStr1 = str1.length();

            // Try all possible combinations of character increments
            for (int mask = 0; mask < (1 << lengthStr1); mask++) {
                StringBuilder temp = new StringBuilder(str1);

                // Apply increments based on current mask
                for (int str1Index = 0; str1Index < lengthStr1; str1Index++) {
                    if ((mask & (1 << str1Index)) != 0) {
                        temp.setCharAt(
                                str1Index,
                                getNextChar(temp.charAt(str1Index)));
                    }
                }

                // Check if str2 is a subsequence of the modified string
                if (isSubsequence(temp.toString(), str2)) {
                    return true;
                }
            }

            return false;
        }

        // Helper function to get the next character cyclically
        private char getNextChar(char str1Char) {
            return str1Char == 'z' ? 'a' : (char) (str1Char + 1);
        }

        // Helper function to check if str2 is a subsequence of str1
        private boolean isSubsequence(String str1, String str2) {
            int str1Index = 0, str2Index = 0;
            int lengthStr1 = str1.length(), lengthStr2 = str2.length();

            // Traverse through both strings using a while loop
            while (str1Index < lengthStr1 && str2Index < lengthStr2) {
                if (str1.charAt(str1Index) == str2.charAt(str2Index)) {
                    str2Index++;
                }
                str1Index++;
            }
            // Check if all characters in str2 were matched
            return str2Index == lengthStr2;
        }
    }

    static class Solution2 {
        public boolean canMakeSubsequence(String str1, String str2) {
            int str2Index = 0;
            int lengthStr1 = str1.length(), lengthStr2 = str2.length();

            for (int str1Index = 0; str1Index < lengthStr1 && str2Index < lengthStr2; ++str1Index) {
                if (str1.charAt(str1Index) == str2.charAt(str2Index) ||
                        (str1.charAt(str1Index) + 1 == str2.charAt(str2Index)) ||
                        (str1.charAt(str1Index) - 25 == str2.charAt(str2Index))) {
                    str2Index++;
                }
            }
            return str2Index == lengthStr2;
        }
    }
}
