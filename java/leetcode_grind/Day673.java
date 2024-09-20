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

}
