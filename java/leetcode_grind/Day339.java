package leetcode_grind;

import java.util.ArrayList;
import java.util.Stack;

public class Day339 {
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

    static class Pair<F, S> {
        F key;
        S value;

        Pair(F k, S v) {
            key = k;
            value = v;
        }

        F getKey() {
            return key;
        }

        S getValue() {
            return value;
        }
    }

    static class Solution {
        public int[] nextLargerNodes(ListNode head) {
            var ans = new ArrayList<Integer>();
            var stack = new Stack<Pair<Integer, Integer>>();
            while (head != null) {
                while (stack.size() > 0 && stack.get(stack.size() - 1).getValue() < head.val) {
                    var top = stack.pop();
                    ans.set(top.getKey(), head.val);
                }
                ans.add(0);
                stack.push(new Pair<>(ans.size() - 1, head.val));
                head = head.next;
            }
            return ans.stream().mapToInt(int.class::cast).toArray();
        }
    }
}
