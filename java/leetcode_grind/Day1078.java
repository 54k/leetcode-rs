package leetcode_grind;

import java.util.HashSet;

public class Day1078 {
    // https://leetcode.com/problems/delete-nodes-from-linked-list-present-in-array/description/?envType=daily-question&envId=2025-11-01
    static public class ListNode {
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
        public ListNode modifiedList(int[] nums, ListNode head) {
            var set = new HashSet<Integer>();
            for (var num : nums) {
                set.add(num);
            }
            var dummy = new ListNode(-1, head);
            var current = dummy;
            while (current.next != null) {
                if (set.contains(current.next.val)) {
                    current.next = current.next.next;
                } else {
                    current = current.next;
                }
            }
            return dummy.next;
        }
    }
}
