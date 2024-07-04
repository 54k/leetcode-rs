package leetcode_grind;

public class Day596 {
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

    // https://leetcode.com/problems/merge-nodes-in-between-zeros/description/?envType=daily-question&envId=2024-07-04
    static class Solution1 {
        public ListNode mergeNodes(ListNode head) {
            ListNode ptr = head;
            ListNode newHead = new ListNode(-1);
            ListNode headPtr = newHead;
            while (ptr != null) {
                if (ptr.val == 0 && ptr.next != null) {
                    headPtr.next = new ListNode(0);
                    headPtr = headPtr.next;
                } else {
                    headPtr.val += ptr.val;
                }
                ptr = ptr.next;
            }
            return newHead.next;
        }
    }

}
