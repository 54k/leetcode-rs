package leetcode_grind;

public class Day484 {
    // https://leetcode.com/problems/custom-sort-string/
    static class Solution1 {
        public String customSortString(String order, String s) {
            int[] freq = new int[26];
            for (var ch : s.toCharArray()) {
                freq[ch - 'a']++;
            }
            var ans = "";
            for (var ch : order.toCharArray()) {
                var i = 0;
                while (freq[ch - 'a'] > i) {
                    ans += ch;
                    i++;
                }
                freq[ch - 'a'] = 0;
            }
            for (var ch : s.toCharArray()) {
                var i = 0;
                while (freq[ch - 'a'] > i) {
                    ans += ch;
                    i++;
                }
                freq[ch - 'a'] = 0;
            }
            return ans;
        }
    }
}
