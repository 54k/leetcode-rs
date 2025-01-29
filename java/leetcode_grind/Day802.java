package leetcode_grind;

import java.util.*;

public class Day802 {
    // https://leetcode.com/problems/redundant-connection/description/?envType=daily-question&envId=2025-01-29
    static class Solution1 {
        boolean isConnected(int src, int target, boolean[] visited, List<Integer>[] adjList) {
            visited[src] = true;
            if (src == target) {
                return true;
            }
            boolean isFound = false;
            for (int adj : adjList[src]) {
                if (!visited[adj]) {
                    isFound = isFound || isConnected(adj, target, visited, adjList);
                }
            }
            return isFound;
        }

        public int[] findRedundantConnection(int[][] edges) {
            int N = edges.length;
            List<Integer>[] adjList = new ArrayList[N];
            for (int i = 0; i < N; i++) {
                adjList[i] = new ArrayList<>();
            }
            for (int[] edge : edges) {
                boolean[] visited = new boolean[N];
                if (isConnected(edge[0] - 1, edge[1] - 1, visited, adjList)) {
                    return new int[] { edge[0], edge[1] };
                }

                adjList[edge[0] - 1].add(edge[1] - 1);
                adjList[edge[1] - 1].add(edge[0] - 1);
            }

            return new int[] {};
        }
    }

    static class Solution2 {
        int cycleStart = -1;

        void DFS(int src, boolean[] visited, List<Integer>[] adjList, int[] parent) {
            visited[src] = true;
            for (int adj : adjList[src]) {
                if (!visited[adj]) {
                    parent[adj] = src;
                    DFS(adj, visited, adjList, parent);
                } else if (adj != parent[src] && cycleStart == -1) {
                    cycleStart = adj;
                    parent[adj] = src;
                }
            }
        }

        public int[] findRedundantConnection(int[][] edges) {
            int N = edges.length;
            boolean[] visited = new boolean[N];
            int[] parent = new int[N];
            Arrays.fill(parent, -1);

            List<Integer>[] adjList = new ArrayList[N];
            for (int i = 0; i < N; i++) {
                adjList[i] = new ArrayList<>();
            }
            for (int[] edge : edges) {
                adjList[edge[0] - 1].add(edge[1] - 1);
                adjList[edge[1] - 1].add(edge[0] - 1);
            }
            DFS(0, visited, adjList, parent);

            Map<Integer, Integer> cycleNodes = new HashMap<>();
            int node = cycleStart;
            do {
                cycleNodes.put(node, 1);
                node = parent[node];
            } while (node != cycleStart);

            for (int i = edges.length - 1; i >= 0; i--) {
                if (cycleNodes.containsKey(edges[i][0] - 1) && cycleNodes.containsKey(edges[i][1] - 1)) {
                    return edges[i];
                }
            }

            return new int[] {};
        }
    }

    static class Solution3 {
        class DSU {
            int N;
            int[] size;
            int[] representative;

            DSU(int N) {
                this.N = N;
                size = new int[N];
                representative = new int[N];
                for (int node = 0; node < N; node++) {
                    size[node] = 1;
                    representative[node] = node;
                }
            }

            int find(int node) {
                if (representative[node] == node) {
                    return node;
                }
                return representative[node] = find(representative[node]);
            }

            boolean doUnion(int nodeOne, int nodeTwo) {
                nodeOne = find(nodeOne);
                nodeTwo = find(nodeTwo);

                if (nodeOne == nodeTwo) {
                    return false;
                }

                if (size[nodeOne] > size[nodeTwo]) {
                    representative[nodeTwo] = nodeOne;
                    size[nodeOne] += size[nodeTwo];
                } else {
                    representative[nodeOne] = nodeTwo;
                    size[nodeTwo] += size[nodeOne];
                }
                return true;
            }
        }

        public int[] findRedundantConnection(int[][] edges) {
            int N = edges.length;
            DSU dsu = new DSU(N);
            for (int[] edge : edges) {
                if (!dsu.doUnion(edge[0] - 1, edge[1] - 1)) {
                    return edge;
                }
            }
            return new int[] {};
        }
    }
}
