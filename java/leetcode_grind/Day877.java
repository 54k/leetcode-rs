package leetcode_grind;

import java.util.ArrayList;
import java.util.List;
import java.util.Stack;

public class Day877 {
    // https://leetcode.com/problems/count-good-triplets/description/
    static class Solution1 {
        public int countGoodTriplets(int[] arr, int a, int b, int c) {
            int ans = 0, n = arr.length;
            int[] sum = new int[1001];
            for (int j = 0; j < n; j++) {
                for (int k = j + 1; k < n; k++) {
                    if (Math.abs(arr[j] - arr[k]) <= b) {
                        int lj = arr[j] - a, rj = arr[j] + a;
                        int lk = arr[k] - c, rk = arr[k] + c;
                        int l = Math.max(0, Math.max(lj, lk)), r = Math.min(1000, Math.min(rj, rk));
                        if (l <= r) {
                            if (l == 0) {
                                ans += sum[r];
                            } else {
                                ans += sum[r] - sum[l - 1];
                            }
                        }
                    }
                }
                for (int k = arr[j]; k <= 1000; ++k) {
                    ++sum[k];
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/decode-string/description/
    static class Solution2 {
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

    static class Solution3 {
        public String decodeString(String s) {
            Stack<Integer> countStack = new Stack<>();
            Stack<StringBuilder> stringStack = new Stack<>();

            StringBuilder currentString = new StringBuilder();
            int k = 0;

            for (char ch : s.toCharArray()) {
                if (Character.isDigit(ch)) {
                    k = k * 10 + ch - '0';
                } else if (ch == '[') {
                    countStack.push(k);
                    stringStack.push(currentString);
                    currentString = new StringBuilder();
                    k = 0;
                } else if (ch == ']') {
                    StringBuilder decodedString = stringStack.pop();
                    for (int currentK = countStack.pop(); currentK > 0; currentK--) {
                        decodedString.append(currentString);
                    }
                    currentString = decodedString;
                } else {
                    currentString.append(ch);
                }
            }

            return currentString.toString();
        }
    }

    static class Solution4 {
        int index = 0;

        public String decodeString(String s) {
            StringBuilder result = new StringBuilder();
            while (index < s.length() && s.charAt(index) != ']') {
                if (!Character.isDigit(s.charAt(index))) {
                    result.append(s.charAt(index++));
                } else {
                    int k = 0;
                    while (index < s.length() && Character.isDigit(s.charAt(index))) {
                        k = k * 10 + s.charAt(index++) - '0';
                    }
                    index++;
                    String decodedString = decodeString(s);
                    index++;
                    while (k-- > 0) {
                        result.append(decodedString);
                    }
                }
            }
            return new String(result);
        }
    }
}
