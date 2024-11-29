package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.PriorityQueue;

public class Day742 {
    // https://leetcode.com/problems/minimum-time-to-visit-a-cell-in-a-grid/description/?envType=daily-question&envId=2024-11-29
    static class Solution1 {
        public int minimumTime(int[][] grid) {
            if (grid[0][1] > 1 && grid[1][0] > 1) {
                return -1;
            }

            int rows = grid.length, cols = grid[0].length;
            int[][] directions = { { 1, 0 }, { -1, 0 }, { 0, 1 }, { 0, -1 } };
            boolean[][] visited = new boolean[rows][cols];

            PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> Integer.compare(a[0], b[0]));
            pq.add(new int[] { grid[0][0], 0, 0 });

            while (!pq.isEmpty()) {
                int[] curr = pq.poll();
                int time = curr[0], row = curr[1], col = curr[2];

                if (row == rows - 1 && col == cols - 1) {
                    return time;
                }

                if (visited[row][col]) {
                    continue;
                }
                visited[row][col] = true;

                for (int[] d : directions) {
                    int nextRow = row + d[0], nextCol = col + d[1];
                    if (!isValid(visited, nextRow, nextCol)) {
                        continue;
                    }

                    int waitTime = ((grid[nextRow][nextCol] - time) % 2 == 0) ? 1 : 0;
                    int nextTime = Math.max(grid[nextRow][nextCol] + waitTime, time + 1);
                    pq.add(new int[] { nextTime, nextRow, nextCol });
                }
            }
            return -1;
        }

        boolean isValid(boolean[][] visited, int row, int col) {
            return (row >= 0 &&
                    col >= 0 &&
                    row < visited.length &&
                    col < visited[0].length &&
                    !visited[row][col]);
        }
    }

    // https://leetcode.com/problems/distance-to-a-cycle-in-undirected-graph/description/?envType=weekly-question&envId=2024-11-29
    static class Solution2 {
        boolean detectCycleNodes(
                int currentNode,
                List<List<Integer>> adjacencyList,
                boolean[] isInCycle,
                boolean[] visited,
                int[] parent) {
            visited[currentNode] = true;
            for (int neighbor : adjacencyList.get(currentNode)) {
                if (!visited[neighbor]) {
                    parent[neighbor] = currentNode;
                    if (detectCycleNodes(neighbor, adjacencyList, isInCycle, visited, parent)) {
                        return true;
                    }
                } else if (parent[currentNode] != neighbor) {
                    isInCycle[neighbor] = true;
                    int tempNode = currentNode;
                    while (tempNode != neighbor) {
                        isInCycle[tempNode] = true;
                        tempNode = parent[tempNode];
                    }
                    return true;
                }
            }
            return false;
        }

        void calculateDistances(
                int currentNode,
                int currentDistance,
                List<List<Integer>> adjacencyList,
                int[] distances,
                boolean[] visited,
                boolean[] isInCycle) {
            distances[currentNode] = currentDistance;
            visited[currentNode] = true;
            for (int neighbor : adjacencyList.get(currentNode)) {
                if (visited[neighbor])
                    continue;
                int newDistance = isInCycle[neighbor] ? 0 : currentDistance + 1;
                calculateDistances(neighbor, newDistance, adjacencyList, distances, visited, isInCycle);
            }
        }

        public int[] distanceToCycle(int n, int[][] edges) {
            boolean[] isInCycle = new boolean[n];
            boolean[] visited = new boolean[n];
            int[] parent = new int[n];
            int[] distances = new int[n];
            List<List<Integer>> adjacencyList = new ArrayList<>();

            for (int i = 0; i < n; i++) {
                adjacencyList.add(new ArrayList<>());
            }

            for (int[] edge : edges) {
                adjacencyList.get(edge[0]).add(edge[1]);
                adjacencyList.get(edge[1]).add(edge[0]);
            }

            detectCycleNodes(0, adjacencyList, isInCycle, visited, parent);

            Arrays.fill(visited, false);

            for (int i = 0; i < n; i++) {
                if (isInCycle[i]) {
                    calculateDistances(i, 0, adjacencyList, distances, visited, isInCycle);
                    break;
                }
            }

            return distances;
        }
    }
}
