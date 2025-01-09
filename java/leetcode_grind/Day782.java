package leetcode_grind;

import java.util.HashSet;
import java.util.Set;

public class Day782 {
    // https://leetcode.com/problems/counting-words-with-a-given-prefix/description/?envType=daily-question&envId=2025-01-09
    static class Solution1 {
        public int prefixCount(String[] words, String pref) {
            int count = 0;
            for (String word : words) {
                count += hasPrefix(word, pref);
            }
            return count;
        }

        int hasPrefix(String str, String pref) {
            int itr;
            for (itr = 0; itr < str.length() && itr < pref.length(); itr++) {
                if (str.charAt(itr) != pref.charAt(itr)) {
                    return 0;
                }
            }

            if (itr != pref.length()) {
                return 0;
            }
            return 1;
        }
    }

    static class Solution2 {
        public int prefixCount(String[] words, String pref) {
            int count = 0;
            for (var word : words) {
                if (word.startsWith(pref)) {
                    count++;
                }
            }
            return count;
        }
    }

    static class Solution3 {
        public int prefixCount(String[] words, String pref) {
            Trie trie = new Trie();
            for (String word : words) {
                trie.addWord(word);
            }
            return trie.countPrefix(pref);
        }

        static class Trie {
            static class Node {
                Node[] links;
                int count;

                Node() {
                    links = new Node[26];
                    count = 0;
                }
            }

            Node root;

            Trie() {
                root = new Node();
            }

            void addWord(String word) {
                Node curr = root;
                for (int i = 0; i < word.length(); i++) {
                    char c = word.charAt(i);
                    if (curr.links[c - 'a'] == null) {
                        curr.links[c - 'a'] = new Node();
                    }
                    curr = curr.links[c - 'a'];
                    curr.count++;
                }
            }

            int countPrefix(String pref) {
                Node curr = root;
                for (int i = 0; i < pref.length(); i++) {
                    char c = pref.charAt(i);
                    if (curr.links[c - 'a'] == null) {
                        return 0;
                    }
                    curr = curr.links[c - 'a'];
                }
                return curr.count;
            }
        }
    }

    // https://leetcode.com/problems/unique-substrings-with-equal-digit-frequency/description/?envType=weekly-question&envId=2025-01-08
    static class Solution4 {
        public int equalDigitFrequency(String s) {
            int n = s.length();
            Set<String> validSubstrings = new HashSet<>();
            for (int start = 0; start < n; start++) {
                int[] digitFrequency = new int[10];

                for (int end = start; end < n; end++) {
                    digitFrequency[s.charAt(end) - '0']++;

                    int commonFrequency = 0;
                    boolean isValid = true;

                    for (int i = 0; i < digitFrequency.length; i++) {
                        if (digitFrequency[i] == 0)
                            continue;

                        if (commonFrequency == 0) {
                            commonFrequency = digitFrequency[i];
                        }

                        if (commonFrequency != digitFrequency[i]) {
                            isValid = false;
                            break;
                        }
                    }

                    if (isValid) {
                        String subString = s.substring(start, end + 1);
                        validSubstrings.add(subString);
                    }
                }
            }
            return validSubstrings.size();
        }
    }

    static class Solution5 {
        public int equalDigitFrequency(String s) {
            int n = s.length();
            int prime = 31;
            long mod = 1000000000L;
            Set<Long> validSubstringHashes = new HashSet<>();

            for (int start = 0; start < n; start++) {
                int[] digitFrequency = new int[10];
                int uniqueDigitCount = 0;
                long substringHash = 0;
                int maxDigitFrequency = 0;

                for (int end = start; end < n; end++) {
                    int currentDigit = s.charAt(end) - '0';

                    if (digitFrequency[currentDigit] == 0) {
                        uniqueDigitCount++;
                    }

                    digitFrequency[currentDigit]++;
                    maxDigitFrequency = Math.max(maxDigitFrequency, digitFrequency[currentDigit]);

                    substringHash = (prime * substringHash + currentDigit + 1) % mod;

                    if (maxDigitFrequency * uniqueDigitCount == end - start + 1) {
                        validSubstringHashes.add(substringHash);
                    }
                }
            }
            return validSubstringHashes.size();
        }
    }

    static class Solution6 {
        static class TrieNode {
            TrieNode[] children = new TrieNode[10];
            boolean isVisited;
        }

        public int equalDigitFrequency(String s) {
            TrieNode root = new TrieNode();
            int totalValidSubstrings = 0;

            for (int start = 0; start < s.length(); start++) {
                TrieNode currentNode = root;
                int[] digitFrequency = new int[10];
                int uniqueDigitsCount = 0;
                int maxDigitFrequency = 0;

                for (int end = start; end < s.length(); end++) {
                    int currentDigit = s.charAt(end) - '0';
                    if (digitFrequency[currentDigit]++ == 0) {
                        uniqueDigitsCount++;
                    }
                    maxDigitFrequency = Math.max(maxDigitFrequency, digitFrequency[currentDigit]);
                    if (currentNode.children[currentDigit] == null) {
                        currentNode.children[currentDigit] = new TrieNode();
                    }
                    currentNode = currentNode.children[currentDigit];

                    if (uniqueDigitsCount * maxDigitFrequency == end - start + 1 && !currentNode.isVisited) {
                        totalValidSubstrings++;
                        currentNode.isVisited = true;
                    }
                }
            }
            return totalValidSubstrings;
        }
    }
}
