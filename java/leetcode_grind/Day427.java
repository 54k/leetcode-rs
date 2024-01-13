package leetcode_grind;

public class Day427 {
    // https://leetcode.com/problems/insertion-sort-list/description/
    static class ListNode {
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
        public ListNode insertionSortList(ListNode head) {
            var dummy = new ListNode(Integer.MIN_VALUE);
            var prev = dummy;

            var curr = head;
            while (curr != null) {
                var next = curr.next;
                curr.next = null;
                while (prev != null) {
                    if (prev.next == null || prev.next.val >= curr.val) {
                        curr.next = prev.next;
                        prev.next = curr;
                        break;
                    }
                    prev = prev.next;
                }
                curr = next;
                prev = dummy;
            }

            return dummy.next;
        }
    }
}
