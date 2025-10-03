package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day1048 {
    // https://leetcode.com/problems/water-bottles-ii/description/?envType=daily-question&envId=2025-10-02
    static class Solution1 {
        public int maxBottlesDrunk(int numBottles, int numExchange) {
            int ans = 0;
            while (numBottles >= numExchange) {
                ans += numExchange;
                numBottles -= numExchange;
                numBottles++;
                numExchange++;
            }
            return ans + numBottles;
        }
    }

    static class Solution2 {
        public int maxBottlesDrunk(int numBottles, int numExchange) {
            int ans = numBottles;
            for (int empty = numBottles; empty >= numExchange; numExchange++) {
                ans++;
                empty -= numExchange - 1;
            }
            return ans;
        }
    }

    static class Solution3 {
        public int maxBottlesDrunk(int numBottles, int numExchange) {
            int t = 0;
            int empty = t * numExchange + (t * (t - 1)) / 2;
            int total = numBottles + t;
            int a = 1;
            int b = 2 * numExchange - 3;
            int c = -2 * numBottles;

            t = (int) Math.ceil(((-b + Math.sqrt(b * b - 4 * a * c)) / (2 * a)));
            return numBottles + t - 1;
        }
    }

    // https://leetcode.com/problems/lru-cache/description/
    static class LRUCache1 {
        static class ListNode {
            int key;
            int val;
            ListNode next;
            ListNode prev;

            ListNode(int key, int val) {
                this.key = key;
                this.val = val;
            }
        }

        int capacity;
        Map<Integer, ListNode> dic;
        ListNode head;
        ListNode tail;

        public LRUCache1(int capacity) {
            this.capacity = capacity;
            dic = new HashMap<>();
            head = new ListNode(-1, -1);
            tail = new ListNode(-1, -1);
            head.next = tail;
            tail.prev = head;
        }

        public int get(int key) {
            if (!dic.containsKey(key)) {
                return -1;
            }
            ListNode node = dic.get(key);
            remove(node);
            add(node);
            return node.val;
        }

        public void put(int key, int value) {
            if (dic.containsKey(key)) {
                ListNode oldNode = dic.get(key);
                remove(oldNode);
            }

            ListNode node = new ListNode(key, value);
            dic.put(key, node);
            add(node);

            if (dic.size() > capacity) {
                ListNode nodeToDelete = head.next;
                remove(nodeToDelete);
                dic.remove(nodeToDelete.key);
            }
        }

        void add(ListNode node) {
            ListNode previousEnd = tail.prev;
            previousEnd.next = node;
            node.prev = previousEnd;
            node.next = tail;
            tail.prev = node;
        }

        void remove(ListNode node) {
            node.prev.next = node.next;
            node.next.prev = node.prev;
        }
    }

    /**
     * Your LRUCache object will be instantiated and called as such:
     * LRUCache obj = new LRUCache(capacity);
     * int param_1 = obj.get(key);
     * obj.put(key,value);
     */

}
