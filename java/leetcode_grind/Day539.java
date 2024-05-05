package leetcode_grind;

public class Day539 {
    // https://leetcode.com/problems/delete-node-in-a-linked-list/
    public static class ListNode {
        int val;
        ListNode next;

        ListNode(int x) {
            val = x;
        }
    }

    static class Solution1 {
        public void deleteNode(ListNode node) {
            node.val = node.next.val;
            node.next = node.next.next;
        }
    }
}
