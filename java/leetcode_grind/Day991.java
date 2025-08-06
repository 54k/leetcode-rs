package leetcode_grind;

import java.util.Arrays;

public class Day991 {
    // https://leetcode.com/problems/fruits-into-baskets-iii/description/?envType=daily-question&envId=2025-08-06
    static class Solution1 {
        public int numOfUnplacedFruits(int[] fruits, int[] baskets) {
            int n = baskets.length;
            int m = (int) Math.sqrt(n);
            int section = (n + m - 1) / m;
            int count = 0;
            int[] maxV = new int[section];
            Arrays.fill(maxV, 0);

            for (int i = 0; i < n; i++) {
                maxV[i / m] = Math.max(maxV[i / m], baskets[i]);
            }

            for (int fruit : fruits) {
                int sec;
                int unset = 1;
                for (sec = 0; sec < section; sec++) {
                    if (maxV[sec] < fruit) {
                        continue;
                    }
                    int choose = 0;
                    maxV[sec] = 0;
                    for (int i = 0; i < m; i++) {
                        int pos = sec * m + i;
                        if (pos < n && baskets[pos] >= fruit && choose == 0) {
                            baskets[pos] = 0;
                            choose = 1;
                        }
                        if (pos < n) {
                            maxV[sec] = Math.max(maxV[sec], baskets[pos]);
                        }
                    }
                    unset = 0;
                    break;
                }
                count += unset;
            }
            return count;
        }
    }

}
