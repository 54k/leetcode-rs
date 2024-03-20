package leetcode_grind;

public class Day493 {
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

    // https://leetcode.com/problems/merge-in-between-linked-lists/description/
    static class Solution1 {
        public ListNode mergeInBetween(ListNode list1, int a, int b, ListNode list2) {
            ListNode aptr = list1, bptr = aptr, tptr = aptr;
            int i = 0;
            while (tptr != null) {
                if (i < b) {
                    bptr = bptr.next;
                    if (i > (b - a)) {
                        aptr = aptr.next;
                    }
                }
                tptr = tptr.next;
                i++;
            }
            aptr.next = list2;
            while (aptr.next != null) {
                aptr = aptr.next;
            }
            aptr.next = bptr.next;
            return list1;
        }
    }
}
