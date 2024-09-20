package leetcode_grind;

public class Day673 {
    // https://leetcode.com/problems/shortest-palindrome/description/?envType=daily-question&envId=2024-09-20
    static class Solution1 {
        public String shortestPalindrome(String s) {
            if (s.isEmpty())
                return s;
            var isPalindrome = new Object() {
                boolean apply(int i) {
                    int j = 0;
                    int end = i / 2;
                    while (i >= end) {
                        if (s.charAt(i) != s.charAt(j)) {
                            return false;
                        }
                        i--;
                        j++;
                    }
                    return true;
                }
            };
            int n = s.length();
            int max = 0;
            for (int i = 0; i < n; i++) {
                if (isPalindrome.apply(i)) {
                    max = i;
                }
            }
            String rest = s.substring(max + 1);
            var r = new StringBuilder(rest).reverse().toString();
            return r + s;
        }
    }

    static class Solution2 {
        public String shortestPalindrome(String s) {
            int length = s.length();
            String reversedString = new StringBuilder(s).reverse().toString();
            for (int i = 0; i < length; i++) {
                if (s.substring(0, length - i).equals(reversedString.substring(i))) {
                    return new StringBuilder(reversedString.substring(0, i))
                            .append(s)
                            .toString();
                }
            }
            return "";
        }
    }

    static class Solution3 {
        public String shortestPalindrome(String s) {
            int length = s.length();
            if (length == 0) {
                return s;
            }
            int left = 0;
            for (int right = length - 1; right >= 0; right--) {
                if (s.charAt(right) == s.charAt(left)) {
                    left++;
                }
            }
            if (left == length) {
                return s;
            }
            String nonPalindromeSuffix = s.substring(left);
            StringBuilder reverseSuffix = new StringBuilder(nonPalindromeSuffix).reverse();
            return reverseSuffix
                    .append(shortestPalindrome(s.substring(0, left)))
                    .append(nonPalindromeSuffix)
                    .toString();
        }
    }
}
