package leetcode_grind;

public class Day1018 {
    // https://leetcode.com/problems/find-the-number-of-ways-to-place-people-i/description/?envType=daily-question&envId=2025-09-02
    static class Solution1 {
        public int numberOfPairs(int[][] points) {
            int ans = 0;
            int n = points.length;

            for (int i = 0; i < n; i++) {
                int[] pointA = points[i];
                for (int j = 0; j < n; j++) {
                    int[] pointB = points[j];
                    if (i == j || !(pointA[0] <= pointB[0] && pointA[1] >= pointB[1])) {
                        continue;
                    }

                    if (n == 2) {
                        ans++;
                        continue;
                    }

                    boolean illegal = false;
                    for (int k = 0; k < n; k++) {
                        if (k == i || k == j) {
                            continue;
                        }

                        int[] pointTmp = points[k];
                        boolean isXContained = pointTmp[0] >= pointA[0] && pointTmp[0] <= pointB[0];
                        boolean isYContained = pointTmp[1] <= pointA[1] && pointTmp[1] >= pointB[1];

                        if (isXContained && isYContained) {
                            illegal = true;
                            break;
                        }
                    }
                    if (!illegal) {
                        ans++;
                    }
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/android-unlock-patterns/description/
    static class Solution2 {

        int backtrack(int num, int length, int m, int n, int[][][] moves, boolean[] vis) {
            if (length > n)
                return 0;

            int result = 0;
            if (length >= m)
                result++;

            for (int i = 0; i < moves[num].length; i++) {
                int nextNum = moves[num][i][0];
                if (vis[nextNum])
                    continue;
                if (moves[num][i][1] != -1 && !vis[moves[num][i][1]])
                    continue;

                vis[nextNum] = true;
                result += backtrack(nextNum, length + 1, m, n, moves, vis);
                vis[nextNum] = false;
            }

            return result;
        }

        public int numberOfPatterns(int m, int n) {
            int result = 0;
            int[][][] moves = new int[][][] {
                    {},
                    { { 2, -1 }, { 4, -1 }, { 5, -1 }, { 6, -1 }, { 8, -1 }, { 3, 2 }, { 9, 5 }, { 7, 4 } },
                    { { 1, -1 }, { 3, -1 }, { 5, -1 }, { 4, -1 }, { 6, -1 }, { 7, -1 }, { 9, -1 }, { 8, 5 } },
                    { { 2, -1 }, { 6, -1 }, { 5, -1 }, { 4, -1 }, { 8, -1 }, { 1, 2 }, { 9, 6 }, { 7, 5 } },
                    { { 1, -1 }, { 5, -1 }, { 7, -1 }, { 2, -1 }, { 8, -1 }, { 3, -1 }, { 9, -1 }, { 6, 5 } },
                    { { 1, -1 }, { 2, -1 }, { 3, -1 }, { 4, -1 }, { 6, -1 }, { 7, -1 }, { 8, -1 }, { 9, -1 } },
                    { { 1, -1 }, { 2, -1 }, { 3, -1 }, { 5, -1 }, { 7, -1 }, { 8, -1 }, { 9, -1 }, { 4, 5 } },
                    { { 2, -1 }, { 4, -1 }, { 5, -1 }, { 6, -1 }, { 8, -1 }, { 1, 4 }, { 3, 5 }, { 9, 8 } },
                    { { 1, -1 }, { 3, -1 }, { 4, -1 }, { 5, -1 }, { 6, -1 }, { 7, -1 }, { 9, -1 }, { 2, 5 } },
                    { { 2, -1 }, { 4, -1 }, { 5, -1 }, { 6, -1 }, { 8, -1 }, { 7, 8 }, { 3, 6 }, { 1, 5 } },
            };

            for (int i = 1; i <= 9; i++) {
                boolean[] vis = new boolean[10];
                vis[i] = true;
                result += backtrack(i, 1, m, n, moves, vis);
            }
            return result;
        }
    }
}
