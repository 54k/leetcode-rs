package leetcode_grind;

import java.util.ArrayList;
import java.util.List;
import java.util.PriorityQueue;

public class Day728 {
    // https://leetcode.com/problems/minimized-maximum-of-products-distributed-to-any-store/description/?envType=daily-question&envId=2024-11-14
    static class Solution1 {
        boolean canDistribute(int x, int[] quantities, int n) {
            int j = 0;
            int remaining = quantities[j];
            for (int i = 0; i < n; i++) {
                if (remaining <= x) {
                    j++;
                    if (j == quantities.length) {
                        return true;
                    } else {
                        remaining = quantities[j];
                    }
                } else {
                    remaining -= x;
                }
            }
            return false;
        }

        public int minimizedMaximum(int n, int[] quantities) {
            int left = 0;
            int right = 0;
            for (int quantity : quantities) {
                if (quantity > right) {
                    right = quantity;
                }
            }

            while (left < right) {
                int middle = (left + right) / 2;
                if (canDistribute(middle, quantities, n)) {
                    right = middle;
                } else {
                    left = middle + 1;
                }
            }
            return left;
        }
    }

    static class Solution2 {
        public int minimizedMaximum(int n, int[] quantities) {
            int m = quantities.length;
            List<int[]> typeStorePairsArray = new ArrayList<>();
            for (int i = 0; i < m; i++) {
                typeStorePairsArray.add(new int[] { quantities[i], 1 });
            }
            PriorityQueue<int[]> typeStorePairs = new PriorityQueue<>(
                    (a, b) -> Long.compare((long) b[0] * a[1], (long) a[0] * b[1]));
            typeStorePairs.addAll(typeStorePairsArray);

            for (int i = 0; i < n - m; i++) {
                int[] pairWithMaxRatio = typeStorePairs.poll();
                int totalQuatnityOfType = pairWithMaxRatio[0];
                int storesAssignedToType = pairWithMaxRatio[1];

                typeStorePairs.offer(new int[] { totalQuatnityOfType, storesAssignedToType + 1 });
            }

            int[] pairWithMaxRatio = typeStorePairs.poll();
            int totalQuatnityOfType = pairWithMaxRatio[0];
            int storesAssignedToType = pairWithMaxRatio[1];

            return (int) Math.ceil((double) totalQuatnityOfType / storesAssignedToType);
        }
    }
}
