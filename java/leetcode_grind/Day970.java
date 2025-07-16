package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;

public class Day970 {
    // https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-i/description/?envType=daily-question&envId=2025-07-16
    static class Solution1 {
        public int maximumLength(int[] nums) {
            int res = 0;
            int[][] patterns = { { 0, 0 }, { 0, 1 }, { 1, 0 }, { 1, 1 } };
            for (int[] pattern : patterns) {
                int cnt = 0;
                for (int num : nums) {
                    if (num % 2 == pattern[cnt % 2]) {
                        cnt++;
                    }
                }
                res = Math.max(res, cnt);
            }
            return res;
        }
    }

    // https://leetcode.com/problems/word-squares/description/?envType=weekly-question&envId=2025-07-15
    static class Solution2 {
        int N = 0;
        String[] words = null;
        HashMap<String, List<String>> prefixHashTable = null;

        public List<List<String>> wordSquares(String[] words) {
            this.words = words;
            this.N = words[0].length();

            List<List<String>> results = new ArrayList<List<String>>();
            this.buildPrefixHashTable(words);

            for (String word : words) {
                LinkedList<String> wordSquares = new LinkedList<String>();
                wordSquares.addLast(word);
                this.backtracking(1, wordSquares, results);
            }

            return results;
        }

        void backtracking(int step, LinkedList<String> wordSquares, List<List<String>> results) {
            if (step == N) {
                results.add((List<String>) wordSquares.clone());
                return;
            }

            StringBuilder prefix = new StringBuilder();
            for (String word : wordSquares) {
                prefix.append(word.charAt(step));
            }

            for (String candidate : this.getWordsWithPrefix(prefix.toString())) {
                wordSquares.addLast(candidate);
                this.backtracking(step + 1, wordSquares, results);
                wordSquares.removeLast();
            }
        }

        void buildPrefixHashTable(String[] words) {
            this.prefixHashTable = new HashMap<String, List<String>>();
            for (String word : words) {
                for (int i = 1; i < this.N; i++) {
                    String prefix = word.substring(0, i);
                    List<String> wordList = this.prefixHashTable.get(prefix);
                    if (wordList == null) {
                        wordList = new ArrayList<String>();
                        wordList.add(word);
                        this.prefixHashTable.put(prefix, wordList);
                    } else {
                        wordList.add(word);
                    }
                }
            }
        }

        List<String> getWordsWithPrefix(String prefix) {
            List<String> wordList = this.prefixHashTable.get(prefix);
            return (wordList != null ? wordList : new ArrayList<String>());
        }
    }

    static class Solution3 {
        static class TrieNode {
            HashMap<Character, TrieNode> children = new HashMap<>();
            List<Integer> wordList = new ArrayList<>();
        }

        int N = 0;
        String[] words = null;
        TrieNode trie = null;

        public List<List<String>> wordSquares(String[] words) {
            this.words = words;
            this.N = words[0].length();

            List<List<String>> results = new ArrayList<List<String>>();
            this.buildTrie(words);

            for (String word : words) {
                LinkedList<String> wordSquares = new LinkedList<String>();
                wordSquares.addLast(word);
                this.backtracking(1, wordSquares, results);
            }

            return results;
        }

        void backtracking(int step, LinkedList<String> wordSquares, List<List<String>> results) {
            if (step == N) {
                results.add((List<String>) wordSquares.clone());
                return;
            }

            StringBuilder prefix = new StringBuilder();
            for (String word : wordSquares) {
                prefix.append(word.charAt(step));
            }

            for (Integer wordIndex : this.getWordsWithPrefix(prefix.toString())) {
                wordSquares.addLast(this.words[wordIndex]);
                this.backtracking(step + 1, wordSquares, results);
                wordSquares.removeLast();
            }
        }

        void buildTrie(String[] words) {
            this.trie = new TrieNode();

            for (int wordIndex = 0; wordIndex < words.length; ++wordIndex) {
                String word = words[wordIndex];

                TrieNode node = this.trie;
                for (Character letter : word.toCharArray()) {
                    if (node.children.containsKey(letter)) {
                        node = node.children.get(letter);
                    } else {
                        TrieNode newNode = new TrieNode();
                        node.children.put(letter, newNode);
                        node = newNode;
                    }
                    node.wordList.add(wordIndex);
                }
            }
        }

        List<Integer> getWordsWithPrefix(String prefix) {
            TrieNode node = this.trie;
            for (Character letter : prefix.toCharArray()) {
                if (node.children.containsKey(letter)) {
                    node = node.children.get(letter);
                } else {
                    return new ArrayList<>();
                }
            }
            return node.wordList;
        }
    }
}
