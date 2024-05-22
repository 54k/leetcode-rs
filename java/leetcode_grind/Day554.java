package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class Day554 {
    // https://leetcode.com/problems/palindrome-partitioning/description/
    static class Solution1 {
        public List<List<String>> partition(String s) {
            int len = s.length();
            boolean[][] dp = new boolean[len][len];
            List<List<String>> result = new ArrayList<>();
            dfs(result, s, 0, new ArrayList<>(), dp);
            return result;
        }

        void dfs(List<List<String>> result, String s, int start, List<String> currentList, boolean[][] dp) {
            if (start >= s.length())
                result.add(new ArrayList<>(currentList));
            for (int end = start; end < s.length(); end++) {
                if (s.charAt(start) == s.charAt(end) && (end - start <= 2 || dp[start + 1][end - 1])) {
                    dp[start][end] = true;
                    currentList.add(s.substring(start, end + 1));
                    dfs(result, s, end + 1, currentList, dp);
                    currentList.remove(currentList.size() - 1);
                }
            }
        }
    }

    // https://leetcode.com/problems/shortest-palindrome/description/
    static class Solution2 {
        public String shortestPalindrome(String s) {
            int n = s.length();
            String rev = new StringBuilder(s).reverse().toString();
            String s_new = s + '#' + rev;
            int n_new = s_new.length();
            int[] f = new int[n_new];
            for (int i = 1; i < n_new; i++) {
                int t = f[i - 1];
                while (t > 0 && s_new.charAt(i) != s_new.charAt(t)) {
                    t = f[t - 1];
                }
                if (s_new.charAt(i) == s_new.charAt(t)) {
                    t++;
                }
                f[i] = t;
            }
            return rev.substring(0, n - f[n_new - 1]) + s;
        }
    }

    // https://leetcode.com/problems/word-pattern-ii/description/
    static class Solution {
        public boolean wordPatternMatch(String pattern, String s) {
            String[] symbols = new String[26];
            Arrays.fill(symbols, "");
            Set<String> wordSet = new HashSet<>();

            return isMatch(s, 0, pattern, 0, symbols, wordSet);
        }

        boolean isMatch(String s, int sIndex, String pattern, int pIndex, String[] symbols, Set<String> wordSet) {
            if (pIndex == pattern.length()) {
                return sIndex == s.length();
            }

            char symbol = pattern.charAt(pIndex);

            if (!symbols[symbol - 'a'].equals("")) {
                String word = symbols[symbol - 'a'];
                if (!s.startsWith(word, sIndex)) {
                    return false;
                }
                return isMatch(s, sIndex + word.length(), pattern, pIndex + 1, symbols, wordSet);
            }
            int filledSpots = 0;
            for (int i = pIndex + 1; i < pattern.length(); i++) {
                char p = pattern.charAt(i);
                filledSpots += symbols[p - 'a'].equals("") ? 1 : symbols[p - 'a'].length();
            }

            for (int k = sIndex + 1; k <= s.length() - filledSpots; k++) {
                String newWord = s.substring(sIndex, k);
                if (wordSet.contains(newWord)) {
                    continue;
                }
                symbols[symbol - 'a'] = newWord;
                wordSet.add(newWord);
                if (isMatch(s, k, pattern, pIndex + 1, symbols, wordSet)) {
                    return true;
                }
                symbols[symbol - 'a'] = "";
                wordSet.remove(newWord);
            }
            return false;
        }
    }
}
