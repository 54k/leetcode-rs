package leetcode_grind;

import java.util.ArrayList;
import java.util.List;
import java.util.Stack;

public class Day998 {
    // https://leetcode.com/problems/power-of-three/description/?envType=daily-question&envId=2025-08-13
    static class Solution1 {
        public boolean isPowerOfThree(int n) {
            if (n < 1) {
                return false;
            }
            while (n % 3 == 0) {
                n /= 3;
            }
            return n == 1;
        }
    }

    static class Solution2 {
        public boolean isPowerOfThree(int n) {
            return Integer.toString(n, 3).matches("^10*$");
        }
    }

    static class Solution3 {
        public boolean isPowerOfThree(int n) {
            return (Math.log10(n) / Math.log10(3)) % 1 == 0;
        }
    }

    // https://leetcode.com/problems/decode-string/description/
    static class Solution4 {
        public String decodeString(String s) {
            Stack<Character> stack = new Stack<>();
            for (int i = 0; i < s.length(); i++) {
                if (s.charAt(i) == ']') {
                    List<Character> decodedString = new ArrayList<>();
                    while (stack.peek() != '[') {
                        decodedString.add(stack.pop());
                    }
                    stack.pop();
                    int base = 1;
                    int k = 0;
                    while (!stack.isEmpty() && Character.isDigit(stack.peek())) {
                        k = k + (stack.pop() - '0') * base;
                        base *= 10;
                    }
                    while (k != 0) {
                        for (int j = decodedString.size() - 1; j >= 0; j--) {
                            stack.push(decodedString.get(j));
                        }
                        k--;
                    }
                } else {
                    stack.push(s.charAt(i));
                }
            }

            char[] result = new char[stack.size()];
            for (int i = result.length - 1; i >= 0; i--) {
                result[i] = stack.pop();
            }
            return new String(result);
        }
    }
}
