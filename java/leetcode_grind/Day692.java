package leetcode_grind;

import java.util.Stack;

public class Day692 {
    // https://leetcode.com/problems/minimum-number-of-swaps-to-make-the-string-balanced/description/?envType=daily-question&envId=2024-10-08
    static class Solution1 {
        public int minSwaps(String s) {
            Stack<Character> stack = new Stack<>();
            int unbalanced = 0;
            for (int i = 0; i < s.length(); i++) {
                char ch = s.charAt(i);
                if (ch == '[')
                    stack.push(ch);
                else {
                    if (!stack.isEmpty())
                        stack.pop();
                    else
                        unbalanced++;
                }
            }
            return (unbalanced + 1) / 2;
        }
    }

    

}
