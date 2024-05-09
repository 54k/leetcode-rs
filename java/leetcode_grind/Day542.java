package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.List;
import java.util.PriorityQueue;
import java.util.Queue;

public class Day542 {
    static class Pair<F, S> {
        F key;
        S value;

        Pair(F f1, S s1) {
            key = f1;
            value = s1;
        }

        F getKey() {
            return key;
        }

        S getValue() {
            return value;
        }
    }

    // https://leetcode.com/problems/minimum-cost-to-buy-apples/description
    static class Solution1 {
        public long[] minCost(int n, int[][] roads, int[] appleCost, int k) {
            List<List<Pair<Integer, Integer>>> graph = new ArrayList<>();

            for (int i = 0; i < n; i++) {
                graph.add(new ArrayList<>());
            }

            for (int[] road : roads) {
                int cityA = road[0] - 1, cityB = road[1] - 1, cost = road[2];
                graph.get(cityA).add(new Pair<Integer, Integer>(cityB, cost));
                graph.get(cityB).add(new Pair<Integer, Integer>(cityA, cost));
            }

            long[] result = new long[n];
            for (int startCity = 0; startCity < n; startCity++) {
                result[startCity] = shortestPath(startCity, graph, appleCost, k, n);
            }
            return result;
        }

        private long shortestPath(int startCity, List<List<Pair<Integer, Integer>>> graph, int[] appleCost, int k,
                int n) {
            int[] travelCosts = new int[n];
            Arrays.fill(travelCosts, Integer.MAX_VALUE);
            travelCosts[startCity] = 0;

            Queue<int[]> heap = new PriorityQueue<>(Comparator.comparingInt(a -> a[0]));
            heap.offer(new int[] { 0, startCity });
            long minCost = Integer.MAX_VALUE;

            while (!heap.isEmpty()) {
                int[] current = heap.poll();
                int travelCost = current[0];
                int currCity = current[1];

                minCost = Math.min(minCost, (long) appleCost[currCity] + (k + 1) * travelCost);

                for (Pair<Integer, Integer> next : graph.get(currCity)) {
                    int neighbor = next.getKey();
                    int nextCost = travelCost + next.getValue();
                    if (nextCost < travelCosts[neighbor]) {
                        travelCosts[neighbor] = nextCost;
                        heap.offer(new int[] { nextCost, neighbor });
                    }
                }
            }
            return minCost;
        }
    }
}
