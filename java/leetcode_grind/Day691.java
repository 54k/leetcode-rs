package leetcode_grind;

import java.util.*;

public class Day691 {
    // https://leetcode.com/problems/minimum-string-length-after-removing-substrings/description/?envType=daily-question&envId=2024-10-07
    static class Solution1 {
        public int minLength(String s) {
            while (s.contains("AB") || s.contains("CD")) {
                if (s.contains("AB")) {
                    s = s.replace("AB", "");
                } else if (s.contains("CD")) {
                    s = s.replace("CD", "");
                }
            }
            return s.length();
        }
    }

    static class Solution2 {
        public int minLength(String s) {
            Stack<Character> stack = new Stack<>();
            for (int i = 0; i < s.length(); i++) {
                char currentChar = s.charAt(i);
                if (stack.isEmpty()) {
                    stack.push(currentChar);
                    continue;
                }
                if (currentChar == 'B' && stack.peek() == 'A') {
                    stack.pop();
                } else if (currentChar == 'D' && stack.peek() == 'C') {
                    stack.pop();
                } else {
                    stack.push(currentChar);
                }
            }
            return stack.size();
        }
    }

    static class Solution3 {
        public int minLength(String s) {
            int writePtr = 0;
            char[] charArray = s.toCharArray();
            for (int readPtr = 0; readPtr < s.length(); readPtr++) {
                charArray[writePtr] = charArray[readPtr];

                if (writePtr > 0 && (charArray[writePtr - 1] == 'A' || charArray[writePtr - 1] == 'C')
                        && charArray[writePtr] == charArray[writePtr - 1] + 1) {
                    writePtr--;
                } else {
                    writePtr++;
                }
            }
            return writePtr;
        }
    }
}
