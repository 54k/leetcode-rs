package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day600 {
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

    // https://leetcode.com/problems/remove-duplicates-from-an-unsorted-linked-list/description/?envType=weekly-question&envId=2024-07-08
    static class Solution1 {
        public ListNode deleteDuplicatesUnsorted(ListNode head) {
            ListNode dummy = new ListNode(-1, head);
            Map<Integer, Integer> frequency = new HashMap<Integer, Integer>();
            ListNode temp = head, current = dummy.next, prev = dummy;

            while (temp != null) {
                frequency.put(temp.val, frequency.getOrDefault(temp.val, 0) + 1);
                temp = temp.next;
            }

            while (current != null) {
                if (frequency.get(current.val) > 1) {
                    prev.next = current.next;
                } else {
                    prev = current;
                }
                current = current.next;
            }
            return dummy.next;
        }
    }
}
