package leetcode_grind;

public class Day535 {
    // https://leetcode.com/problems/reverse-prefix-of-word/description
    static class Solution1 {
        public String reversePrefix(String word, char ch) {
            char[] result = word.toCharArray();
            int left = 0;
            var swap = new Object() {
                void apply(int left, int right) {
                    char t = result[left];
                    result[left] = result[right];
                    result[right] = t;
                }
            };
            for (int right = 0; right < word.length(); right++) {
                if (result[right] == ch) {
                    while (left <= right) {
                        swap.apply(left, right);
                        left++;
                        right--;
                    }
                    return new String(result);
                }
            }
            return word;
        }
    }

    // https://leetcode.com/problems/valid-palindrome-ii/description/
    static class Solution2 {
        public boolean validPalindrome(String s) {
            var check = new Object() {
                boolean apply(int i, int j) {
                    while (i < j) {
                        if (s.charAt(i) != s.charAt(j)) {
                            return false;
                        }
                        i++;
                        j--;
                    }
                    return true;
                }
            };

            int i = 0, j = s.length() - 1;
            while (i < j) {
                if (s.charAt(i) != s.charAt(j)) {
                    return check.apply(i + 1, j) || check.apply(i, j - 1);
                }
                i++;
                j--;
            }
            return true;
        }
    }
}
