package leetcode_grind;

public class Day968 {
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

    // https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/description/?envType=daily-question&envId=2025-07-14
    static class Solution {
        public int getDecimalValue(ListNode head) {
            int num = head.val;
            while (head.next != null) {
                num = (num << 1) | head.next.val;
                head = head.next;
            }
            return num;
        }
    }
}