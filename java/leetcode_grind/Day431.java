package leetcode_grind;

public class Day431 {
    // https://leetcode.com/problems/valid-word-abbreviation/description/
    static class Solution1 {
        public boolean validWordAbbreviation(String word, String abbr) {
            int j = 0, i = 0, cur = 0;
            for (; i < abbr.length(); i++) {
                if (abbr.charAt(i) == '0' && cur == 0) {
                    return false;
                }

                if (Character.isDigit(abbr.charAt(i))) {
                    cur = cur * 10 + (abbr.charAt(i) - '0');
                } else {
                    j += cur;
                    cur = 0;
                    if (j > word.length() - 1) {
                        return false;
                    }
                    if (word.charAt(j) != abbr.charAt(i)) {
                        System.out.printf("%s %s\n", word.charAt(j), abbr.charAt(i));
                        return false;
                    }
                    j++;
                }
            }
            j += cur;
            return j == word.length();
        }
    }
}
