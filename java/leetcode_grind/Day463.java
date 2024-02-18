package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day463 {
    // https://leetcode.com/problems/find-all-anagrams-in-a-string/description/
    static class Solution1 {
        public List<Integer> findAnagrams(String s, String p) {
            var res = new ArrayList<Integer>();
            if (s.length() < p.length()) {
                return res;
            }

            s += "@";
            int[] m1 = new int[26];
            int[] m2 = new int[26];
            for (int i = 0; i < p.length(); i++) {
                m1[p.charAt(i) - 'a']++;
                m2[s.charAt(i) - 'a']++;
            }
            int count = 0;
            for (int i = 0; i < 26; i++) {
                if (m1[i] == m2[i]) {
                    count++;
                }
            }

            for (int i = 0; i < s.length() - p.length(); i++) {
                int l = s.charAt(i) - 'a';
                int r = s.charAt(i + p.length()) - 'a';

                if (count == 26) {
                    res.add(i);
                }

                if (s.charAt(i + p.length()) == '@') {
                    break;
                }

                m2[r]++;
                if (m2[r] == m1[r]) {
                    count++;
                } else if (m2[r] == m1[r] + 1) {
                    count--;
                }

                m2[l]--;
                if (m2[l] == m1[l]) {
                    count++;
                } else if (m2[l] + 1 == m1[l]) {
                    count--;
                }
            }

            return res;
        }
    }
}
