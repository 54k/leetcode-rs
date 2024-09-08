package leetcode_grind;

public class Day661 {
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

    // https://leetcode.com/problems/split-linked-list-in-parts/description/
    static class Solution1 {
        public ListNode[] splitListToParts(ListNode head, int k) {
            ListNode[] ans = new ListNode[k];
            int size = 0;

            ListNode current = head;
            while (current != null) {
                size++;
                current = current.next;
            }

            int splitSize = size / k;
            int numRemainingParts = size % k;

            current = head;
            ListNode prev = current;

            for (int i = 0; i < k; i++) {
                ListNode newPart = current;
                int currentSize = splitSize;

                if (numRemainingParts > 0) {
                    numRemainingParts--;
                    currentSize++;
                }

                int j = 0;
                while (j < currentSize) {
                    prev = current;
                    current = current.next;
                    j++;
                }

                if (prev != null) {
                    prev.next = null;
                }

                ans[i] = newPart;
            }

            return ans;
        }
    }

}
