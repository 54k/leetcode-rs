package leetcode_grind;

import java.util.Stack;

public class Day930 {
    // https://leetcode.com/problems/using-a-robot-to-print-the-lexicographically-smallest-string/description/?envType=daily-question&envId=2025-06-06
    static class Solution1 {
        public String robotWithString(String s) {
            int[] cnt = new int[26];
            for (char c : s.toCharArray()) {
                cnt[c - 'a']++;
            }

            Stack<Character> stack = new Stack<>();
            StringBuilder res = new StringBuilder();
            char minCharacter = 'a';
            for (char c : s.toCharArray()) {
                stack.push(c);
                cnt[c - 'a']--;
                while (minCharacter != 'z' && cnt[minCharacter - 'a'] == 0) {
                    minCharacter++;
                }
                while (!stack.isEmpty() && stack.peek() <= minCharacter) {
                    res.append(stack.pop());
                }
            }
            return res.toString();
        }
    }
}
