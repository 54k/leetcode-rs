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

    static class Solution2 {
        public ListNode doubleIt(ListNode head) {
            var twiceOfVal = new Object() {
                int apply(ListNode node) {
                    if (node == null)
                        return 0;
                    int doubledValue = node.val * 2 + apply(node.next);
                    node.val = doubledValue % 10;
                    return doubledValue / 10;
                }
            };

            int carry = twiceOfVal.apply(head);
            if (carry != 0) {
                head = new ListNode(carry, head);
            }
            return head;
        }
    }

    static class Solution3 {
        public ListNode doubleIt(ListNode head) {
            ListNode curr = head;
            ListNode prev = null;
            while (curr != null) {
                int twiceOfVal = curr.val * 2;
                if (twiceOfVal < 10) {
                    curr.val = twiceOfVal;
                } else if (prev != null) {
                    curr.val = twiceOfVal % 10;
                    prev.val = prev.val + 1;
                } else {
                    head = new ListNode(1, curr);
                    curr.val = twiceOfVal % 10;
                }
                prev = curr;
                curr = curr.next;
            }
            return head;
        }
    }

    static class Solution4 {
        public ListNode doubleIt(ListNode head) {
            if (head.val > 4) {
                head = new ListNode(0, head);
            }
            for (ListNode node = head; node != null; node = node.next) {
                node.val = (node.val * 2) % 10;
                if (node.next != null && node.next.val > 4) {
                    node.val++;
                }
            }
            return head;
        }
    }
}
