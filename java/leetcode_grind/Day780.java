package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day780 {
    // https://leetcode.com/problems/string-matching-in-an-array/description/?envType=daily-question&envId=2025-01-07
    static class Solution1 {
        public List<String> stringMatching(String[] words) {
            List<String> matchingWords = new ArrayList<>();

            for (int currentWordIndex = 0; currentWordIndex < words.length; currentWordIndex++) {
                int[] lps = computeLPSArray(words[currentWordIndex]);

                for (int otherWordIndex = 0; otherWordIndex < words.length; otherWordIndex++) {
                    if (currentWordIndex == otherWordIndex)
                        continue;
                    if (isSubstringOf(words[currentWordIndex], words[otherWordIndex], lps)) {
                        matchingWords.add(words[currentWordIndex]);
                        break;
                    }
                }

            }

            return matchingWords;
        }

        int[] computeLPSArray(String sub) {
            int[] lps = new int[sub.length()];
            int currentIndex = 1;
            int len = 0;

            while (currentIndex < sub.length()) {
                if (sub.charAt(currentIndex) == sub.charAt(len)) {
                    len++;
                    lps[currentIndex] = len;
                    currentIndex++;
                } else {
                    if (len > 0) {
                        len = lps[len - 1];
                    } else {
                        currentIndex++;
                    }
                }
            }
            return lps;
        }

        boolean isSubstringOf(String sub, String main, int[] lps) {
            int mainIndex = 0;
            int subIndex = 0;

            while (mainIndex < main.length()) {
                if (main.charAt(mainIndex) == sub.charAt(subIndex)) {
                    mainIndex++;
                    subIndex++;
                    if (subIndex == sub.length())
                        return true;
                } else {
                    if (subIndex > 0) {
                        subIndex = lps[subIndex - 1];
                    } else {
                        mainIndex++;
                    }
                }
            }
            return false;
        }
    }

    static class Solution2 {
        static class TrieNode {
            int frequency;
            Map<Character, TrieNode> childNodes;

            TrieNode() {
                frequency = 0;
                childNodes = new HashMap<>();
            }
        }

        public List<String> stringMatching(String[] words) {
            List<String> matchingWords = new ArrayList<>();
            TrieNode root = new TrieNode();

            for (String word : words) {
                for (int si = 0; si < word.length(); si++) {
                    insertWord(root, word.substring(si));
                }
            }

            for (String word : words) {
                if (isSubstring(root, word)) {
                    matchingWords.add(word);
                }
            }
            return matchingWords;
        }

        void insertWord(TrieNode root, String word) {
            TrieNode currentNode = root;
            for (char c : word.toCharArray()) {
                currentNode.childNodes.putIfAbsent(c, new TrieNode());
                currentNode = currentNode.childNodes.get(c);
                currentNode.frequency++;
            }
        }

        boolean isSubstring(TrieNode root, String word) {
            TrieNode currentNode = root;
            for (char c : word.toCharArray()) {
                currentNode = currentNode.childNodes.get(c);
            }
            return currentNode.frequency > 1;
        }
    }
}
