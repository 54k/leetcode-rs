package leetcode_grind;

public class Day679 {
    // https://leetcode.com/problems/my-calendar-i/description/?envType=daily-question&envId=2024-09-26
    // class MyCalendar {

    // List<int[]> calendar;

    // public MyCalendar() {
    // calendar = new ArrayList<>();
    // }

    // public boolean book(int start, int end) {
    // for (int[] iv : calendar) {
    // if (iv[0] < end && start < iv[1]) {
    // return false;
    // }
    // }
    // calendar.add(new int[] { start, end });
    // return true;
    // }
    // }

    // /**
    // * Your MyCalendar object will be instantiated and called as such:
    // * MyCalendar obj = new MyCalendar();
    // * boolean param_1 = obj.book(start,end);
    // */

    // class MyCalendar {
    // TreeMap<Integer, Integer> calendar;

    // public MyCalendar() {
    // calendar = new TreeMap<>();
    // }

    // public boolean book(int start, int end) {
    // Integer prev = calendar.floorKey(start),
    // next = calendar.ceilingKey(start);
    // if ((prev == null || calendar.get(prev) <= start) &&
    // (next == null || end <= next)) {
    // calendar.put(start, end);
    // return true;
    // }
    // return false;
    // }
    // }

    // https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/description/
    static class Solution1 {
        static class TrieNode {
            TrieNode[] children = new TrieNode[2];
        }

        public int findMaximumXOR(int[] nums) {
            int maxNum = nums[0];
            for (int num : nums) {
                maxNum = Math.max(maxNum, num);
            }

            int L = (Integer.toBinaryString(maxNum)).length();
            int maxXor = 0;

            TrieNode root = new TrieNode();
            for (int num : nums) {
                TrieNode node = root, xorNode = root;
                int currXor = 0;

                for (int i = L - 1; i >= 0; i--) {
                    int bit = (num >> i) & 1;
                    int toggledBit = bit ^ 1;

                    if (node.children[bit] == null) {
                        TrieNode newNode = new TrieNode();
                        node.children[bit] = newNode;
                    }
                    node = node.children[bit];

                    if (xorNode.children[toggledBit] != null) {
                        currXor |= (1 << i);
                        xorNode = xorNode.children[toggledBit];
                    } else {
                        xorNode = xorNode.children[bit];
                    }
                }
                maxXor = Math.max(maxXor, currXor);
            }
            return maxXor;
        }
    }
}
