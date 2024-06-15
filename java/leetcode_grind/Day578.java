package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.PriorityQueue;

public class Day578 {
    // https://leetcode.com/problems/ipo/description/
    static class Solution1 {
        public int findMaximizedCapital(int k, int w, int[] profits, int[] capital) {
            var n = profits.length;
            var pq = new PriorityQueue<int[]>((a, b) -> {
                return b[0] - a[0];
            });
            for (int i = 0; i < n; i++) {
                pq.add(new int[] { profits[i], capital[i] });
            }
            while (k > 0 && !pq.isEmpty()) {
                var lst = new ArrayList<int[]>();
                while (!pq.isEmpty() && pq.peek()[1] > w) {
                    lst.add(pq.remove());
                }
                if (pq.isEmpty()) {
                    break;
                }
                var p = pq.remove();
                w += p[0];
                k--;
                for (var e : lst) {
                    pq.add(e);
                }
            }
            return w;
        }
    }

    // https://leetcode.com/problems/put-boxes-into-the-warehouse-i/description/
    static class Solution2 {
        public int maxBoxesInWarehouse(int[] boxes, int[] warehouse) {
            for (int i = 1; i < warehouse.length; i++) {
                warehouse[i] = Math.min(warehouse[i - 1], warehouse[i]);
            }
            Arrays.sort(boxes);
            int count = 0;
            for (int i = warehouse.length - 1; i >= 0; i--) {
                if (count < boxes.length && boxes[count] <= warehouse[i]) {
                    count++;
                }
            }
            return count;
        }
    }

    // https://leetcode.com/problems/put-boxes-into-the-warehouse-ii/description
    static class Solution3 {
        public int maxBoxesInWarehouse(int[] boxes, int[] warehouse) {
            int n = warehouse.length;
            int minHeight = Integer.MAX_VALUE;
            int[] effectiveHeights = new int[n];

            for (int i = 0; i < n; i++) {
                minHeight = Math.min(minHeight, warehouse[i]);
                effectiveHeights[i] = minHeight;
            }

            minHeight = Integer.MAX_VALUE;
            for (int i = n - 1; i >= 0; i--) {
                minHeight = Math.min(minHeight, warehouse[i]);
                effectiveHeights[i] = Math.max(effectiveHeights[i], minHeight);
            }

            Arrays.sort(effectiveHeights);
            Arrays.sort(boxes);

            int count = 0;
            int boxIndex = 0;

            for (int i = 0; i < n; i++) {
                if (boxIndex < boxes.length && boxes[boxIndex] <= effectiveHeights[i]) {
                    ++count;
                    ++boxIndex;
                }
            }

            return count;
        }
    }

    // https://leetcode.com/problems/put-boxes-into-the-warehouse-ii/description
    static class Solution4 {
        public int maxBoxesInWarehouse(int[] boxes, int[] warehouse) {
            int warehouseSize = warehouse.length;
            Arrays.sort(boxes);

            int leftIndex = 0;
            int rightIndex = warehouseSize - 1;
            int boxCount = 0;
            int boxIndex = boxes.length - 1;

            while (leftIndex <= rightIndex && boxIndex >= 0) {
                if (boxes[boxIndex] <= warehouse[leftIndex]) {
                    boxCount++;
                    leftIndex++;
                } else if (boxes[boxIndex] <= warehouse[rightIndex]) {
                    boxCount++;
                    rightIndex--;
                }
                boxIndex--;
            }
            return boxCount;
        }
    }
}
