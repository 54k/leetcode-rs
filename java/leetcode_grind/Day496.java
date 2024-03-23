package leetcode_grind;

public class Day496 {

    // https://leetcode.com/problems/reorder-list/description/
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
        public void reorderList(ListNode head) {
            var middle = new Object() {
                ListNode apply(ListNode list) {
                    ListNode slow = list, fast = list;
                    while (fast != null && fast.next != null) {
                        slow = slow.next;
                        fast = fast.next.next;
                    }
                    return slow;
                }
            };
            var reverse = new Object() {
                ListNode apply(ListNode list) {
                    ListNode prev = null;
                    while (list != null) {
                        var next = list.next;
                        list.next = prev;
                        prev = list;
                        list = next;
                    }
                    return prev;
                }
            };
            var merge = new Object() {
                void apply(ListNode list1, ListNode list2) {
                    while (list2.next != null) {
                        var t = list1.next;
                        var t2 = list2.next;
                        list1.next = list2;
                        list2.next = t;
                        list2 = t2;
                        list1 = t;
                    }
                }
            };

            var m = middle.apply(head);
            var r = reverse.apply(m);
            merge.apply(head, r);
        }
    }

}
