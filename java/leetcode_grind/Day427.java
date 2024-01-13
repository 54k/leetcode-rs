package leetcode_grind;

public class Day427 {
    // https://leetcode.com/problems/insertion-sort-list/description/
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

    static class Solution1 {
        public ListNode insertionSortList(ListNode head) {
            var dummy = new ListNode(Integer.MIN_VALUE);
            var prev = dummy;

            var curr = head;
            while (curr != null) {
                var next = curr.next;
                curr.next = null;
                while (prev != null) {
                    if (prev.next == null || prev.next.val >= curr.val) {
                        curr.next = prev.next;
                        prev.next = curr;
                        break;
                    }
                    prev = prev.next;
                }
                curr = next;
                prev = dummy;
            }

            return dummy.next;
        }
    }

    // https://leetcode.com/problems/maximum-area-of-longest-diagonal-rectangle/description/
    static class Solution2 {
        public int areaOfMaxDiagonal(int[][] dimensions) {
            int ans = 0;
            double mx = 0.0;

            for (var d : dimensions) {
                double a = (double) d[0] * (double) d[0];
                double b = (double) d[1] * (double) d[1];
                double diag = Math.sqrt((double) a + (double) b);

                if (diag > mx) {
                    ans = d[0] * d[1];
                    mx = diag;
                } else if (diag == mx) {
                    ans = Math.max(ans, d[0] * d[1]);
                }
            }

            return ans;
        }
    }
}
