package leetcode_grind;

public class Day597 {
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

    // https://leetcode.com/problems/find-the-minimum-and-maximum-number-of-nodes-between-critical-points/description/?envType=daily-question&envId=2024-07-05
    static class Solution1 {
        public int[] nodesBetweenCriticalPoints(ListNode head) {
            int[] result = { -1, -1 };
            int minDistance = Integer.MAX_VALUE;

            ListNode previousNode = head;
            ListNode currentNode = head.next;
            int currentIndex = 1;
            int previousCriticalIndex = 0;
            int firstCriticalIndex = 0;

            while (currentNode.next != null) {
                if ((currentNode.val < previousNode.val && currentNode.val < currentNode.next.val)
                        || (currentNode.val > previousNode.val && currentNode.val > currentNode.next.val)) {
                    if (previousCriticalIndex == 0) {
                        previousCriticalIndex = currentIndex;
                        firstCriticalIndex = currentIndex;
                    } else {
                        minDistance = Math.min(minDistance, currentIndex - previousCriticalIndex);
                        previousCriticalIndex = currentIndex;
                    }
                }
                currentIndex++;
                previousNode = currentNode;
                currentNode = currentNode.next;
            }

            if (minDistance != Integer.MAX_VALUE) {
                int maxDistance = previousCriticalIndex - firstCriticalIndex;
                result = new int[] { minDistance, maxDistance };
            }
            return result;
        }
    }
}
