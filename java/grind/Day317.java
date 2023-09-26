package grind;
import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;

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

    // https://leetcode.com/problems/design-search-autocomplete-system/description/
    static class AutocompleteSystem {
        static class TrieNode {
            Map<Character, TrieNode> children = new HashMap<Character, TrieNode>();
            Map<String, Integer> sentences = new HashMap<String, Integer>();
        }

        TrieNode root = new TrieNode();
        TrieNode current = root;
        TrieNode dead = new TrieNode();
        StringBuilder currentSentence = new StringBuilder();

        public AutocompleteSystem(String[] sentences, int[] times) {
            for (var i = 0; i < times.length; i++) {
                add(sentences[i], times[i]);
            }
        }

        public List<String> input(char c) {
            if (c == '#') {
                add(currentSentence.toString(), 1);
                currentSentence.setLength(0);
                current = root;
                return Collections.emptyList();
            }

            currentSentence.append(c);
            if (!current.children.containsKey(c)) {
                current = dead;
                return Collections.emptyList();
            }

            var result = new ArrayList<String>();
            current = current.children.get(c);

            var pq = new PriorityQueue<String>(
                    (String o1, String o2) -> {
                        var a = current.sentences.get(o1);
                        var b = current.sentences.get(o2);
                        if (a == b) {
                            return o2.compareTo(o1);
                        }
                        return a - b;
                    });

            for (var k : current.sentences.keySet()) {
                pq.add(k);
                if (pq.size() > 3) {
                    pq.poll();
                }
            }

            while (pq.size() > 0) {
                result.add(pq.poll());
            }

            Collections.reverse(result);
            return result;
        }

        void add(String sentence, Integer count) {
            var node = root;
            for (var ch : sentence.toCharArray()) {
                node.children.putIfAbsent(ch, new TrieNode());
                node = node.children.get(ch);
                node.sentences.put(sentence, node.sentences.getOrDefault(sentence, 0) + count);
            }
        }
    }
}
