package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day356 {
    // https://leetcode.com/problems/build-an-array-with-stack-operations/description/
    static class Solution {
        public List<String> buildArray(int[] target, int n) {
            var tptr = 0;
            var stack = new ArrayList<String>();
            for (int i = 1; i <= n && tptr < target.length; i++) {
                stack.add("Push");
                if (i != target[tptr]) {
                    stack.add("Pop");
                } else {
                    tptr++;
                }
            }
            return stack;
        }
    }
}
