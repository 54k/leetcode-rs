package leetcode_grind;

public class Day540 {

    // https://leetcode.com/problems/remove-nodes-from-linked-list/description
    public class ListNode {
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
        ListNode reverseList(ListNode head) {
            ListNode prev = null;
            ListNode current = head;
            ListNode nextTemp = null;
            while (current != null) {
                nextTemp = current.next;
                current.next = prev;
                prev = current;
                current = nextTemp;
            }
            return prev;
        }

        public ListNode removeNodes(ListNode head) {
            head = reverseList(head);
            int maximum = 0;
            ListNode prev = null;
            ListNode current = head;
            while (current != null) {
                maximum = Math.max(maximum, current.val);
                if (current.val < maximum) {
                    prev.next = current.next;
                    ListNode deleted = current;
                    current = current.next;
                    deleted.next = null;
                } else {
                    prev = current;
                    current = current.next;
                }
            }
            return reverseList(head);
        }
    }

}
