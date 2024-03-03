package leetcode_grind;

public class Day476 {
    // https://leetcode.com/problems/remove-nth-node-from-end-of-list/
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
        public ListNode removeNthFromEnd(ListNode head, int n) {
            var dummy = new ListNode(-1, head);
            var fast = dummy.next;
            var slow = dummy;
            int i = 0;
            while (fast != null) {
                if (i >= n) {
                    slow = slow.next;
                }
                i++;
                fast = fast.next;
            }
            slow.next = slow.next.next;
            return dummy.next;
        }
    }
}
