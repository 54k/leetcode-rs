package leetcode_grind;

import java.util.Arrays;

public class Day583 {
    // https://leetcode.com/problems/magnetic-force-between-two-balls/description/
    static class Solution1 {
        public int maxDistance(int[] position, int m) {
            if (position.length < m) {
                return -1;
            }

            Arrays.sort(position);

            var check = new Object() {
                boolean apply(int mid) {
                    int place = 1;
                    int last = 0;
                    for (int i = 1; i < position.length; i++) {
                        if (position[i] - position[last] >= mid) {
                            last = i;
                            place++;
                        }
                        if (place == m) {
                            return true;
                        }
                    }
                    return place == m;
                }
            };

            int lo = 0;
            int hi = ((int) 1e9) + 7;
            while (lo + 1 < hi) {
                int mid = lo + (hi - lo) / 2;
                if (check.apply(mid)) {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            return lo;
        }
    }
}
