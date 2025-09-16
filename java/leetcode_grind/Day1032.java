package leetcode_grind;

import java.util.ArrayList;
import java.util.List;
import java.util.Stack;

public class Day1032 {
    // https://leetcode.com/problems/replace-non-coprime-numbers-in-array/description/?envType=daily-question&envId=2025-09-16
    static class Solution1 {
        static int lcm(int x, int y) {
            return x / gcd(x, y) * y;
        }

        static int gcd(int x, int y) {
            if (y == 0) {
                return x;
            }
            return gcd(y, x % y);
        }

        public List<Integer> replaceNonCoprimes(int[] nums) {
            Stack<Integer> stack = new Stack<>();
            for (int x : nums) {
                int cur = x;
                while (!stack.isEmpty()) {
                    int gcd = gcd(stack.peek(), cur);
                    if (gcd == 1)
                        break;
                    cur = lcm(stack.peek(), cur);
                    stack.pop();
                }
                stack.push(cur);
            }
            return new ArrayList<>(stack);
        }
    }

}
