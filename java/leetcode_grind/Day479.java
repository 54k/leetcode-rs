package leetcode_grind;

public class Day479 {

    static class ListNode {
        int val;
        ListNode next;

        ListNode(int x) {
            val = x;
            next = null;
        }
    }

    // https://leetcode.com/problems/linked-list-cycle/
    public static class Solution1 {
        public boolean hasCycle(ListNode head) {
            if (head == null)
                return false;
            ListNode fast = head.next, slow = head;
            while (fast != slow) {
                if (fast == null || fast.next == null) {
                    return false;
                }
                slow = slow.next;
                fast = fast.next.next;
            }
            return true;
        }
    }
}
