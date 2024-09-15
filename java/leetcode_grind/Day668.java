package leetcode_grind;

public class Day668 {
    // https://leetcode.com/problems/find-the-longest-substring-containing-vowels-in-even-counts/description/?envType=daily-question&envId=2024-09-15
    static class Solution1 {
        public int findTheLongestSubstring(String s) {
            int prefixXOR = 0;
            int[] charMap = new int[26];
            charMap['a' - 'a'] = 1;
            charMap['e' - 'a'] = 2;
            charMap['i' - 'a'] = 4;
            charMap['o' - 'a'] = 8;
            charMap['u' - 'a'] = 16;
            int[] mp = new int[32];
            for (int i = 0; i < 32; i++)
                mp[i] = -1;
            int longestSubstring = 0;
            for (int i = 0; i < s.length(); i++) {
                prefixXOR ^= charMap[s.charAt(i) - 'a'];
                if (mp[prefixXOR] == -1 && prefixXOR != 0)
                    mp[prefixXOR] = i;
                longestSubstring = Math.max(longestSubstring, i - mp[prefixXOR]);
            }
            return longestSubstring;
        }
    }
}
