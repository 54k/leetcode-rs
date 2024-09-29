package leetcode_grind;

import java.util.*;

public class Day682 {
    // https://leetcode.com/problems/all-oone-data-structure/description/?envType=daily-question&envId=2024-09-29
    static class AllOne {

        static class Node {
            int freq;
            Node prev;
            Node next;
            Set<String> keys = new HashSet<>();

            Node(int freq) {
                this.freq = freq;
            }
        }

        Node head;
        Node tail;
        Map<String, Node> map = new HashMap<>();

        public AllOne() {
            head = new Node(0);
            tail = new Node(0);
            head.next = tail;
            tail.prev = head;
        }

        public void inc(String key) {
            if (map.containsKey(key)) {
                Node node = map.get(key);
                int freq = node.freq;
                node.keys.remove(key);

                Node nextNode = node.next;
                if (nextNode == tail || nextNode.freq != freq + 1) {
                    Node newNode = new Node(freq + 1);
                    newNode.keys.add(key);
                    newNode.prev = node;
                    newNode.next = nextNode;
                    node.next = newNode;
                    nextNode.prev = newNode;
                    map.put(key, newNode);
                } else {
                    nextNode.keys.add(key);
                    map.put(key, nextNode);
                }

                if (node.keys.isEmpty()) {
                    removeNode(node);
                }
            } else {
                Node firstNode = head.next;
                if (firstNode == tail || firstNode.freq > 1) {
                    Node newNode = new Node(1);
                    newNode.keys.add(key);
                    newNode.prev = head;
                    newNode.next = firstNode;
                    head.next = newNode;
                    firstNode.prev = newNode;
                    map.put(key, newNode);
                } else {
                    firstNode.keys.add(key);
                    map.put(key, firstNode);
                }
            }
        }

        public void dec(String key) {
            if (!map.containsKey(key)) {
                return;
            }

            Node node = map.get(key);
            node.keys.remove(key);
            int freq = node.freq;

            if (freq == 1) {
                map.remove(key);
            } else {
                Node prevNode = node.prev;
                if (prevNode == head || prevNode.freq != freq - 1) {
                    Node newNode = new Node(freq - 1);
                    newNode.keys.add(key);
                    newNode.prev = prevNode;
                    newNode.next = node;
                    prevNode.next = newNode;
                    node.prev = newNode;
                    map.put(key, newNode);
                } else {
                    prevNode.keys.add(key);
                    map.put(key, prevNode);
                }
            }

            if (node.keys.isEmpty()) {
                removeNode(node);
            }
        }

        public String getMaxKey() {
            if (tail.prev == head) {
                return "";
            }
            return tail.prev.keys.iterator().next();
        }

        public String getMinKey() {
            if (head.next == tail) {
                return "";
            }
            return head.next.keys.iterator().next();
        }

        void removeNode(Node node) {
            Node prevNode = node.prev;
            Node nextNode = node.next;
            prevNode.next = nextNode;
            nextNode.prev = prevNode;
        }
    }

    // https://leetcode.com/problems/implement-trie-ii-prefix-tree/description/?envType=weekly-question&envId=2024-09-29
    static class Trie {

        class TrieNode {
            TrieNode[] links = new TrieNode[26];
            int wordsEndingHere = 0;
            int wordsStartingHere = 0;
        }

        TrieNode root;

        public Trie() {
            root = new TrieNode();
        }

        public void insert(String word) {
            TrieNode node = root;
            for (char w : word.toCharArray()) {
                int charIndex = w - 'a';
                if (node.links[charIndex] == null) {
                    node.links[charIndex] = new TrieNode();
                }
                node = node.links[charIndex];
                node.wordsStartingHere++;
            }
            node.wordsEndingHere++;
        }

        public int countWordsEqualTo(String word) {
            TrieNode node = root;
            for (char w : word.toCharArray()) {
                int charIndex = w - 'a';
                if (node.links[charIndex] == null) {
                    return 0;
                }
                node = node.links[charIndex];
            }
            return node.wordsEndingHere;
        }

        public int countWordsStartingWith(String prefix) {
            TrieNode node = root;
            for (char w : prefix.toCharArray()) {
                int charIndex = w - 'a';
                if (node.links[charIndex] == null) {
                    return 0;
                }
                node = node.links[charIndex];
            }
            return node.wordsStartingHere;
        }

        public void erase(String word) {
            TrieNode node = root;
            for (char w : word.toCharArray()) {
                int charIndex = w - 'a';
                node = node.links[charIndex];
                node.wordsStartingHere--;
            }
            node.wordsEndingHere--;
        }
    }
}
