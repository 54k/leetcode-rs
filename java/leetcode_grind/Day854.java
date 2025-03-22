package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Queue;
import java.util.Set;

public class Day854 {

    // https://leetcode.com/problems/count-the-number-of-complete-components/description/?envType=daily-question&envId=2025-03-22
    static class Solution1 {
        public int countCompleteComponents(int n, int[][] edges) {
            List<Integer>[] graph = new ArrayList[n];
            Map<List<Integer>, Integer> componentFreq = new HashMap<>();

            for (int vertex = 0; vertex < n; vertex++) {
                graph[vertex] = new ArrayList<>();
                graph[vertex].add(vertex);
            }

            for (int[] edge : edges) {
                graph[edge[0]].add(edge[1]);
                graph[edge[1]].add(edge[0]);
            }

            for (int vertex = 0; vertex < n; vertex++) {
                List<Integer> neighbors = graph[vertex];
                Collections.sort(neighbors);
                componentFreq.put(neighbors, componentFreq.getOrDefault(neighbors, 0) + 1);
            }

            int completeCount = 0;
            for (Map.Entry<List<Integer>, Integer> entry : componentFreq.entrySet()) {
                if (entry.getKey().size() == entry.getValue()) {
                    completeCount++;
                }
            }
            return completeCount;
        }
    }

    static class Solution2 {
        public int countCompleteComponents(int n, int[][] edges) {
            List<Integer>[] graph = new ArrayList[n];
            Map<List<Integer>, Integer> componentFreq = new HashMap<>();

            for (int vertex = 0; vertex < n; vertex++) {
                graph[vertex] = new ArrayList<>();
            }

            for (int[] edge : edges) {
                graph[edge[0]].add(edge[1]);
                graph[edge[1]].add(edge[0]);
            }

            int completeCount = 0;
            Set<Integer> visited = new HashSet<>();

            for (int vertex = 0; vertex < n; vertex++) {
                if (visited.contains(vertex))
                    continue;

                int[] componentInfo = new int[2];
                dfs(vertex, graph, visited, componentInfo);
                if (componentInfo[0] * (componentInfo[0] - 1) == componentInfo[1]) {
                    completeCount++;
                }
            }
            return completeCount;
        }

        void dfs(int curr, List<Integer>[] graph, Set<Integer> visited, int[] componentInfo) {
            visited.add(curr);
            componentInfo[0]++;
            componentInfo[1] += graph[curr].size();

            for (int next : graph[curr]) {
                if (!visited.contains(next)) {
                    dfs(next, graph, visited, componentInfo);
                }
            }
        }
    }

    static class Solution3 {
        public int countCompleteComponents(int n, int[][] edges) {
            List<Integer>[] graph = new ArrayList[n];

            for (int vertex = 0; vertex < n; vertex++) {
                graph[vertex] = new ArrayList<>();
            }

            for (int[] edge : edges) {
                graph[edge[0]].add(edge[1]);
                graph[edge[1]].add(edge[0]);
            }
            boolean[] visited = new boolean[n];
            int completeComponents = 0;

            for (int vertex = 0; vertex < n; vertex++) {
                if (!visited[vertex]) {
                    List<Integer> component = new ArrayList<>();
                    Queue<Integer> queue = new LinkedList<>();
                    queue.add(vertex);
                    visited[vertex] = true;

                    while (!queue.isEmpty()) {
                        int current = queue.poll();
                        component.add(current);

                        for (int neighbor : graph[current]) {
                            if (!visited[neighbor]) {
                                queue.add(neighbor);
                                visited[neighbor] = true;
                            }
                        }
                    }

                    boolean isComplete = true;
                    for (int node : component) {
                        if (graph[node].size() != component.size() - 1) {
                            isComplete = false;
                            break;
                        }
                    }

                    if (isComplete) {
                        completeComponents++;
                    }
                }
            }

            return completeComponents;
        }
    }

    static class Solution4 {
        public int countCompleteComponents(int n, int[][] edges) {
            UnionFind dsu = new UnionFind(n);
            Map<Integer, Integer> edgeCount = new HashMap<>();

            for (int[] edge : edges) {
                dsu.union(edge[0], edge[1]);
            }

            for (int[] edge : edges) {
                int root = dsu.find(edge[0]);
                edgeCount.put(root, edgeCount.getOrDefault(root, 0) + 1);
            }

            int completeCount = 0;
            for (int vertex = 0; vertex < n; vertex++) {
                if (dsu.find(vertex) == vertex) {
                    int nodeCount = dsu.size[vertex];
                    int expectedEdges = (nodeCount * (nodeCount - 1)) / 2;
                    if (edgeCount.getOrDefault(vertex, 0) == expectedEdges) {
                        completeCount++;
                    }
                }
            }
            return completeCount;
        }

        static class UnionFind {
            int[] parent;
            int[] size;

            UnionFind(int n) {
                parent = new int[n];
                size = new int[n];
                Arrays.fill(parent, -1);
                Arrays.fill(size, 1);
            }

            int find(int node) {
                if (parent[node] == -1) {
                    return node;
                }
                return parent[node] = find(parent[node]);
            }

            void union(int node1, int node2) {
                int root1 = find(node1);
                int root2 = find(node2);

                if (root1 == root2) {
                    return;
                }

                if (size[root1] > size[root2]) {
                    parent[root2] = root1;
                    size[root1] += size[root2];
                } else {
                    parent[root1] = root2;
                    size[root2] += size[root1];
                }
            }
        }
    }
}
