package leetcode_grind;

import java.util.Arrays;
import java.util.HashSet;

public class Day358 {
    // https://leetcode.com/problems/perfect-squares/description/
    static class Solution {
        public int numSquares(int n) {
            var dp = new int[n + 1];
            Arrays.fill(dp, Integer.MAX_VALUE);
            dp[0] = 0;

            var maxSquareIndex = (int) Math.sqrt(n) + 1;
            var squareNums = new int[maxSquareIndex];
            for (var i = 1; i < maxSquareIndex; i++) {
                squareNums[i] = i * i;
            }

            for (var i = 1; i <= n; i++) {
                for (var s = 1; s < maxSquareIndex; s++) {
                    if (i < squareNums[s]) {
                        break;
                    }
                    dp[i] = Math.min(dp[i], dp[i - squareNums[s]] + 1);
                }
            }
            return dp[n];
        }

        public int numSquaresRecursion(int n) {
            var squareNums = new HashSet<Integer>();
            var isDividedBy = new Object() {
                boolean apply(int n, int count) {
                    if (count == 1) {
                        return squareNums.contains(n);
                    }

                    for (var square : squareNums) {
                        if (apply(n - square, count - 1)) {
                            return true;
                        }
                    }
                    return false;
                }
            };

            for (int i = 1; i * i <= n; i++) {
                squareNums.add(i * i);
            }

            var count = 1;
            for (; count <= n; ++count) {
                if (isDividedBy.apply(n, count)) {
                    return count;
                }
            }
            return count;
        }

        public int numSquaresGreedyBFS(int n) {
            var squareNums = new HashSet<Integer>();
            var isDividedBy = new Object() {
                boolean apply(int n, int count) {
                    if (count == 1) {
                        return squareNums.contains(n);
                    }

                    for (var square : squareNums) {
                        if (apply(n - square, count - 1)) {
                            return true;
                        }
                    }
                    return false;
                }
            };

            for (int i = 1; i * i <= n; i++) {
                squareNums.add(i * i);
            }

            var count = 1;
            for (; count <= n; ++count) {
                if (isDividedBy.apply(n, count)) {
                    return count;
                }
            }
            return count;
        }
    }
}
