package leetcode_grind;

import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class Day1136 {
    // https://leetcode.com/problems/pyramid-transition-matrix/description/?envType=daily-question&envId=2025-12-29
    static class Solution1 {
        int[][] T;
        Set<Long> seen;

        public boolean pyramidTransition(String bottom, List<String> allowed) {
            T = new int[7][7];
            for (String a : allowed) {
                T[a.charAt(0) - 'A'][a.charAt(1) - 'A'] |= 1 << (a.charAt(2) - 'A');
            }

            seen = new HashSet<>();
            int N = bottom.length();
            int[][] A = new int[N][N];
            int t = 0;
            for (char c : bottom.toCharArray()) {
                A[N - 1][t++] = c - 'A';
            }
            return solve(A, 0, N - 1, 0);
        }

        boolean solve(int[][] A, long R, int N, int i) {
            if (N == 1 && i == 1) {
                return true;
            } else if (i == N) {
                if (seen.contains(R))
                    return false;
                seen.add(R);
                return solve(A, 0, N - 1, 0);
            } else {
                int w = T[A[N][i]][A[N][i + 1]];
                for (int b = 0; b < 7; ++b)
                    if (((w >> b) & 1) != 0) {
                        A[N - 1][i] = b;
                        if (solve(A, R * 8 + (b + 1), N, i + 1))
                            return true;
                    }
                return false;
            }
        }
    }

    // https://leetcode.com/problems/diet-plan-performance/description/?envType=weekly-question&envId=2025-12-29
    static class Solution2 {
        public int dietPlanPerformance(int[] calories, int k, int lower, int upper) {
            int sum = 0;
            int score = 0;
            for (int i = 0; i < k; i++) {
                sum += calories[i];
            }
            for (int i = k; i < calories.length; i++) {
                if (sum < lower) {
                    score--;
                } else if (sum > upper) {
                    score++;
                }
                sum += calories[i] - calories[i - k];
            }
            if (sum < lower) {
                score--;
            } else if (sum > upper) {
                score++;
            }
            return score;
        }
    }
}
