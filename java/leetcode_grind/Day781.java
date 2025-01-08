package leetcode_grind;

public class Day781 {
    // https://leetcode.com/problems/count-prefix-and-suffix-pairs-i/description/?envType=daily-question&envId=2025-01-08
    static class Solution1 {
        public int countPrefixSuffixPairs(String[] words) {
            int n = words.length;
            int count = 0;
            for (int i = 0; i < n; i++) {
                for (int j = i + 1; j < n; j++) {
                    String str1 = words[i];
                    String str2 = words[j];

                    if (str1.length() > str2.length())
                        continue;

                    if (str2.startsWith(str1) && str2.endsWith(str1)) {
                        count++;
                    }
                }
            }
            return count;
        }
    }

    static class Solution2 {
        static class Node {
            Node[] links = new Node[26];

            boolean contains(char c) {
                return links[c - 'a'] != null;
            }

            void put(char c, Node node) {
                links[c - 'a'] = node;
            }

            Node next(char c) {
                return links[c - 'a'];
            }
        }

        static class Trie {
            Node root;

            Trie() {
                root = new Node();
            }

            void insert(String word) {
                Node node = root;
                for (char c : word.toCharArray()) {
                    if (!node.contains(c)) {
                        node.put(c, new Node());
                    }
                    node = node.next(c);
                }
            }

            boolean startsWith(String prefix) {
                Node node = root;
                for (char c : prefix.toCharArray()) {
                    if (!node.contains(c)) {
                        return false;
                    }
                    node = node.next(c);
                }
                return true;
            }
        }

        public int countPrefixSuffixPairs(String[] words) {
            int n = words.length;
            int count = 0;
            for (int i = 0; i < n; i++) {
                Trie prefixTrie = new Trie();
                Trie suffixTrie = new Trie();

                prefixTrie.insert(words[i]);

                String revWord = new StringBuilder(words[i]).reverse().toString();

                suffixTrie.insert(revWord);

                for (int j = 0; j < i; j++) {
                    if (words[j].length() > words[i].length())
                        continue;

                    String prefixWord = words[j];
                    String revPrefixWord = new StringBuilder(prefixWord).reverse().toString();

                    if (prefixTrie.startsWith(prefixWord) && suffixTrie.startsWith(revPrefixWord)) {
                        count++;
                    }
                }
            }
            return count;
        }
    }

}
