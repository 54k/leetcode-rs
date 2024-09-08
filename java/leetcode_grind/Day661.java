package leetcode_grind;

import java.util.*;

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

    static class Solution2 {
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

            for (int i = 0; i < k; i++) {
                ListNode newPart = new ListNode(0);
                ListNode tail = newPart;

                int currentSize = splitSize;
                if (numRemainingParts > 0) {
                    numRemainingParts--;
                    currentSize++;
                }
                int j = 0;
                while (j < currentSize) {
                    tail.next = new ListNode(current.val);
                    tail = tail.next;
                    j++;
                    current = current.next;
                }
                ans[i] = newPart.next;
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/split-a-circular-linked-list/description/
    static class Solution3 {
        public ListNode[] splitCircularLinkedList(ListNode list) {
            var n = 1;
            var end = list;
            while (end.next != list) {
                n++;
                end = end.next;
            }
            end.next = null;
            var secEnd = end;
            end = list;
            var i = 1;
            while (i < Math.ceil((double) n / 2)) {
                end = end.next;
                i++;
            }
            var nxt = end.next;
            secEnd.next = nxt;
            end.next = null;
            end.next = list;
            return new ListNode[] { list, nxt };
        }
    }

    static class Solution4 {
        public ListNode[] splitCircularLinkedList(ListNode list) {
            ListNode slow = list, fast = list;
            while (fast.next != list && fast.next.next != list) {
                fast = fast.next.next;
                slow = slow.next;
            }
            if (fast.next != list)
                fast = fast.next;
            fast.next = slow.next;
            slow.next = list;
            return new ListNode[] { slow.next, fast.next };
        }
    }

    // https://leetcode.com/problems/meeting-scheduler/description/?envType=weekly-question&envId=2024-09-08
    static class Solution5 {
        public List<Integer> minAvailableDuration(int[][] slots1, int[][] slots2, int duration) {
            Arrays.sort(slots1, (a, b) -> a[0] - b[0]);
            Arrays.sort(slots2, (a, b) -> a[0] - b[0]);

            int pointer1 = 0, pointer2 = 0;

            while (pointer1 < slots1.length && pointer2 < slots2.length) {
                int intersectLeft = Math.max(slots1[pointer1][0], slots2[pointer2][0]);
                int intersectRight = Math.min(slots1[pointer1][1], slots2[pointer2][1]);

                if (intersectRight - intersectLeft >= duration) {
                    return new ArrayList<Integer>(Arrays.asList(intersectLeft, intersectLeft + duration));
                }

                if (slots1[pointer1][1] < slots2[pointer2][1]) {
                    pointer1++;
                } else {
                    pointer2++;
                }
            }

            return new ArrayList<Integer>();
        }
    }

    static class Solution6 {
        public List<Integer> minAvailableDuration(int[][] slots1, int[][] slots2, int duration) {
            PriorityQueue<int[]> timeslots = new PriorityQueue<>((slot1, slot2) -> slot1[0] - slot2[0]);
            for (int[] slot : slots1) {
                if (slot[1] - slot[0] >= duration)
                    timeslots.offer(slot);
            }
            for (int[] slot : slots2) {
                if (slot[1] - slot[0] >= duration)
                    timeslots.offer(slot);
            }
            while (timeslots.size() > 1) {
                int[] slot1 = timeslots.poll();
                int[] slot2 = timeslots.peek();
                if (slot1[1] >= slot2[0] + duration) {
                    return new ArrayList<Integer>(Arrays.asList(slot2[0], slot2[0] + duration));
                }
            }
            return new ArrayList<Integer>();
        }
    }
}
