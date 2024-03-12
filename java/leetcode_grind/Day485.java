package leetcode_grind;

import java.util.HashMap;

public class Day485 {
    // https://leetcode.com/problems/remove-zero-sum-consecutive-nodes-from-linked-list/description
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
        public ListNode removeZeroSumSublists(ListNode head) {
            var front = new ListNode(0, head);

            var map = new HashMap<Integer, ListNode>();
            var prefixSum = 0;
            var cur = front;

            while (cur != null) {
                prefixSum += cur.val;
                if (map.containsKey(prefixSum)) {
                    var prev = map.get(prefixSum);
                    cur = prev.next;
                    int p = prefixSum + cur.val;
                    while (p != prefixSum) {
                        map.remove(p);
                        cur = cur.next;
                        p += cur.val;
                    }
                    prev.next = cur.next;
                } else {
                    map.put(prefixSum, cur);
                }
                cur = cur.next;
            }

            return front.next;
        }
    }
}
