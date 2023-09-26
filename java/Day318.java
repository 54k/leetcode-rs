import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.Map;
import java.util.Stack;
import java.util.stream.Collectors;

public class Day318 {
    // https://leetcode.com/problems/remove-duplicate-letters/description
    class Solution {
        public String removeDuplicateLettersGreedy(String s) {
            var cnt = new int[26];
            for (int i = 0; i < s.length(); i++) {
                cnt[s.charAt(i) - 'a']++;
            }

            var pos = 0;
            for (int i = 0; i < s.length(); i++) {
                if (s.charAt(i) < s.charAt(pos)) {
                    pos = i;
                }
                if (--cnt[s.charAt(i) - 'a'] == 0) {
                    break;
                }
            }

            if (s.length() == 0) {
                return "";
            }
            return s.charAt(pos)
                    + removeDuplicateLettersGreedy(s.substring(pos + 1).replaceAll("" + s.charAt(pos), ""));
        }

        public String removeDuplicateLettersWithStack(String s) {
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

    // https://leetcode.com/problems/design-add-and-search-words-data-structure/description/
    static class WordDictionary {
        static class TrieNode {
            Map<Character, TrieNode> children = new HashMap<>();
            boolean word;
        }

        TrieNode root;

        public WordDictionary() {
            root = new TrieNode();
        }

        public void addWord(String word) {
            var cur = root;
            for (var ch : word.toCharArray()) {
                cur.children.putIfAbsent(ch, new TrieNode());
                cur = cur.children.get(ch);
            }
            cur.word = true;
        }

        public boolean search(String word) {
            var curStack = new LinkedList<TrieNode>();
            curStack.push(root);

            for (var ch : word.toCharArray()) {
                var nextStack = new LinkedList<TrieNode>();
                while (curStack.size() > 0) {
                    var cur = curStack.pop();
                    if (ch == '.') {
                        for (var next : cur.children.values()) {
                            nextStack.push(next);
                        }
                    } else {
                        if (cur.children.containsKey(ch)) {
                            nextStack.push(cur.children.get(ch));
                        }
                    }
                }
                curStack = nextStack;
            }

            while (curStack.size() > 0) {
                if (curStack.pop().word) {
                    return true;
                }
            }

            return false;
        }
    }
}
