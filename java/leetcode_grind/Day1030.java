package leetcode_grind;

import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;

public class Day1030 {

    // https://leetcode.com/problems/vowel-spellchecker/description/?envType=daily-question&envId=2025-09-14
    static class Solution1 {
        Set<String> words_perfect;
        Map<String, String> words_cap;
        Map<String, String> words_vow;

        public String[] spellchecker(String[] wordlist, String[] queries) {
            words_perfect = new HashSet<>();
            words_cap = new HashMap<>();
            words_vow = new HashMap<>();

            for (String word : wordlist) {
                words_perfect.add(word);
                String wordlow = word.toLowerCase();
                words_cap.putIfAbsent(wordlow, word);
                String wordlowDV = devowel(wordlow);
                words_vow.putIfAbsent(wordlowDV, word);
            }

            String[] ans = new String[queries.length];
            int t = 0;
            for (String query : queries) {
                ans[t++] = solve(query);
            }
            return ans;
        }

        String solve(String query) {
            if (words_perfect.contains(query)) {
                return query;
            }
            String queryL = query.toLowerCase();
            if (words_cap.containsKey(queryL)) {
                return words_cap.get(queryL);
            }
            String queryLV = devowel(queryL);
            if (words_vow.containsKey(queryLV)) {
                return words_vow.get(queryLV);
            }
            return "";
        }

        String devowel(String word) {
            StringBuilder ans = new StringBuilder();
            for (char c : word.toCharArray()) {
                ans.append(isVowel(c) ? '*' : c);
            }
            return ans.toString();
        }

        boolean isVowel(char c) {
            return (c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u');
        }
    }

}
