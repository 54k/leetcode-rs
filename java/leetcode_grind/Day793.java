package leetcode_grind;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Deque;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.Queue;

public class Day793 {
    // https://leetcode.com/problems/first-completely-painted-row-or-column/description/?envType=daily-question&envId=2025-01-20
    static class Solution1 {
        public int firstCompleteIndex(int[] arr, int[][] mat) {
            Map<Integer, Integer> numToIndex = new HashMap<>();
            for (int i = 0; i < arr.length; i++) {
                numToIndex.put(arr[i], i);
            }

            int result = Integer.MAX_VALUE;
            int numRows = mat.length;
            int numCols = mat[0].length;

            for (int row = 0; row < numRows; row++) {
                int lastElementIndex = Integer.MIN_VALUE;
                for (int col = 0; col < numCols; col++) {
                    int indexVal = numToIndex.get(mat[row][col]);
                    lastElementIndex = Math.max(lastElementIndex, indexVal);
                }
                result = Math.min(result, lastElementIndex);
            }

            for (int col = 0; col < numCols; col++) {
                int lastElementIndex = Integer.MIN_VALUE;
                for (int row = 0; row < numRows; row++) {
                    int indexVal = numToIndex.get(mat[row][col]);
                    lastElementIndex = Math.max(lastElementIndex, indexVal);
                }
                result = Math.min(result, lastElementIndex);
            }

            return result;
        }
    }

    // https://leetcode.com/problems/minimum-cost-to-make-at-least-one-valid-path-in-a-grid/description/
    static class Solution2 {
        public int minCost(int[][] grid) {
            int numRows = grid.length, numCols = grid[0].length;
            int[][] minChanges = new int[numRows][numCols];

            for (int[] row : minChanges) {
                Arrays.fill(row, Integer.MAX_VALUE);
            }
            minChanges[0][0] = 0;

            while (true) {
                int[][] prevState = new int[numRows][numCols];
                for (int row = 0; row < numRows; row++) {
                    prevState[row] = Arrays.copyOf(minChanges[row], numCols);
                }

                for (int row = 0; row < numRows; row++) {
                    for (int col = 0; col < numCols; col++) {
                        if (row > 0) {
                            minChanges[row][col] = Math.min(minChanges[row][col],
                                    minChanges[row - 1][col] + (grid[row - 1][col] == 3 ? 0 : 1));
                        }

                        if (col > 0) {
                            minChanges[row][col] = Math.min(minChanges[row][col],
                                    minChanges[row][col - 1] + (grid[row][col - 1] == 1 ? 0 : 1));
                        }
                    }
                }

                for (int row = numRows - 1; row >= 0; row--) {
                    for (int col = numCols - 1; col >= 0; col--) {
                        if (row < numRows - 1) {
                            minChanges[row][col] = Math.min(minChanges[row][col],
                                    minChanges[row + 1][col] + (grid[row + 1][col] == 4 ? 0 : 1));
                        }

                        if (col < numCols - 1) {
                            minChanges[row][col] = Math.min(minChanges[row][col],
                                    minChanges[row][col + 1] + (grid[row][col + 1] == 2 ? 0 : 1));
                        }
                    }
                }

                if (Arrays.deepEquals(prevState, minChanges)) {
                    break;
                }
            }

            return minChanges[numRows - 1][numCols - 1];
        }
    }

    static class Solution3 {
        int[][] dirs = { { 0, 1 }, { 0, -1 }, { 1, 0 }, { -1, 0 } };

        public int minCost(int[][] grid) {
            int numRows = grid.length, numCols = grid[0].length;
            int[][] minCost = new int[numRows][numCols];
            for (int[] row : minCost) {
                Arrays.fill(row, Integer.MAX_VALUE);
            }

            Deque<int[]> deque = new ArrayDeque<>();
            deque.offerFirst(new int[] { 0, 0 });
            minCost[0][0] = 0;

            while (!deque.isEmpty()) {
                int[] curr = deque.pollFirst();
                int row = curr[0], col = curr[1];

                for (int dir = 0; dir < 4; dir++) {
                    int newRow = row + dirs[dir][0];
                    int newCol = col + dirs[dir][1];
                    int cost = (grid[row][col] != (dir + 1)) ? 1 : 0;

                    if (isValid(newRow, newCol, numRows, numCols)
                            && minCost[row][col] + cost < minCost[newRow][newCol]) {
                        minCost[newRow][newCol] = minCost[row][col] + cost;
                        if (cost == 1) {
                            deque.offerLast(new int[] { newRow, newCol });
                        } else {
                            deque.offerFirst(new int[] { newRow, newCol });
                        }
                    }
                }
            }

            return minCost[numRows - 1][numCols - 1];
        }

        boolean isValid(int row, int col, int numRows, int numCols) {
            return row >= 0 && row < numRows && col >= 0 && col < numCols;
        }
    }

    static class Solution4 {
        int[][] dirs = new int[][] {
                { 0, 1 },
                { 0, -1 },
                { 1, 0 },
                { -1, 0 },
        };

        public int minCost(int[][] grid) {
            int numRows = grid.length, numCols = grid[0].length, cost = 0;
            int[][] minCost = new int[numRows][numCols];
            for (int row = 0; row < numRows; row++) {
                Arrays.fill(minCost[row], Integer.MAX_VALUE);
            }

            Queue<int[]> queue = new LinkedList<>();
            dfs(grid, 0, 0, minCost, cost, queue);

            while (!queue.isEmpty()) {
                cost++;
                int levelSize = queue.size();

                while (levelSize-- > 0) {
                    int[] curr = queue.poll();
                    int row = curr[0], col = curr[1];
                    for (int dir = 0; dir < 4; dir++) {
                        dfs(grid, row + dirs[dir][0], col + dirs[dir][1], minCost, cost, queue);
                    }
                }
            }

            return minCost[numRows - 1][numCols - 1];
        }

        void dfs(
                int[][] grid,
                int row,
                int col,
                int[][] minCost,
                int cost,
                Queue<int[]> queue) {
            if (!isUnvisited(minCost, row, col))
                return;

            minCost[row][col] = cost;
            queue.offer(new int[] { row, col });

            int nextDir = grid[row][col] - 1;
            dfs(grid, row + dirs[nextDir][0], col + dirs[nextDir][1], minCost, cost, queue);
        }

        boolean isUnvisited(int[][] minCost, int row, int col) {
            return (row >= 0 && col >= 0 && row < minCost.length && col < minCost[0].length
                    && minCost[row][col] == Integer.MAX_VALUE);
        }
    }

    // https://leetcode.com/problems/minimum-weighted-subgraph-with-the-required-paths/description/
    static class Solution5 {
        ArrayList<int[]>[] nextGraph, preGraph;

        public long minimumWeight(int n, int[][] edges, int src1, int src2, int dest) {
            buildGraph(n, edges);

            long[] src1To = new long[n], src2To = new long[n], toDest = new long[n];
            Arrays.fill(src1To, -1);
            Arrays.fill(src2To, -1);
            Arrays.fill(toDest, -1);

            shortestPath(src1, src1To, nextGraph);
            shortestPath(src2, src2To, nextGraph);
            shortestPath(dest, toDest, preGraph);

            long res = -1;
            for (int i = 0; i < n; i++) {
                long d1 = src1To[i], d2 = src2To[i], d3 = toDest[i];

                if (d1 >= 0 && d2 >= 0 && d3 >= 0) {
                    long d = d1 + d2 + d3;
                    if (res == -1 || d < res) {
                        res = d;
                    }
                }
            }
            return res;
        }

        void buildGraph(int n, int[][] edges) {
            nextGraph = new ArrayList[n];
            preGraph = new ArrayList[n];
            for (int i = 0; i < n; i++) {
                nextGraph[i] = new ArrayList<int[]>();
                preGraph[i] = new ArrayList<int[]>();
            }

            for (int[] edge : edges) {
                int from = edge[0], to = edge[1], weight = edge[2];
                nextGraph[from].add(new int[] { to, weight });
                preGraph[to].add(new int[] { from, weight });
            }
        }

        void shortestPath(int src, long[] srcTo, ArrayList<int[]>[] graph) {
            PriorityQueue<long[]> heap = new PriorityQueue<>((a, b) -> Long.compare(a[1], b[1]));
            heap.offer(new long[] { src, 0 });

            while (!heap.isEmpty()) {
                long[] node = heap.poll();
                int to = (int) node[0];
                long dist = node[1];

                if (srcTo[to] != -1 && srcTo[to] <= dist)
                    continue;
                srcTo[to] = dist;

                for (int[] next : graph[to]) {
                    heap.offer(new long[] { next[0], dist + next[1] });
                }
            }
        }
    }
}
