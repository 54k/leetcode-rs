package leetcode_grind;

import java.util.Stack;

public class Day471 {
    static class Solution1 {
        // https://leetcode.com/problems/remove-one-element-to-make-the-array-strictly-increasing/description/
        public boolean canBeIncreasing(int[] nums) {
            var s = new Stack<Integer>();
            boolean shot = false;
            for (var n : nums) {
                if (!s.isEmpty() && n <= s.peek()) {
                    if (shot)
                        return false;
                    shot = true;
                    int prev = s.pop();
                    int mn = Math.min(prev, n);
                    int mx = Math.max(prev, n);

                    if (s.isEmpty() || s.peek() < mn) {
                        s.push(mn);
                    } else {
                        s.push(mx);
                    }
                } else {
                    s.push(n);
                }
            }
            return true;
        }
    }
}
