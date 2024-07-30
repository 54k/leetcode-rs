package leetcode_grind;

import java.util.Comparator;
import java.util.HashMap;
import java.util.PriorityQueue;
import java.util.Stack;

public class Day621 {
    // https://leetcode.com/problems/minimum-deletions-to-make-string-balanced/description/?envType=daily-question&envId=2024-07-30
    static class Solution1 {
        public int minimumDeletions(String s) {
            var st = new Stack<Character>();
            for (int i = 0; i < s.length(); i++) {
                while (i < s.length() && !st.isEmpty() && st.peek() > s.charAt(i)) {
                    st.pop();
                    i++;
                }
                if (i < s.length())
                    st.push(s.charAt(i));
            }
            return (s.length() - st.size()) / 2;
        }
    }

    static class Solution2 {
        public int minimumDeletions(String s) {
            int n = s.length();
            int[] countA = new int[n];
            int aCount = 0;

            for (int i = n - 1; i >= 0; i--) {
                countA[i] = aCount;
                if (s.charAt(i) == 'a')
                    aCount++;
            }

            int minimumDeletions = n;
            int bCount = 0;

            for (int i = 0; i < n; i++) {
                minimumDeletions = Math.min(countA[i] + bCount, minimumDeletions);
                if (s.charAt(i) == 'b')
                    bCount++;
            }

            return minimumDeletions;
        }
    }

    static class Solution3 {
        public int minimumDeletions(String s) {
            int n = s.length();
            int[] dp = new int[n + 1];
            int bCount = 0;

            for (int i = 0; i < n; i++) {
                if (s.charAt(i) == 'b') {
                    dp[i + 1] = dp[i];
                    bCount++;
                } else {
                    dp[i + 1] = Math.min(dp[i] + 1, bCount);
                }
            }

            return dp[n];
        }
    }

    // https://leetcode.com/problems/valid-parentheses/description/
    static class Solution4 {
        HashMap<Character, Character> mappings;
        {
            mappings = new HashMap<>();
            mappings.put(')', '(');
            mappings.put('}', '{');
            mappings.put(']', '[');
        }

        public boolean isValid(String s) {
            Stack<Character> stack = new Stack<>();

            for (int i = 0; i < s.length(); i++) {
                char c = s.charAt(i);

                if (mappings.containsKey(c)) {
                    char topElement = stack.empty() ? '#' : stack.pop();

                    if (topElement != mappings.get(c)) {
                        return false;
                    }
                } else {
                    stack.push(c);
                }
            }

            return stack.isEmpty();
        }
    }

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

    // https://leetcode.com/problems/merge-k-sorted-lists/description/
    static class Solution5 {
        public ListNode mergeKLists(ListNode[] lists) {
            ListNode head = new ListNode(0);
            ListNode point = head;
            PriorityQueue<ListNode> queue = new PriorityQueue<>(
                    new Comparator<ListNode>() {
                        @Override
                        public int compare(ListNode o1, ListNode o2) {
                            if (o1.val > o2.val) {
                                return 1;
                            } else if (o1.val == o2.val) {
                                return 0;
                            } else {
                                return -1;
                            }
                        }
                    });
            for (ListNode node : lists) {
                if (node != null) {
                    queue.add(node);
                }
            }
            while (!queue.isEmpty()) {
                point.next = queue.poll();
                point = point.next;
                if (point.next != null) {
                    queue.add(point.next);
                }
            }
            return head.next;
        }
    }

    static class Solution6 {
        public ListNode mergeKLists(ListNode[] lists) {
            int amount = lists.length;
            int interval = 1;
            while (interval < amount) {
                for (int i = 0; i < amount - interval; i += interval * 2) {
                    lists[i] = merge2lists(lists[i], lists[i + interval]);
                }
                interval *= 2;
            }
            return amount > 0 ? lists[0] : null;
        }

        ListNode merge2lists(ListNode l1, ListNode l2) {
            ListNode head = new ListNode(0);
            ListNode point = head;
            while (l1 != null && l2 != null) {
                if (l1.val <= l2.val) {
                    point.next = l1;
                    l1 = l1.next;
                } else {
                    point.next = l2;
                    l2 = l1;
                    l1 = point.next.next;
                }
                point = point.next;
            }
            if (l1 == null) {
                point.next = l2;
            } else {
                point.next = l1;
            }
            return head.next;
        }
    }
}
