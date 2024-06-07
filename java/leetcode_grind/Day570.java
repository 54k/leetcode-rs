package leetcode_grind;

import java.util.List;

public class Day570 {
    static class TrieNode {
        boolean isEnd;
        TrieNode[] children;

        TrieNode() {
            isEnd = false;
            children = new TrieNode[26];
        }
    }

    static class Trie {
        private TrieNode root;

        Trie() {
            root = new TrieNode();
        }

        void insert(String word) {
            TrieNode current = root;
            for (char c : word.toCharArray()) {
                if (current.children[c - 'a'] == null) {
                    current.children[c - 'a'] = new TrieNode();
                }
                current = current.children[c - 'a'];
            }
            current.isEnd = true;
        }

        String shortestRoot(String word) {
            TrieNode current = root;
            for (int i = 0; i < word.length(); i++) {
                char c = word.charAt(i);
                if (current.children[c - 'a'] == null) {
                    return word;
                }
                current = current.children[c - 'a'];
                if (current.isEnd) {
                    return word.substring(0, i + 1);
                }
            }
            return word;
        }
    }

    static class Solution {
        public String replaceWords(List<String> dictionary, String sentence) {
            String[] wordArray = sentence.split(" ");
            Trie dictTrie = new Trie();
            for (var word : dictionary) {
                dictTrie.insert(word);
            }

            for (int word = 0; word < wordArray.length; word++) {
                wordArray[word] = dictTrie.shortestRoot(wordArray[word]);
            }
            return String.join(" ", wordArray);
        }
    }
}
