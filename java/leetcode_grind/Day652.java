package leetcode_grind;

import java.util.*;

public class Day652 {
    // https://leetcode.com/problems/modify-graph-edge-weights/description/?envType=daily-question&envId=2024-08-30
    static class Solution1 {
        static final int INF = (int) 2e9;

        public int[][] modifiedGraphEdges(int n, int[][] edges, int source, int destination, int target) {
            long currentShortestDistance = runDijkstra(
                    edges,
                    n,
                    source,
                    destination);

            if (currentShortestDistance < target)
                return new int[0][0];

            boolean matchesTarget = (currentShortestDistance == target);

            for (int[] edge : edges) {
                if (edge[2] > 0)
                    continue;
                edge[2] = matchesTarget ? INF : 1;

                if (!matchesTarget) {
                    long newDistance = runDijkstra(edges, n, source, destination);

                    if (newDistance <= target) {
                        matchesTarget = true;
                        edge[2] += target - newDistance;
                    }
                }
            }

            return matchesTarget ? edges : new int[0][0];
        }

        long runDijkstra(
                int[][] edges,
                int n,
                int source,
                int destination) {
            long[][] adjMatrix = new long[n][n];
            long[] minDistance = new long[n];
            boolean[] visited = new boolean[n];

            Arrays.fill(minDistance, INF);
            for (long[] row : adjMatrix) {
                Arrays.fill(row, INF);
            }

            minDistance[source] = 0;

            for (int[] edge : edges) {
                if (edge[2] != -1) {
                    adjMatrix[edge[0]][edge[1]] = edge[2];
                    adjMatrix[edge[1]][edge[0]] = edge[2];
                }
            }

            for (int i = 0; i < n; i++) {
                int nearestUnvisitedNode = -1;
                for (int j = 0; j < n; j++) {
                    if (!visited[j]
                            && (nearestUnvisitedNode == -1 || minDistance[j] < minDistance[nearestUnvisitedNode])) {
                        nearestUnvisitedNode = j;
                    }
                }
                visited[nearestUnvisitedNode] = true;

                for (int v = 0; v < n; v++) {
                    minDistance[v] = Math.min(
                            minDistance[v],
                            minDistance[nearestUnvisitedNode] +
                                    adjMatrix[nearestUnvisitedNode][v]);
                }
            }

            return minDistance[destination];
        }
    }

    static class Solution2 {
        List<int[]>[] graph;
        static final int INF = (int) 2e9;

        public int[][] modifiedGraphEdges(int n, int[][] edges, int source, int destination, int target) {
            graph = new ArrayList[n];
            for (int i = 0; i < n; i++) {
                graph[i] = new ArrayList<>();
            }
            for (int[] edge : edges) {
                if (edge[2] != -1) {
                    graph[edge[0]].add(new int[] { edge[1], edge[2] });
                    graph[edge[1]].add(new int[] { edge[0], edge[2] });
                }
            }
            int currentShortestDistance = runDijkstra(n, source, destination);
            if (currentShortestDistance < target) {
                return new int[0][0];
            }

            boolean matchesTarget = (currentShortestDistance == target);
            for (int[] edge : edges) {
                if (edge[2] != -1)
                    continue;
                edge[2] = matchesTarget ? INF : 1;
                graph[edge[0]].add(new int[] { edge[1], edge[2] });
                graph[edge[1]].add(new int[] { edge[0], edge[2] });

                if (!matchesTarget) {
                    int newDistance = runDijkstra(n, source, destination);
                    if (newDistance <= target) {
                        matchesTarget = true;
                        edge[2] += target - newDistance;
                    }
                }
            }
            return matchesTarget ? edges : new int[0][0];
        }

        int runDijkstra(int n, int source, int destination) {
            int[] minDistance = new int[n];
            PriorityQueue<int[]> minHeap = new PriorityQueue<>(
                    (a, b) -> a[1] - b[1]);
            Arrays.fill(minDistance, INF);
            minDistance[source] = 0;
            minHeap.add(new int[] { source, 0 });

            while (!minHeap.isEmpty()) {
                int[] curr = minHeap.poll();
                int u = curr[0];
                int d = curr[1];

                if (d > minDistance[u])
                    continue;

                for (int[] neighbor : graph[u]) {
                    int v = neighbor[0];
                    int weight = neighbor[1];

                    if (d + weight < minDistance[v]) {
                        minDistance[v] = d + weight;
                        minHeap.add(new int[] { v, minDistance[v] });
                    }
                }
            }

            return minDistance[destination];
        }
    }

    // https://leetcode.com/problems/number-of-distinct-islands/description/
    static class Solution3 {
        int[][] grid;
        boolean[][] visited;
        StringBuffer currentIsland;

        public int numDistinctIslands(int[][] grid) {
            this.grid = grid;
            this.visited = new boolean[grid.length][grid[0].length];
            Set<String> islands = new HashSet<>();
            for (int row = 0; row < grid.length; row++) {
                for (int col = 0; col < grid[0].length; col++) {
                    currentIsland = new StringBuffer();
                    dfs(row, col, '0');
                    if (currentIsland.length() == 0) {
                        continue;
                    }
                    islands.add(currentIsland.toString());
                }
            }
            return islands.size();
        }

        void dfs(int row, int col, char dir) {
            if (row < 0 || col < 0 || row >= grid.length || col >= grid[0].length) {
                return;
            }
            if (visited[row][col] || grid[row][col] == 0) {
                return;
            }
            visited[row][col] = true;
            currentIsland.append(dir);
            dfs(row + 1, col, 'D');
            dfs(row - 1, col, 'U');
            dfs(row, col + 1, 'R');
            dfs(row, col - 1, 'L');
            currentIsland.append('0');
        }
    }
}
