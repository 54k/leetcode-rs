package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day809 {
    // https://leetcode.com/problems/check-if-one-string-swap-can-make-strings-equal/
    static class Solution1 {
        public boolean areAlmostEqual(String s1, String s2) {
            int firstIndexDiff = 0;
            int secondIndexDiff = 0;
            int numDiffs = 0;
            for (int i = 0; i < s1.length(); i++) {
                if (s1.charAt(i) != s2.charAt(i)) {
                    numDiffs++;

                    if (numDiffs > 2)
                        return false;
                    else if (numDiffs == 1) {
                        firstIndexDiff = i;
                    } else {
                        secondIndexDiff = i;
                    }
                }
            }

            return (s1.charAt(firstIndexDiff) == s2.charAt(secondIndexDiff) &&
                    s1.charAt(secondIndexDiff) == s2.charAt(firstIndexDiff));
        }
    }

    static class Solution {
        public int minimumLengthEncoding(String[] words) {
            TrieNode trie = new TrieNode();
            Map<TrieNode, Integer> nodes = new HashMap<>();

            for (int i = 0; i < words.length; i++) {
                String word = words[i];
                TrieNode cur = trie;
                for (int j = word.length() - 1; j >= 0; j--) {
                    cur = cur.get(word.charAt(j));
                }
                nodes.put(cur, i);
            }
            int ans = 0;
            for (TrieNode node : nodes.keySet()) {
                if (node.count == 0) {
                    ans += words[nodes.get(node)].length() + 1;
                }
            }
            return ans;
        }

        class TrieNode {
            TrieNode[] children;
            int count;

            TrieNode() {
                children = new TrieNode[26];
                count = 0;
            }

            TrieNode get(char c) {
                if (children[c - 'a'] == null) {
                    children[c - 'a'] = new TrieNode();
                    count++;
                }
                return children[c - 'a'];
            }
        }
    }
}
