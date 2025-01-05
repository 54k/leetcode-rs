package leetcode_grind;

public class Day778 {
    // https://leetcode.com/problems/shifting-letters/description/
    static class Solution1 {
        public String shiftingLetters(String s, int[] shifts) {
            StringBuilder ans = new StringBuilder();
            int X = 0;
            for (int shift : shifts) {
                X = (X + shift) % 26;
            }
            for (int i = 0; i < s.length(); i++) {
                int index = s.charAt(i) - 'a';
                ans.append((char) ((index + X) % 26 + 97));
                X = Math.floorMod(X - shifts[i], 26);
            }
            return ans.toString();
        }
    }

    // https://leetcode.com/problems/shifting-letters-ii/description/?envType=daily-question&envId=2025-01-05
    static class Solution2 {

        public String shiftingLetters(String s, int[][] shifts) {
            int n = s.length();
            int[] diffArray = new int[n]; // Initialize a difference array with all elements set to 0.

            // Process each shift operation
            for (int[] shift : shifts) {
                if (shift[2] == 1) { // If direction is forward (1)
                    diffArray[shift[0]]++; // Increment at the start index
                    if (shift[1] + 1 < n) {
                        diffArray[shift[1] + 1]--; // Decrement at the end+1 index
                    }
                } else { // If direction is backward (0)
                    diffArray[shift[0]]--; // Decrement at the start index
                    if (shift[1] + 1 < n) {
                        diffArray[shift[1] + 1]++; // Increment at the end+1 index
                    }
                }
            }

            StringBuilder result = new StringBuilder(s);
            int numberOfShifts = 0;

            // Apply the shifts to the string
            for (int i = 0; i < n; i++) {
                numberOfShifts = (numberOfShifts + diffArray[i]) % 26; // Update cumulative shifts, keeping within the
                                                                       // alphabet range
                if (numberOfShifts < 0)
                    numberOfShifts += 26; // Ensure non-negative shifts

                // Calculate the new character by shifting `s[i]`
                char shiftedChar = (char) ('a' +
                        ((s.charAt(i) - 'a' + numberOfShifts) % 26));
                result.setCharAt(i, shiftedChar);
            }

            return result.toString();
        }
    }
}