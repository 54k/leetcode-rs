package leetcode_grind;

public class Day495 {
    // https://leetcode.com/problems/palindrome-linked-list/description/
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
        public boolean isPalindrome(ListNode head) {
            var rev = new Object() {
                ListNode apply(ListNode node) {
                    ListNode h = null;
                    while (node != null) {
                        var next = node.next;
                        node.next = h;
                        h = node;
                        node = next;
                    }
                    return h;
                }
            };
            var mid = new Object() {
                ListNode apply(ListNode node) {
                    ListNode slow = node, fast = node.next;
                    while (fast != null && fast.next != null) {
                        slow = slow.next;
                        fast = fast.next.next;
                    }
                    return slow;
                }
            };

            ListNode m = mid.apply(head);
            ListNode sec = rev.apply(m.next);

            while (sec != null) {
                if (head.val != sec.val) {
                    return false;
                }
                head = head.next;
                sec = sec.next;
            }

            return true;
        }
    }

}
