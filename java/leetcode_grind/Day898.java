package leetcode_grind;

public class Day898 {
    // https://leetcode.com/problems/domino-and-tromino-tiling/description/?envType=daily-question&envId=2025-05-05
    static class Solution1 {
        int MOD = 1_000_000_007;

        Map<Integer, Long> f_cache = new HashMap<>();
        Map<Integer, Long> p_cache = new HashMap<>();

        long p(int n) {
            if (p_cache.containsKey(n)) {
                return p_cache.get(n);
            }

            long val;
            if (n == 2) {
                val = 1L;
            } else {
                val = (p(n - 1) + f(n - 2)) % MOD;
            }
            p_cache.put(n, val);
            return val;
        }

        long f(int n) {
            if (f_cache.containsKey(n)) {
                return f_cache.get(n);
            }

            long val;
            if (n == 1) {
                val = 1L;
            } else if (n == 2) {
                val = 2L;
            } else {
                val = (f(n - 1) + f(n - 2) + 2 * p(n - 1)) % MOD;
            }
            f_cache.put(n, val);
            return val;
        }

        public int numTilings(int n) {
            return (int) (f(n));
        }
    }

    static class Solution2 {
        public int numTilings(int n) {
            int MOD = 1_000_000_007;        
            if (n <= 2) {
                return n;
            }

            long[] f = new long[n + 1];
            long[] p = new long[n + 1];

            f[1] = 1L;
            f[2] = 2L;
            p[2] = 1L;

            for (int k = 3; k < n + 1; ++k) {
                f[k] = (f[k - 1] + f[k - 2] + 2 * p[k - 1]) % MOD;
                p[k] = (p[k - 1] + f[k - 2]) % MOD;
            }

            return (int) (f[n]);
        }
    }

    // https://leetcode.com/problems/longest-valid-parentheses/description/
    static class Solution3 {
        public int longestValidParentheses(String s) {
            int maxans = 0;        
            int dp[] = new int[s.length()];
            for (int i = 1; i < s.length(); i++) {
                if (s.charAt(i) == ')') {
                    if (s.charAt(i - 1) == '(') {
                        dp[i] = (i >= 2 ? dp[i - 2] : 0) + 2;
                    } else if (
                        i - dp[i - 1] > 0 && s.charAt(i - dp[i - 1] - 1) == '('
                    ) {
                        dp[i] = dp[i - 1] + ((i - dp[i - 1]) >= 2 ? dp[i - dp[i-1] - 2] : 0) + 2;
                        
                    }
                    maxans = Math.max(maxans, dp[i]);
                }
            }
            return maxans;
        }
    }

    static class Solution4 {
        public int longestValidParentheses(String s) {
            int maxans = 0;        
            Stack<Integer> stack = new Stack<>();
            stack.push(-1);
            for (int i = 0; i < s.length(); i++) {
                if (s.charAt(i) == '(') {
                    stack.push(i);
                } else {
                    stack.pop();
                    if (stack.empty()) {
                        stack.push(i);
                    } else {
                        maxans = Math.max(maxans, i - stack.peek());
                    }
                }
            }
            return maxans;
        }
    }

    static class Solution5 {
        public int longestValidParentheses(String s) {
            int left = 0, right = 0, maxlength = 0;
            for (int i = 0; i < s.length(); i++) {
                if (s.charAt(i) == '(') {
                    left++;
                } else {
                    right++;
                }
                if (left == right) {
                    maxlength = Math.max(maxlength, 2 * right);
                } else if (right > left) {
                    left = right = 0;
                }
            }

            left = right = 0;
            for (int i = s.length() - 1; i >= 0; i--) {
                if (s.charAt(i) == '(') {
                    left++;
                } else {
                    right++;
                }
                if (left == right) {
                    maxlength = Math.max(maxlength, 2 * left);
                } else if (left > right) {
                    left = right = 0;
                }
            }
            
            return maxlength;
        }
    }
}