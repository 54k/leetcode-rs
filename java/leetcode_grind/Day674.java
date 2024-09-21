package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day674 {
    // https://leetcode.com/problems/lexicographical-numbers/description/?envType=daily-question&envId=2024-09-21
    static class Solution1 {
        public List<Integer> lexicalOrder(int n) {
            var res = new ArrayList<Integer>();
            var dfs = new Object() {
                void apply(int num) {
                    if (num > n)
                        return;
                    res.add(num);
                    for (int i = 0; i < 10; i++) {
                        apply(num * 10 + i);
                    }
                }
            };
            for (int i = 1; i < 10; i++) {
                dfs.apply(i);
            }
            return res;
        }
    }

    // https://leetcode.com/problems/shortest-palindrome/description/?envType=daily-question&envId=2024-09-20
    static class Solution2 {
        public String shortestPalindrome(String s) {
            if (s == null || s.length() == 0) {
                return s;
            }

            String modifiedString = preprocessString(s);
            int[] palindromeRadiusArray = new int[modifiedString.length()];
            int center = 0, rightBoundary = 0;
            int maxPalindromeLength = 0;

            for (int i = 1; i < modifiedString.length() - 1; i++) {
                int mirrorIndex = 2 * center - i;
                if (rightBoundary > i) {
                    palindromeRadiusArray[i] = Math.min(rightBoundary - i, palindromeRadiusArray[mirrorIndex]);
                }
                while (modifiedString.charAt(i + 1 + palindromeRadiusArray[i]) == modifiedString
                        .charAt(i - 1 - palindromeRadiusArray[i])) {
                    palindromeRadiusArray[i]++;
                }
                if (i + palindromeRadiusArray[i] > rightBoundary) {
                    center = i;
                    rightBoundary = i + palindromeRadiusArray[i];
                }

                if (i - palindromeRadiusArray[i] == 1) {
                    maxPalindromeLength = Math.max(maxPalindromeLength, palindromeRadiusArray[i]);
                }
            }

            StringBuilder suffix = new StringBuilder(s.substring(maxPalindromeLength)).reverse();
            return suffix.append(s).toString();
        }

        String preprocessString(String s) {
            StringBuilder sb = new StringBuilder("^");
            for (char c : s.toCharArray()) {
                sb.append("#").append(c);
            }
            return sb.append("#$").toString();
        }
    }
}
