import java.util.HashSet;
import java.util.Stack;
import java.util.stream.Collectors;

public class Day318 {
    class Solution {
        // https://leetcode.com/problems/remove-duplicate-letters/description
        public String removeDuplicateLetters(String s) {
            var freq = new char[26];
            for (var ch : s.toCharArray()) {
                freq[ch - 'a']++;
            }

            var onStack = new HashSet<Character>();
            var stack = new Stack<Character>();

            for (var ch : s.toCharArray()) {
                if (!onStack.contains(ch)) {
                    while (!stack.isEmpty() && freq[stack.get(stack.size() - 1) - 'a'] > 0
                            && stack.get(stack.size() - 1) >= ch) {
                        var c = stack.pop();
                        onStack.remove(c);
                    }
                    stack.push(ch);
                    onStack.add(ch);
                }
                freq[ch - 'a']--;
            }

            return stack.stream().map(ch -> Character.toString(ch)).collect(Collectors.joining());
        }
    }
}
