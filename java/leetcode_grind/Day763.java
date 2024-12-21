package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Queue;
import java.util.Set;

public class Day763 {
    // https://leetcode.com/problems/maximum-number-of-k-divisible-components/description/?envType=daily-question&envId=2024-12-21
    static class Solution1 {
        public int maxKDivisibleComponents(int n, int[][] edges, int[] values, int k) {
            List<Integer>[] adjList = new ArrayList[n];
            for (int i = 0; i < n; i++) {
                adjList[i] = new ArrayList<>();
            }
            for (int[] edge : edges) {
                int node1 = edge[0];
                int node2 = edge[1];
                adjList[node1].add(node2);
                adjList[node2].add(node1);
            }
            int[] componentCount = new int[1];
            dfs(0, -1, adjList, values, k, componentCount);
            return componentCount[0];
        }

        int dfs(
                int currentNode,
                int parentNode,
                List<Integer>[] adjList,
                int[] nodeValues,
                int k,
                int[] componentCount) {
            int sum = 0;
            for (int neighborNode : adjList[currentNode]) {
                if (neighborNode != parentNode) {
                    sum += dfs(neighborNode, currentNode, adjList, nodeValues, k, componentCount);
                    sum %= k;
                }
            }
            sum += nodeValues[currentNode];
            sum %= k;
            if (sum % k == 0) {
                componentCount[0]++;
            }
            return sum;
        }
    }

    static class Solution2 {
        public int maxKDivisibleComponents(int n, int[][] edges, int[] values, int k) {
            if (n < 2)
                return 1;
            int componentCount = 0;
            Map<Integer, Set<Integer>> graph = new HashMap<>();
            for (int[] edge : edges) {
                int node1 = edge[0], node2 = edge[1];
                graph.computeIfAbsent(node1, key -> new HashSet<>()).add(node2);
                graph.computeIfAbsent(node2, key -> new HashSet<>()).add(node1);
            }

            long[] longValues = new long[values.length];
            for (int i = 0; i < values.length; i++) {
                longValues[i] = values[i];
            }

            Queue<Integer> queue = new LinkedList<>();
            for (Map.Entry<Integer, Set<Integer>> entry : graph.entrySet()) {
                if (entry.getValue().size() == 1) {
                    queue.add(entry.getKey());
                }
            }

            while (!queue.isEmpty()) {
                int currentNode = queue.poll();
                int neighborNode = -1;
                if (graph.get(currentNode) != null && !graph.get(currentNode).isEmpty()) {
                    neighborNode = graph.get(currentNode).iterator().next();
                }

                if (neighborNode >= 0) {
                    graph.get(neighborNode).remove(currentNode);
                    graph.get(currentNode).remove(neighborNode);
                }

                if (longValues[currentNode] % k == 0) {
                    componentCount++;
                } else if (neighborNode >= 0) {
                    longValues[neighborNode] += longValues[currentNode];
                }

                if (neighborNode >= 0 && graph.get(neighborNode) != null && graph.get(neighborNode).size() == 1) {
                    queue.add(neighborNode);
                }
            }

            return componentCount;
        }
    }
}
