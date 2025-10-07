package leetcode_grind;

import java.util.HashSet;
import java.util.PriorityQueue;
import java.util.Set;
import java.util.Stack;

public class Day1051 {

    // https://leetcode.com/problems/swim-in-rising-water/description/?envType=daily-question&envId=2025-10-06
    static class Solution1 {
        public int swimInWater(int[][] grid) {
            int N = grid.length;
            Set<Integer> seen = new HashSet<>();
            PriorityQueue<Integer> pq = new PriorityQueue<>((k1, k2) -> grid[k1 / N][k1 % N] - grid[k2 / N][k2 % N]);
            pq.offer(0);
            int ans = 0;

            int[] dr = new int[] { 1, -1, 0, 0 };
            int[] dc = new int[] { 0, 0, 1, -1 };

            while (!pq.isEmpty()) {
                int k = pq.poll();
                int r = k / N, c = k % N;

                ans = Math.max(ans, grid[r][c]);
                if (r == N - 1 && c == N - 1)
                    return ans;

                for (int i = 0; i < 4; i++) {
                    int cr = r + dr[i], cc = c + dc[i];
                    int ck = cr * N + cc;
                    if (0 <= cr && cr < N && 0 <= cc && cc < N && !seen.contains(ck)) {
                        pq.offer(ck);
                        seen.add(ck);
                    }
                }
            }
            throw null;
        }
    }

    static class Solution2 {
        public int swimInWater(int[][] grid) {
            int N = grid.length;
            int lo = grid[0][0], hi = N * N;

            while (lo < hi) {
                int mid = lo + (hi - lo) / 2;
                if (!possible(mid, grid)) {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }

            return lo;
        }

        boolean possible(int T, int[][] grid) {
            int N = grid.length;
            Set<Integer> seen = new HashSet<>();
            seen.add(0);

            int[] dr = new int[] { 1, -1, 0, 0 };
            int[] dc = new int[] { 0, 0, 1, -1 };

            Stack<Integer> stack = new Stack<>();
            stack.add(0);

            while (!stack.isEmpty()) {
                int k = stack.pop();
                int r = k / N, c = k % N;
                if (r == N - 1 && c == N - 1)
                    return true;

                for (int i = 0; i < 4; i++) {
                    int cr = r + dr[i], cc = c + dc[i];
                    int ck = cr * N + cc;

                    if (0 <= cr && cr < N && 0 <= cc && cc < N && !seen.contains(ck) && grid[cr][cc] <= T) {
                        stack.add(ck);
                        seen.add(ck);
                    }
                }
            }

            return false;
        }
    }
}
