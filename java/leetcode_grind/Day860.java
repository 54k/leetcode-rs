package leetcode_grind;

public class Day860 {
    // https://leetcode.com/problems/maximum-number-of-points-from-grid-queries/description/
    static class Solution1 {
        public int[] maxPoints(int[][] grid, int[] queries) {
            int rowCount = grid.length, colCount = grid[0].length;
            int[] result = new int[queries.length];
            int[][] DIRECTIONS = { { 0, 1 }, { 1, 0 }, { 0, -1 }, { -1, 0 } };

            int[][] sortedQueries = new int[queries.length][2];
            for (int index = 0; index < queries.length; index++) {
                sortedQueries[index][0] = queries[index];
                sortedQueries[index][1] = index;
            }

            Arrays.sort(sortedQueries, (a, b) -> a[0] - b[0]);

            PriorityQueue<int[]> minHeap = new PriorityQueue<>(
                    (a, b) -> a[0] - b[0]);
            boolean[][] visited = new boolean[rowCount][colCount];

            int totalPoints = 0;
            minHeap.add(new int[] { grid[0][0], 0, 0 });
            visited[0][0] = true;

            for (int[] query : sortedQueries) {
                int queryValue = query[0], queryIndex = query[1];
                while (!minHeap.isEmpty() && minHeap.peek()[0] < queryValue) {
                    int[] top = minHeap.poll();
                    int currentRow = top[1], currentCol = top[2];
                    totalPoints++;
                    for (int[] dir : DIRECTIONS) {
                        int newRow = currentRow + dir[0], newCol = currentCol + dir[1];

                        if (newRow >= 0 && newCol >= 0 && newRow < rowCount && newCol < colCount
                                && !visited[newRow][newCol]) {
                            minHeap.add(new int[] { grid[newRow][newCol], newRow, newCol });
                            visited[newRow][newCol] = true;
                        }
                    }
                }
                result[queryIndex] = totalPoints;
            }

            return result;
        }
    }

    static class Solution2 {
        int[][] DIRECTIONS = { { 0, 1 }, { 1, 0 }, { 0, -1 }, { -1, 0 } };

        public int[] maxPoints(int[][] grid, int[] queries) {
            int queryCount = queries.length;
            int[] result = new int[queryCount];
            int rowCount = grid.length, colCount = grid[0].length;

            int totalCells = rowCount * colCount;
            int[] thresholdForMaxPoints = new int[totalCells + 1];
            int[][] minValueToReach = new int[rowCount][colCount];

            for (int[] row : minValueToReach) {
                Arrays.fill(row, Integer.MAX_VALUE);
            }
            minValueToReach[0][0] = grid[0][0];
            PriorityQueue<int[]> minHeap = new PriorityQueue<>((a, b) -> Integer.compare(a[2], b[2]));
            minHeap.offer(new int[] { 0, 0, grid[0][0] });
            int visitedCells = 0;

            while (!minHeap.isEmpty()) {
                int[] current = minHeap.poll();
                thresholdForMaxPoints[++visitedCells] = current[2];
                for (int[] direction : DIRECTIONS) {
                    int newRow = current[0] + direction[0];
                    int newCol = current[1] + direction[1];

                    if (newRow >= 0 && newRow < rowCount && newCol >= 0 && newCol < colCount
                            && minValueToReach[newRow][newCol] == Integer.MAX_VALUE) {
                        minValueToReach[newRow][newCol] = Math.max(current[2], grid[newRow][newCol]);
                        minHeap.offer(new int[] { newRow, newCol, minValueToReach[newRow][newCol] });
                    }
                }
            }

            for (int i = 0; i < queryCount; i++) {
                int threshold = queries[i];
                int left = 0, right = totalCells;

                while (left < right) {
                    int mid = (left + right + 1) >>> 1;
                    if (thresholdForMaxPoints[mid] < threshold) {
                        left = mid;
                    } else {
                        right = mid - 1;
                    }
                }

                result[i] = left;
            }

            return result;
        }
    }

}
