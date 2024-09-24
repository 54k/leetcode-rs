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

    static class Solution2 {
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

    // https://leetcode.com/problems/longest-common-suffix-queries/description/
    static class Solution3 {
        static class Pair {
            Integer key;
            String value;

            Pair(Integer k, String v) {
                key = k;
                value = v;
            }

            Integer getKey() {
                return key;
            }

            String getValue() {
                return value;
            }

            public String toString() {
                return String.format("[%s, %s]", key, value);
            }
        }

        static class TrieNode {
            TrieNode[] children = new TrieNode[26];
            int idx = -1;
        }

        TrieNode root;

        void insert(Integer idx, String s) {
            var cur = root;
            for (char c : s.toCharArray()) {
                if (cur.children[c - 'a'] == null) {
                    cur.children[c - 'a'] = new TrieNode();
                    cur.children[c - 'a'].idx = idx;
                }
                cur = cur.children[c - 'a'];
            }
        }

        int longestPrefix(String s) {
            var ans = -1;
            var cur = root;
            for (char c : s.toCharArray()) {
                if (cur.children[c - 'a'] != null) {
                    ans = cur.children[c - 'a'].idx;
                    cur = cur.children[c - 'a'];
                } else {
                    break;
                }
            }
            return ans;
        }

        public int[] stringIndices(String[] wordsContainer, String[] wordsQuery) {
            var ans = new int[wordsQuery.length];
            root = new TrieNode();
            Pair[] vc = new Pair[wordsContainer.length];
            for (int i = 0; i < wordsContainer.length; i++) {
                var w = new StringBuilder(wordsContainer[i]).reverse().toString();
                vc[i] = new Pair(i, w);
            }
            Arrays.sort(vc, (a, b) -> {
                if (a.getValue().length() == b.getValue().length()) {
                    return a.getKey().compareTo(b.getKey());
                }
                return Integer.compare(a.getValue().length(), b.getValue().length());
            });
            // System.out.println(Arrays.toString(vc));
            for (var p : vc) {
                insert(p.getKey(), p.getValue());
            }
            for (int i = 0; i < wordsQuery.length; i++) {
                var w = new StringBuilder(wordsQuery[i]).reverse().toString();
                ans[i] = longestPrefix(w);
                if (ans[i] == -1) {
                    ans[i] = vc[0].getKey();
                }
            }
            return ans;
        }
    }

    static class Solution4 {
        static class TrieNode {
            TrieNode[] next = new TrieNode[26];
            int idx = -1;
            int val = -1;
        }

        public int[] stringIndices(String[] wordsContainer, String[] wordsQuery) {
            TrieNode trie = new TrieNode();
            for (int i = 0; i < wordsContainer.length; i++) {
                String word = new StringBuilder(wordsContainer[i]).reverse().toString();
                TrieNode node = trie;
                if (node.val == -1 || word.length() < node.val) {
                    node.idx = i;
                    node.val = word.length();
                }
                for (var ch : word.toCharArray()) {
                    if (node.next[ch - 'a'] == null) {
                        node.next[ch - 'a'] = new TrieNode();
                    }
                    node = node.next[ch - 'a'];
                    if (node.val == -1 || word.length() < node.val) {
                        node.idx = i;
                        node.val = word.length();
                    }
                }
            }

            int n = wordsQuery.length;
            int[] ans = new int[n];
            for (int i = 0; i < n; i++) {
                String word = new StringBuilder(wordsQuery[i]).reverse().toString();
                TrieNode node = trie;
                for (char ch : word.toCharArray()) {
                    if (node.next[ch - 'a'] == null)
                        break;
                    node = node.next[ch - 'a'];
                }
                ans[i] = node.idx;
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/longest-word-in-dictionary-through-deleting/description/
    static class Solution5 {
        public String findLongestWord(String s, List<String> dictionary) {
            var set = new HashSet<String>(dictionary);
            var l = new ArrayList<String>();
            generate(s, "", 0, l);
            var max_str = "";
            for (String str : l) {
                if (set.contains(str)) {
                    if (str.length() > max_str.length()
                            || (str.length() == max_str.length() && str.compareTo(max_str) < 0)) {
                        max_str = str;
                    }
                }
            }
            return max_str;
        }

        void generate(String s, String str, int i, List<String> l) {
            if (i == s.length()) {
                l.add(str);
            } else {
                generate(s, str + s.charAt(i), i + 1, l);
                generate(s, str, i + 1, l);
            }
        }
    }

    static class Solution6 {
        public String findLongestWord(String s, List<String> dictionary) {
            String max_str = "";
            var set = new HashSet<String>(dictionary);
            List<String> l = new ArrayList<>();
            for (int i = 0; i < (1 << s.length()); i++) {
                String str = "";
                for (int j = 0; j < s.length(); j++) {
                    if (((i >> j) & 1) != 0) {
                        str += s.charAt(j);
                    }
                }
                if (set.contains(str)) {
                    if (str.length() > max_str.length()
                            || (str.length() == max_str.length() && str.compareTo(max_str) < 0)) {
                        max_str = str;
                    }
                }
            }
            return max_str;
        }
    }

    static class Solution7 {
        boolean isSubsequence(String x, String y) {
            int j = 0;
            for (int i = 0; i < y.length() && j < x.length(); i++) {
                if (x.charAt(j) == y.charAt(i)) {
                    j++;
                }
            }
            return j == x.length();
        }

        public String findLongestWord(String s, List<String> dictionary) {
            Collections.sort(dictionary, (s1, s2) -> {
                return s2.length() != s1.length() ? s2.length() - s1.length() : s1.compareTo(s2);
            });
            for (var str : dictionary) {
                if (isSubsequence(str, s)) {
                    return str;
                }
            }
            return "";
        }
    }

    
}
