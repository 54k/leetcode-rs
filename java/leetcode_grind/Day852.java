package leetcode_grind;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;
import java.util.Queue;

public class Day852 {
    // https://leetcode.com/problems/minimum-cost-walk-in-weighted-graph/description/?envType=daily-question&envId=2025-03-20
    static class Solution1 {
        int[] parent;
        int[] depth;

        public int[] minimumCost(int n, int[][] edges, int[][] queries) {
            parent = new int[n];
            for (int i = 0; i < n; i++)
                parent[i] = -1;
            depth = new int[n];

            int[] componentCost = new int[n];
            for (int i = 0; i < n; i++) {
                componentCost[i] = Integer.MAX_VALUE;
            }

            for (int[] edge : edges) {
                union(edge[0], edge[1]);
            }

            for (int[] edge : edges) {
                int root = find(edge[0]);
                componentCost[root] &= edge[2];
            }

            int[] answer = new int[queries.length];
            for (int i = 0; i < queries.length; i++) {
                int start = queries[i][0];
                int end = queries[i][1];

                if (find(start) != find(end)) {
                    answer[i] = -1;
                } else {
                    int root = find(start);
                    answer[i] = componentCost[root];
                }
            }

            return answer;
        }

        int find(int node) {
            if (parent[node] == -1)
                return node;
            return parent[node] = find(parent[node]);
        }

        void union(int node1, int node2) {
            int root1 = find(node1);
            int root2 = find(node2);

            if (root1 == root2)
                return;

            if (depth[root1] < depth[root2]) {
                int temp = root1;
                root1 = root2;
                root2 = temp;
            }

            parent[root2] = root1;
            if (depth[root1] == depth[root2]) {
                depth[root1]++;
            }
        }
    }

    static class Solution2 {
        public int[] minimumCost(int n, int[][] edges, int[][] query) {
            List<List<int[]>> adjList = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                adjList.add(new ArrayList<>(2));
            }

            for (int[] edge : edges) {
                adjList.get(edge[0]).add(new int[] { edge[1], edge[2] });
                adjList.get(edge[1]).add(new int[] { edge[0], edge[2] });
            }

            boolean[] visited = new boolean[n];
            int[] components = new int[n];
            List<Integer> componentCost = new ArrayList<>();
            int componentId = 0;
            for (int node = 0; node < n; node++) {
                if (!visited[node]) {
                    componentCost.add(
                            getComponentCost(
                                    node, adjList, visited, components, componentId));
                    componentId++;
                }
            }

            int[] answer = new int[query.length];
            for (int i = 0; i < query.length; i++) {
                int start = query[i][0];
                int end = query[i][1];

                if (components[start] == components[end]) {
                    answer[i] = componentCost.get(components[start]);
                } else {
                    answer[i] = -1;
                }
            }

            return answer;
        }

        int getComponentCost(int source, List<List<int[]>> adjList, boolean[] visited, int[] components,
                int componentId) {
            Queue<Integer> nodesQueue = new LinkedList<>();
            int componentCost = Integer.MAX_VALUE;
            nodesQueue.offer(source);
            visited[source] = true;

            while (!nodesQueue.isEmpty()) {
                int node = nodesQueue.poll();
                components[node] = componentId;

                for (int[] neighbor : adjList.get(node)) {
                    int weight = neighbor[1];
                    componentCost &= weight;

                    if (visited[neighbor[0]])
                        continue;

                    visited[neighbor[0]] = true;
                    nodesQueue.offer(neighbor[0]);
                }
            }
            return componentCost;
        }
    }
}
