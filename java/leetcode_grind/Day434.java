package leetcode_grind;

import java.util.Stack;

public class Day434 {
    // https://leetcode.com/problems/sum-of-subarray-minimums/description
    static class Solution1 {
        public int sumSubarrayMins(int[] arr) {
            var log2 = new Object() {
                int apply(int n) {
                    return (int) (Math.log(n) / Math.log(2));
                }
            };

            var n = arr.length;
            var d = log2.apply(n);

            var st = new int[d + 1][n];
            for (int i = 0; i < n; i++) {
                st[0][i] = arr[i];
            }
            for (int i = 0; i < d; i++) {
                for (int j = 0; j + (1 << i) < n; j++) {
                    st[i + 1][j] = Math.min(st[i][j], st[i][j + (1 << i)]);
                }
            }
            var stMin = new Object() {
                int apply(int lo, int hi) {
                    int log = log2.apply(hi - lo + 1);
                    return Math.min(st[log][lo], st[log][hi - (1 << log) + 1]);
                }
            };

            long MOD = 1_000_000_007;
            long ans = 0;
            for (int i = 0; i < n; i++) {
                for (int j = 0; j <= i; j++) {
                    ans += stMin.apply(j, i);
                    ans %= MOD;
                }
            }
            return (int) ans;
        }

        public int sumSubarrayMins2(int[] arr) {
            int MOD = 1_000_000_007;
            int n = arr.length;

            Stack<Integer> stack = new Stack<>();
            long sumOfMinimums = 0;

            for (int i = 0; i <= n; i++) {
                while (!stack.isEmpty() && (i == n || arr[stack.peek()] >= arr[i])) {
                    int mid = stack.pop();
                    int left = stack.isEmpty() ? -1 : stack.peek();
                    int right = i;

                    long numOfSubArrays = (mid - left) * (right - mid);
                    sumOfMinimums += (numOfSubArrays * arr[mid]);
                    sumOfMinimums %= MOD;
                }
                stack.add(i);
            }

            return (int) sumOfMinimums;
        }

        public int sumSubarrayMins3(int[] arr) {
            int MOD = 1_000_000_007;
            int n = arr.length;

            Stack<Integer> stack = new Stack<>();
            int[] dp = new int[n];

            for (int i = 0; i < n; i++) {
                while (!stack.isEmpty() && arr[stack.peek()] >= arr[i]) {
                    stack.pop();
                }

                if (stack.size() > 0) {
                    int prevSmaller = stack.peek();
                    dp[i] = dp[prevSmaller] + (i - prevSmaller) * arr[i];
                } else {
                    dp[i] = (i + 1) * arr[i];
                }
                stack.push(i);
            }

            long sum = 0;
            for (int count : dp) {
                sum += count;
                sum %= MOD;
            }
            return (int) sum;
        }
    }
}
