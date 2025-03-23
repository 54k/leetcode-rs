package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.List;
import java.util.PriorityQueue;

public class Day855 {
    // https://leetcode.com/problems/number-of-ways-to-arrive-at-destination/description/?envType=daily-question&envId=2025-03-23
    static class Solution1 {
        public int countPaths(int n, int[][] roads) {
            int MOD = 1_000_000_007;
            List<List<int[]>> graph = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                graph.add(new ArrayList<>());
            }
            for (int[] road : roads) {
                int startNode = road[0], endNode = road[1], travelTime = road[2];
                graph.get(startNode).add(new int[] { endNode, travelTime });
                graph.get(endNode).add(new int[] { startNode, travelTime });
            }

            PriorityQueue<long[]> minHeap = new PriorityQueue<>(
                    Comparator.comparingLong(a -> a[0]));

            long[] shortestTime = new long[n];
            Arrays.fill(shortestTime, Long.MAX_VALUE);
            int[] pathCount = new int[n];

            shortestTime[0] = 0;
            pathCount[0] = 1;

            minHeap.offer(new long[] { 0, 0 });

            while (!minHeap.isEmpty()) {
                long[] top = minHeap.poll();
                long currTime = top[0];
                int currNode = (int) top[1];

                if (currTime > shortestTime[currNode]) {
                    continue;
                }

                for (int[] neighbor : graph.get(currNode)) {
                    int neighborNode = neighbor[0], roadTime = neighbor[1];

                    if (currTime + roadTime < shortestTime[neighborNode]) {
                        shortestTime[neighborNode] = currTime + roadTime;
                        pathCount[neighborNode] = pathCount[currNode];
                        minHeap.offer(new long[] { shortestTime[neighborNode], neighborNode });
                    } else if (currTime + roadTime == shortestTime[neighborNode]) {
                        pathCount[neighborNode] = (pathCount[neighborNode] + pathCount[currNode]) % MOD;
                    }
                }
            }

            return pathCount[n - 1];
        }
    }

    static class Solution2 {
        static final int MOD = 1_000_000_007;

        public int countPaths(int n, int[][] roads) {
            long[][][] dp = new long[n][n][2];
            for (int src = 0; src < n; src++) {
                for (int dest = 0; dest < n; dest++) {
                    if (src != dest) {
                        dp[src][dest][0] = (long) 1e12;
                        dp[src][dest][1] = 0;
                    } else {
                        dp[src][dest][0] = 0;
                        dp[src][dest][1] = 1;
                    }
                }
            }

            for (int[] road : roads) {
                int startNode = road[0], endNode = road[1], travelTime = road[2];
                dp[startNode][endNode][0] = travelTime;
                dp[endNode][startNode][0] = travelTime;
                dp[startNode][endNode][1] = 1;
                dp[endNode][startNode][1] = 1;
            }

            for (int mid = 0; mid < n; mid++) {
                for (int src = 0; src < n; src++) {
                    for (int dest = 0; dest < n; dest++) {
                        if (src != mid && dest != mid) {
                            long newTime = dp[src][mid][0] + dp[mid][dest][0];

                            if (newTime < dp[src][dest][0]) {
                                dp[src][dest][0] = newTime;
                                dp[src][dest][1] = (dp[src][mid][1] * dp[mid][dest][1]) % MOD;
                            } else if (newTime == dp[src][dest][0]) {
                                dp[src][dest][1] = (dp[src][dest][1] + dp[src][mid][1] * dp[mid][dest][1]) % MOD;
                            }
                        }
                    }
                }
            }

            return (int) dp[n - 1][0][1];
        }
    }
}