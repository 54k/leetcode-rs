package leetcode_grind;

import java.util.Arrays;

public class Day902 {
    // https://leetcode.com/problems/count-number-of-balanced-permutations/description/?envType=daily-question&envId=2025-05-09
    static class Solution1 {
        static final long MOD = 1_000_000_007;
        long[][][] memo;
        int[] cnt;
        int[] psum;
        int target;
        long[][] comb;

        public int countBalancedPermutations(String num) {
            int tot = 0, n = num.length();
            cnt = new int[10];
            for (char ch : num.toCharArray()) {
                int d = ch - '0';
                cnt[d]++;
                tot += d;
            }
            if (tot % 2 != 0) {
                return 0;
            }

            target = tot / 2;
            int maxOdd = (n + 1) / 2;

            comb = new long[maxOdd + 1][maxOdd + 1];
            for (int i = 0; i <= maxOdd; i++) {
                comb[i][i] = comb[i][0] = 1;
                for (int j = 1; j < i; j++) {
                    comb[i][j] = (comb[i - 1][j] + comb[i - 1][j - 1]) % MOD;
                }
            }

            psum = new int[11];
            for (int i = 9; i >= 0; i--) {
                psum[i] = psum[i + 1] + cnt[i];
            }

            memo = new long[10][target + 1][maxOdd + 1];
            for (long[][] arr2 : memo) {
                for (long[] arr1 : arr2) {
                    Arrays.fill(arr1, -1);
                }
            }

            return (int) dfs(0, 0, maxOdd);
        }

        long dfs(int pos, int curr, int oddCnt) {
            if (oddCnt < 0 || psum[pos] < oddCnt || curr > target) {
                return 0;
            }
            if (pos > 9) {
                return curr == target && oddCnt == 0 ? 1 : 0;
            }
            if (memo[pos][curr][oddCnt] != -1) {
                return memo[pos][curr][oddCnt];
            }

            int evenCnt = psum[pos] - oddCnt;
            long res = 0;
            for (int i = Math.max(0, cnt[pos] - evenCnt); i <= Math.min(cnt[pos], oddCnt); i++) {
                long ways = (comb[oddCnt][i] * comb[evenCnt][cnt[pos] - i]) % MOD;
                res = (res + ((ways * dfs(pos + 1, curr + i * pos, oddCnt - i)) % MOD)) % MOD;
            }
            memo[pos][curr][oddCnt] = res;
            return res;
        }
    }

    static class Solution2 {
        static final long MOD = 1_000_000_007;

        public int countBalancedPermutations(String num) {
            int tot = 0, n = num.length();
            int[] cnt = new int[10];
            for (char ch : num.toCharArray()) {
                int d = ch - '0';
                cnt[d]++;
                tot += d;
            }
            if (tot % 2 != 0) {
                return 0;
            }

            int target = tot / 2;
            int maxOdd = (n + 1) / 2;
            long[][] comb = new long[maxOdd + 1][maxOdd + 1];
            long[][] f = new long[target + 1][maxOdd + 1];

            for (int i = 0; i <= maxOdd; i++) {
                comb[i][i] = comb[i][0] = 1;
                for (int j = 1; j < i; j++) {
                    comb[i][j] = (comb[i - 1][j] + comb[i - 1][j - 1]) % MOD;
                }
            }

            f[0][0] = 1;
            int psum = 0, totSum = 0;
            for (int i = 0; i <= 9; i++) {
                psum += cnt[i];
                totSum += i * cnt[i];
                for (int oddCnt = Math.min(psum, maxOdd); oddCnt >= Math.max(0, psum - (n - maxOdd)); oddCnt--) {
                    int evenCnt = psum - oddCnt;
                    for (int curr = Math.min(totSum, target); curr >= Math.max(0, totSum - target); curr--) {
                        long res = 0;
                        for (int j = Math.max(0, cnt[i] - evenCnt); j <= Math.min(cnt[i], oddCnt)
                                && i * j <= curr; j++) {
                            long ways = (comb[oddCnt][j] * comb[evenCnt][cnt[i] - j]) % MOD;
                            res = (res + ((ways * f[curr - i * j][oddCnt - j]) % MOD)) % MOD;
                        }
                        f[curr][oddCnt] = res % MOD;
                    }
                }
            }
            return (int) f[target][maxOdd];
        }
    }

}
