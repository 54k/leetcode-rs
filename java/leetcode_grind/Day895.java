package leetcode_grind;

public class Day895 {
    // https://leetcode.com/problems/push-dominoes/description/?envType=daily-question&envId=2025-05-02
    static class Solution1 {
        public String pushDominoes(String dominoes) {
            int N = dominoes.length();

            int[] indexes = new int[N + 2];
            char[] symbols = new char[N + 2];
            int len = 1;

            indexes[0] = -1;
            symbols[0] = 'L';

            for (int i = 0; i < N; ++i) {
                if (dominoes.charAt(i) != '.') {
                    indexes[len] = i;
                    symbols[len++] = dominoes.charAt(i);
                }
            }

            indexes[len] = N;
            symbols[len++] = 'R';

            char[] ans = dominoes.toCharArray();
            for (int index = 0; index < len - 1; ++index) {
                int i = indexes[index], j = indexes[index + 1];
                char x = symbols[index], y = symbols[index + 1];
                if (x == y) {
                    for (int k = i + 1; k < j; ++k) {
                        ans[k] = x;
                    }
                } else if (x > y) {
                    for (int k = i + 1; k < j; ++k) {
                        ans[k] = k - i == j - k ? '.' : k - i < j - k ? 'R' : 'L';
                    }
                }
            }

            return String.valueOf(ans);
        }
    }

    static class Solution2 {
        public String pushDominoes(String dominoes) {
            char[] A = dominoes.toCharArray();
            int N = A.length;
            int[] forces = new int[N];

            int force = 0;
            for (int i = 0; i < N; i++) {
                if (A[i] == 'R')
                    force = N;
                else if (A[i] == 'L')
                    force = 0;
                else
                    force = Math.max(force - 1, 0);
                forces[i] += force;
            }

            force = 0;
            for (int i = N - 1; i >= 0; --i) {
                if (A[i] == 'L')
                    force = N;
                else if (A[i] == 'R')
                    force = 0;
                else
                    force = Math.max(force - 1, 0);
                forces[i] -= force;
            }

            StringBuilder ans = new StringBuilder();
            for (int f : forces) {
                ans.append(f > 0 ? 'R' : f < 0 ? 'L' : '.');
            }
            return ans.toString();
        }
    }

}
