import java.util.HashMap;
import java.util.Map;

public class Day317 {
    // https://leetcode.com/problems/find-the-difference/description
    static class Solution {
        public char findTheDifference(String s, String t) {
            char ch = 0;
            for (var c : s.toCharArray()) {
                ch ^= c;
            }
            for (var c : t.toCharArray()) {
                ch ^= c;
            }
            return ch;
        }
    }

    static class Trie {
        static class Node {
            Map<Character, Node> children = new HashMap<Character, Node>();
            boolean isEnd;
        }

        Node root;

        public Trie() {
            root = new Node();
        }

        public void insert(String word) {
            var root = this.root;
            for (var ch : word.toCharArray()) {
                if (!root.children.containsKey(ch)) {
                    root.children.put(ch, new Node());
                }
                root = root.children.get(ch);
            }
            root.isEnd = true;
        }

        public boolean search(String word) {
            var root = this.root;
            for (var ch : word.toCharArray()) {
                if (!root.children.containsKey(ch)) {
                    return false;
                }
                root = root.children.get(ch);
            }
            return root.isEnd;
        }

        public boolean startsWith(String prefix) {
            var root = this.root;
            for (var ch : prefix.toCharArray()) {
                if (!root.children.containsKey(ch)) {
                    return false;
                }
                root = root.children.get(ch);
            }
            return true;
        }
    }
}
