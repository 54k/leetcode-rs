package leetcode_grind;

import java.util.HashMap;
import java.util.Stack;

public class Day621 {
    // https://leetcode.com/problems/minimum-deletions-to-make-string-balanced/description/?envType=daily-question&envId=2024-07-30
    static class Solution1 {
        public int minimumDeletions(String s) {
            var st = new Stack<Character>();
            for (int i = 0; i < s.length(); i++) {
                while (i < s.length() && !st.isEmpty() && st.peek() > s.charAt(i)) {
                    st.pop();
                    i++;
                }
                if (i < s.length())
                    st.push(s.charAt(i));
            }
            return (s.length() - st.size()) / 2;
        }
    }

    static class Solution2 {
        public int minimumDeletions(String s) {
            int n = s.length();
            int[] countA = new int[n];
            int aCount = 0;

            for (int i = n - 1; i >= 0; i--) {
                countA[i] = aCount;
                if (s.charAt(i) == 'a')
                    aCount++;
            }

            int minimumDeletions = n;
            int bCount = 0;

            for (int i = 0; i < n; i++) {
                minimumDeletions = Math.min(countA[i] + bCount, minimumDeletions);
                if (s.charAt(i) == 'b')
                    bCount++;
            }

            return minimumDeletions;
        }
    }

    static class Solution3 {
        public int minimumDeletions(String s) {
            int n = s.length();
            int[] dp = new int[n + 1];
            int bCount = 0;

            for (int i = 0; i < n; i++) {
                if (s.charAt(i) == 'b') {
                    dp[i + 1] = dp[i];
                    bCount++;
                } else {
                    dp[i + 1] = Math.min(dp[i] + 1, bCount);
                }
            }

            return dp[n];
        }
    }

    // https://leetcode.com/problems/valid-parentheses/description/
    static class Solution4 {
        HashMap<Character, Character> mappings;
        {
            mappings = new HashMap<>();
            mappings.put(')', '(');
            mappings.put('}', '{');
            mappings.put(']', '[');
        }

        public boolean isValid(String s) {
            Stack<Character> stack = new Stack<>();

            for (int i = 0; i < s.length(); i++) {
                char c = s.charAt(i);

                if (mappings.containsKey(c)) {
                    char topElement = stack.empty() ? '#' : stack.pop();

                    if (topElement != mappings.get(c)) {
                        return false;
                    }
                } else {
                    stack.push(c);
                }
            }

            return stack.isEmpty();
        }
    }
}
