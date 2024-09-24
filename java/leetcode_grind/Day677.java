package leetcode_grind;

import java.util.*;

public class Day677 {
    // https://leetcode.com/problems/find-the-length-of-the-longest-common-prefix/description/?envType=daily-question&envId=2024-09-24
    static class Solution1 {
        public int longestCommonPrefix(int[] arr1, int[] arr2) {
            var hs1 = new HashSet<String>();
            for (var x : arr1) {
                var s = Integer.toString(x);
                for (int i = 0; i < s.length(); i++) {
                    hs1.add(s.substring(0, i + 1));
                }
            }
            var hs2 = new HashSet<String>();
            for (var x : arr2) {
                hs2.add(Integer.toString(x));
            }
            int ans = 0;
            for (var x : hs2) {
                for (int i = 0; i < x.length(); i++) {
                    if (hs1.contains(x.substring(0, i + 1))) {
                        ans = Math.max(ans, i + 1);
                    }
                }
            }
            return ans;
        }
    }

        static class TrieNode {
            TrieNode[] children = new TrieNode[10];
        }

        static class Trie {
            TrieNode root = new TrieNode();

            void insert(int num) {
                TrieNode node = root;
                String numStr = Integer.toString(num);
                for (char digit : numStr.toCharArray()) {
                    int idx = digit - '0';
                    if (node.children[idx] == null) {
                        node.children[idx] = new TrieNode();
                    }
                    node = node.children[idx];
                }
            }

            int findLongestPrefix(int num) {
                TrieNode node = root;
                String numStr = Integer.toString(num);
                int len = 0;

                for (char digit : numStr.toCharArray()) {
                    int idx = digit - '0';
                    if (node.children[idx] != null) {
                        len++;
                        node = node.children[idx];
                    } else {
                        break;
                    }
                }
                return len;
            }
        }

        public int longestCommonPrefix(int[] arr1, int[] arr2) {
            Trie trie = new Trie();
            for (int num : arr1) {
                trie.insert(num);
            }
            int longestPrefix = 0;
            for (int num : arr2) {
                int len = trie.findLongestPrefix(num);
                longestPrefix = Math.max(longestPrefix, len);
            }
            return longestPrefix;
        }
    }
}
