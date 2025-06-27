package leetcode_grind;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;
import java.util.Queue;

public class Day951 {
    // https://leetcode.com/problems/longest-subsequence-repeated-k-times/description/?envType=daily-question&envId=2025-06-27
    static class Solution1 {
        public String longestSubsequenceRepeatedK(String s, int k) {
            int[] freq = new int[26];
            for (char ch : s.toCharArray()) {
                freq[ch - 'a']++;
            }

            List<Character> candidate = new ArrayList<>();
            for (int i = 25; i >= 0; i--) {
                if (freq[i] >= k) {
                    candidate.add((char) ('a' + i));
                }
            }

            Queue<String> q = new LinkedList<>();
            for (char ch : candidate) {
                q.add(String.valueOf(ch));
            }

            String ans = "";
            while (!q.isEmpty()) {
                String curr = q.poll();
                if (curr.length() > ans.length()) {
                    ans = curr;
                }
                for (char ch : candidate) {
                    String next = curr + ch;
                    if (isKRepeatedSubsequence(s, next, k)) {
                        q.add(next);
                    }
                }
            }
            return ans;
        }

        public boolean isKRepeatedSubsequence(String s, String t, int k) {
            int pos = 0, matched = 0;
            int m = t.length();
            for (char ch : s.toCharArray()) {
                if (ch == t.charAt(pos)) {
                    pos++;
                    if (pos == m) {
                        pos = 0;
                        matched++;
                        if (matched == k) {
                            return true;
                        }
                    }
                }
            }
            return false;
        }
    }
}
