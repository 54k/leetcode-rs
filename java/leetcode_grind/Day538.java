package leetcode_grind;

import java.util.Arrays;
import java.util.Stack;

public class Day538 {
    static class Solution1 {
        public int numRescueBoats(int[] people, int limit) {
            int ans = 0;
            Arrays.sort(people);
            int left = 0, right = people.length - 1;
            while (left <= right) {
                int l = people[left];
                int r = people[right];

                ans += 1;
                if (l + r <= limit) {
                    left++;
                }
                right--;

            }
            return ans;
        }
    }

    // https://leetcode.com/problems/4-keys-keyboard/description
    static class Solution2 {
        public int maxA(int n) {
            int[] dp = new int[n + 1];
            for (int i = 0; i <= n; i++) {
                dp[i] = i;
            }
            for (int i = 0; i < n - 2; i++) {
                for (int j = i + 3; j < Math.min(i + 6, n) + 1; j++) {
                    dp[j] = Math.max(dp[j], (j - i - 1) * dp[i]);
                }
            }
            return dp[n];
        }
    }

    // https://leetcode.com/problems/longest-valid-parentheses/description
    static class Solution3 {
        public int longestValidParentheses(String s) {
            int maxans = 0;
            Stack<Integer> stack = new Stack<>();
            stack.push(-1);
            for (int i = 0; i < s.length(); i++) {
                if (s.charAt(i) == '(') {
                    stack.push(i);
                } else {
                    stack.pop();
                    if (stack.isEmpty()) {
                        stack.push(i);
                    } else {
                        maxans = Math.max(maxans, i - stack.peek());
                    }
                }
            }
            return maxans;
        }
    }
}
