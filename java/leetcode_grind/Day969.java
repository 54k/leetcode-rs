package leetcode_grind;

public class Day969 {
    // https://leetcode.com/problems/valid-word/description/?envType=daily-question&envId=2025-07-15
    static class Solution1 {
        public boolean isValid(String word) {
            if (word.length() < 3) {
                return false;
            }
            boolean hasVowel = false;
            boolean hasConsonant = false;
            for (char c : word.toCharArray()) {
                if (Character.isLetter(c)) {
                    char ch = Character.toLowerCase(c);
                    if (ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u') {
                        hasVowel = true;
                    } else {
                        hasConsonant = true;
                    }
                } else if (!Character.isDigit(c)) {
                    return false;
                }
            }
            return hasVowel && hasConsonant;
        }
    }

}
