package leetcode_grind;

public class Day541 {
    // https://leetcode.com/problems/double-a-number-represented-as-a-linked-list/description
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
        ListNode reverseList(ListNode node) {
            ListNode previous = null, current = node, nextNode;
            while (current != null) {
                nextNode = current.next;
                current.next = previous;
                previous = current;
                current = nextNode;
            }
            return previous;
        }

        public ListNode doubleIt(ListNode head) {
            ListNode reversedList = reverseList(head);
            int carry = 0;
            ListNode current = reversedList, previous = null;
            while (current != null) {
                int newValue = current.val * 2 + carry;
                current.val = newValue % 10;
                if (newValue > 9) {
                    carry = 1;
                } else {
                    carry = 0;
                }
                previous = current;
                current = current.next;
            }
            if (carry != 0) {
                ListNode extraNode = new ListNode(carry);
                previous.next = extraNode;
            }
            ListNode result = reverseList(reversedList);
            return result;
        }
    }

}
