package leetcode_grind;

import java.util.HashMap;

public class Day481 {
    // https://leetcode.com/problems/linked-list-frequency/description
    public static class ListNode {
        int val;
        ListNode next;

        ListNode() {
        }

        ListNode(int val) {
            this.val = val;
        }

        ListNode(int val, ListNode next) {
            this.val = val;
            this.next = next;
        }
    }

    static class Solution1 {
        public ListNode frequenciesOfElements(ListNode head) {
            var freq = new HashMap<Integer, ListNode>();
            ListNode current = head;
            ListNode freqHead = null;

            while (current != null) {
                if (freq.containsKey(current.val)) {
                    var node = freq.get(current.val);
                    node.val++;
                } else {
                    freq.put(current.val, new ListNode(1, freqHead));
                    freqHead = freq.get(current.val);
                }
                current = current.next;
            }
            return freqHead;
        }
    }

}
