package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.PriorityQueue;

public class Day617 {
    // https://leetcode.com/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/description/?envType=daily-question&envId=2024-07-26
    static class Solution1 {
        public int findTheCity(int n, int[][] edges, int distanceThreshold) {
            List<int[]>[] adjacencyList = new List[n];
            int[][] shortestPathMatrix = new int[n][n];

            for (int i = 0; i < n; i++) {
                Arrays.fill(shortestPathMatrix[i], Integer.MAX_VALUE);
                shortestPathMatrix[i][i] = 0;
                adjacencyList[i] = new ArrayList<>();
            }

            for (int[] edge : edges) {
                int start = edge[0];
                int end = edge[1];
                int weight = edge[2];

                adjacencyList[start].add(new int[] { end, weight });
                adjacencyList[end].add(new int[] { start, weight });
            }

            for (int i = 0; i < n; i++) {
                dijkstra(n, adjacencyList, shortestPathMatrix[i], i);
            }

            return getCityWithFewestReachable(n, shortestPathMatrix, distanceThreshold);
        }

        void dijkstra(int n, List<int[]>[] adjacencyList, int[] shortestPathDistances, int source) {
            PriorityQueue<int[]> priorityQueue = new PriorityQueue<>((a, b) -> (a[1] - b[1]));
            priorityQueue.add(new int[] { source, 0 });
            Arrays.fill(shortestPathDistances, Integer.MAX_VALUE);
            shortestPathDistances[source] = 0;

            while (!priorityQueue.isEmpty()) {
                int[] current = priorityQueue.remove();
                int currentCity = current[0];
                int currentDistance = current[1];
                if (currentDistance > shortestPathDistances[currentCity]) {
                    continue;
                }

                for (int[] neighbor : adjacencyList[currentCity]) {
                    int neighborCity = neighbor[0];
                    int edgeWeight = neighbor[1];

                    if (shortestPathDistances[neighborCity] > currentDistance + edgeWeight) {
                        shortestPathDistances[neighborCity] = currentDistance + edgeWeight;
                        priorityQueue.add(new int[] { neighborCity, shortestPathDistances[neighborCity] });
                    }
                }
            }
        }

        int getCityWithFewestReachable(int n, int[][] shortestPathMatrix, int distanceThreshold) {
            int cityWithFewestReachable = -1;
            int fewestReachableCount = n;

            for (int i = 0; i < n; i++) {
                int reachableCount = 0;
                for (int j = 0; j < n; j++) {
                    if (i == j) {
                        continue;
                    }

                    if (shortestPathMatrix[i][j] <= distanceThreshold) {
                        reachableCount++;
                    }
                }

                if (reachableCount <= fewestReachableCount) {
                    fewestReachableCount = reachableCount;
                    cityWithFewestReachable = i;
                }
            }
            return cityWithFewestReachable;
        }
    }
}
