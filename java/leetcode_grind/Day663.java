package leetcode_grind;

public class Day663 {
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

    // https://leetcode.com/problems/insert-greatest-common-divisors-in-linked-list/description/?envType=daily-question&envId=2024-09-10
    static class Solution1 {
        int gcd(int a, int b) {
            return b > 0 ? gcd(b, a % b) : a;
        }

        public ListNode insertGreatestCommonDivisors(ListNode head) {
            ListNode l = head, r = head.next;
            while (r != null) {
                ListNode m = new ListNode(gcd(l.val, r.val));
                l.next = m;
                m.next = r;
                l = r;
                r = r.next;
            }
            return head;
        }
    }
}
