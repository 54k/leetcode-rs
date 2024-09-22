package leetcode_grind;

import java.util.*;

public class Day675 {
    // https://leetcode.com/problems/longest-word-with-all-prefixes/description/?envType=weekly-question&envId=2024-09-22
    static class Solution1 {
        public String longestWord(String[] words) {
            Arrays.sort(words);
            Set<String> validWords = new HashSet<>();
            String longestValidWord = "";

            for (String currentWord : words) {
                if (currentWord.length() == 1
                        || validWords.contains(currentWord.substring(0, currentWord.length() - 1))) {
                    validWords.add(currentWord);

                    if (longestValidWord.length() < currentWord.length()) {
                        longestValidWord = currentWord;
                    }
                }
            }

            return longestValidWord;
        }
    }

    static class Solution2 {
        public String longestWord(String[] words) {
            Trie trie = new Trie();
            String longestValidWord = "";

            for (String word : words) {
                trie.insert(word);
            }

            for (String word : words) {
                if (trie.hasAllPrefixes(word)) {
                    if (word.length() > longestValidWord.length() ||
                            (word.length() == longestValidWord.length() && word.compareTo(longestValidWord) < 0)) {
                        longestValidWord = word;
                    }
                }
            }
            return longestValidWord;
        }

        static class Trie {
            static class TrieNode {
                TrieNode[] children = new TrieNode[26];
                boolean isEndOfWord;
            }

            TrieNode root = new TrieNode();

            void insert(String word) {
                TrieNode node = root;
                for (char c : word.toCharArray()) {
                    int index = c - 'a';
                    if (node.children[index] == null) {
                        node.children[index] = new TrieNode();
                    }
                    node = node.children[index];
                }
                node.isEndOfWord = true;
            }

            boolean hasAllPrefixes(String word) {
                TrieNode node = root;
                for (char c : word.toCharArray()) {
                    node = node.children[c - 'a'];
                    if (node == null || !node.isEndOfWord) {
                        return false;
                    }
                }
                return true;
            }
        }
    }

    // https://leetcode.com/problems/longest-word-in-dictionary/description/
    static class Solution3 {
        public String longestWord(String[] words) {
            String ans = "";
            Set<String> wordset = new HashSet<>();
            for (String word : words) {
                wordset.add(word);
            }
            for (String word : words) {
                if (word.length() > ans.length() || word.length() == ans.length() && word.compareTo(ans) < 0) {
                    boolean good = true;
                    for (int k = 1; k < word.length(); ++k) {
                        if (!wordset.contains(word.substring(0, k))) {
                            good = false;
                            break;
                        }
                    }
                    if (good) {
                        ans = word;
                    }
                }
            }
            return ans;
        }
    }
}
