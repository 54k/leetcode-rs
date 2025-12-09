package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;

public class Day1116 {
    // https://leetcode.com/problems/count-special-triplets/description/?envType=daily-question&envId=2025-12-09
    static class Solution1 {
        public int specialTriplets(int[] nums) {
            final int MOD = 1000000007;
            Map<Integer, Integer> numCnt = new HashMap<>();
            Map<Integer, Integer> numPartialCnt = new HashMap<>();
            for (int v : nums) {
                numCnt.put(v, numCnt.getOrDefault(v, 0) + 1);
            }

            long ans = 0;
            for (int v : nums) {
                int target = v * 2;
                int lCnt = numPartialCnt.getOrDefault(target, 0);
                numPartialCnt.put(v, numPartialCnt.getOrDefault(v, 0) + 1);
                int rCnt = numCnt.getOrDefault(target, 0) - numPartialCnt.getOrDefault(target, 0);
                ans = (ans + (long) lCnt * rCnt) % MOD;
            }
            return (int) ans;
        }
    }

    static class Solution2 {
        static final int MOD = 1000000007;

        int[] upperBound(List<Integer> arr, int i) {
            int l = 0;
            int r = arr.size() - 1;
            while (l < r) {
                int mid = l + ((r - l + 1) >> 1);
                if (i >= arr.get(mid)) {
                    l = mid;
                } else {
                    r = mid - 1;
                }
            }
            return new int[] { l + 1, arr.size() - 1 - l };
        }

        public int specialTriplets(int[] nums) {
            Map<Integer, List<Integer>> pos = new HashMap<>();
            for (int i = 0; i < nums.length; i++) {
                int v = nums[i];
                pos.computeIfAbsent(v, k -> new ArrayList<>()).add(i);
            }
            int ans = 0;
            for (int i = 1; i < nums.length - 1; i++) {
                int target = nums[i] * 2;
                List<Integer> targetPos = pos.get(target);
                if (targetPos != null && targetPos.size() > 1 && targetPos.get(0) < i) {
                    int[] lr = upperBound(targetPos, i);
                    int l = lr[0];
                    int r = lr[1];
                    if (nums[i] == 0) {
                        l--;
                    }
                    ans = (int) ((ans + (long) l * r) % MOD);
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/contain-virus/description/
    static class Solution3 {
        Set<Integer> seen;
        List<Set<Integer>> regions;
        List<Set<Integer>> frontiers;
        List<Integer> perimeters;

        int[][] grid;
        int R, C;
        int[] dr = new int[] { -1, 1, 0, 0 };
        int[] dc = new int[] { 0, 0, -1, 1 };

        public int containVirus(int[][] isInfected) {
            this.grid = isInfected;
            R = grid.length;
            C = grid[0].length;

            int ans = 0;
            while (true) {
                seen = new HashSet<>();
                regions = new ArrayList<>();
                frontiers = new ArrayList<>();
                perimeters = new ArrayList<>();

                for (int r = 0; r < R; r++) {
                    for (int c = 0; c < C; c++) {
                        if (grid[r][c] == 1 && !seen.contains(r * C + c)) {
                            regions.add(new HashSet<>());
                            frontiers.add(new HashSet<>());
                            perimeters.add(0);
                            dfs(r, c);
                        }
                    }
                }

                if (regions.isEmpty())
                    break;
                int triageIndex = 0;
                for (int i = 0; i < frontiers.size(); i++) {
                    if (frontiers.get(triageIndex).size() < frontiers.get(i).size()) {
                        triageIndex = i;
                    }
                }
                ans += perimeters.get(triageIndex);

                for (int i = 0; i < regions.size(); i++) {
                    if (i == triageIndex) {
                        for (int code : regions.get(i)) {
                            grid[code / C][code % C] = -1;
                        }
                    } else {
                        for (int code : regions.get(i)) {
                            int r = code / C, c = code % C;
                            for (int k = 0; k < 4; k++) {
                                int nr = r + dr[k], nc = c + dc[k];
                                if (nr >= 0 && nr < R && nc >= 0 && nc < C && grid[nr][nc] == 0) {
                                    grid[nr][nc] = 1;
                                }
                            }
                        }
                    }
                }
            }

            return ans;
        }

        void dfs(int r, int c) {
            if (!seen.contains(r * C + c)) {
                seen.add(r * C + c);
                int N = regions.size();
                regions.get(N - 1).add(r * C + c);
                for (int k = 0; k < 4; k++) {
                    int nr = r + dr[k], nc = c + dc[k];
                    if (nr >= 0 && nr < R && nc >= 0 && nc < C) {
                        if (grid[nr][nc] == 1) {
                            dfs(nr, nc);
                        } else if (grid[nr][nc] == 0) {
                            frontiers.get(N - 1).add(nr * C + nc);
                            perimeters.set(N - 1, perimeters.get(N - 1) + 1);
                        }
                    }
                }
            }
        }
    }

}
