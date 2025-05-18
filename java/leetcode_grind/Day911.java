package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day911 {
    // https://leetcode.com/problems/painting-a-grid-with-three-different-colors/description/?envType=daily-question&envId=2025-05-18
    static class Solution1 {
        static final int mod = 1000000007;

        public int colorTheGrid(int m, int n) {
            Map<Integer, List<Integer>> valid = new HashMap<>();
            int maskEnd = (int) Math.pow(3, m);
            for (int mask = 0; mask < maskEnd; ++mask) {
                List<Integer> color = new ArrayList<>();
                int mm = mask;
                for (int i = 0; i < m; i++) {
                    color.add(mm % 3);
                    mm /= 3;
                }

                boolean check = true;
                for (int i = 0; i < m - 1; i++) {
                    if (color.get(i).equals(color.get(i + 1))) {
                        check = false;
                        break;
                    }
                }

                if (check) {
                    valid.put(mask, color);
                }
            }

            Map<Integer, List<Integer>> adjacent = new HashMap<>();
            for (int mask1 : valid.keySet()) {
                for (int mask2 : valid.keySet()) {
                    boolean check = true;
                    for (int i = 0; i < m; i++) {
                        if (valid.get(mask1).get(i).equals(valid.get(mask2).get(i))) {
                            check = false;
                            break;
                        }
                    }
                    if (check) {
                        adjacent.computeIfAbsent(mask1, k -> new ArrayList<>()).add(mask2);
                    }
                }
            }

            Map<Integer, Integer> f = new HashMap<>();
            for (int mask : valid.keySet()) {
                f.put(mask, 1);
            }
            for (int i = 1; i < n; i++) {
                Map<Integer, Integer> g = new HashMap<>();
                for (int mask2 : valid.keySet()) {
                    for (int mask1 : adjacent.getOrDefault(mask2, new ArrayList<>())) {
                        g.put(mask2, (g.getOrDefault(mask2, 0) + f.getOrDefault(mask1, 0)) % mod);
                    }
                }
                f = g;
            }

            int ans = 0;
            for (int num : f.values()) {
                ans = (ans + num) % mod;
            }
            return ans;
        }
    }
}
