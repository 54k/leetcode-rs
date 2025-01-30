package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Queue;
import java.util.Set;
import java.util.Stack;

public class Day803 {
    // https://leetcode.com/problems/divide-nodes-into-the-maximum-number-of-groups/description/?envType=daily-question&envId=2025-01-30
    static class Solution1 {
        public int magnificentSets(int n, int[][] edges) {
            List<List<Integer>> adjList = new ArrayList<>(n);
            for (int i = 0; i < n; i++) {
                adjList.add(new ArrayList<>());
            }
            for (int[] edge : edges) {
                adjList.get(edge[0] - 1).add(edge[1] - 1);
                adjList.get(edge[1] - 1).add(edge[0] - 1);
            }

            int[] colors = new int[n];
            Arrays.fill(colors, -1);
            for (int node = 0; node < n; node++) {
                if (colors[node] != -1)
                    continue;
                colors[node] = 0;
                if (!isBipartite(adjList, node, colors))
                    return -1;
            }

            int[] distances = new int[n];
            for (int node = 0; node < n; node++) {
                distances[node] = getLongestShortestPath(adjList, node, n);
            }

            int maxNumberOfGroups = 0;
            boolean[] visited = new boolean[n];
            for (int node = 0; node < n; node++) {
                if (visited[node])
                    continue;
                maxNumberOfGroups += getNumberOfGroupsForComponent(adjList, node, distances, visited);
            }
            return maxNumberOfGroups;
        }

        boolean isBipartite(List<List<Integer>> adjList, int node, int[] colors) {
            for (int neighbor : adjList.get(node)) {
                if (colors[neighbor] == colors[node])
                    return false;
                if (colors[neighbor] != -1)
                    continue;
                colors[neighbor] = (colors[node] + 1) % 2;
                if (!isBipartite(adjList, neighbor, colors))
                    return false;
            }
            return true;
        }

        int getLongestShortestPath(List<List<Integer>> adjList, int srcNode, int n) {
            Queue<Integer> nodesQueue = new LinkedList<>();
            boolean[] visited = new boolean[n];
            nodesQueue.add(srcNode);
            visited[srcNode] = true;
            int distance = 0;

            while (!nodesQueue.isEmpty()) {
                int numOfNodesInLayer = nodesQueue.size();
                for (int i = 0; i < numOfNodesInLayer; i++) {
                    int currentNode = nodesQueue.poll();
                    for (int neighbor : adjList.get(currentNode)) {
                        if (visited[neighbor])
                            continue;
                        visited[neighbor] = true;
                        nodesQueue.add(neighbor);
                    }
                }
                distance++;
            }
            return distance;
        }

        int getNumberOfGroupsForComponent(List<List<Integer>> adjList, int node, int[] distances, boolean[] visited) {
            int maxNumberOfGroups = distances[node];
            visited[node] = true;
            for (int neighbor : adjList.get(node)) {
                if (visited[neighbor])
                    continue;
                maxNumberOfGroups = Math.max(maxNumberOfGroups,
                        getNumberOfGroupsForComponent(adjList, neighbor, distances, visited));
            }
            return maxNumberOfGroups;
        }
    }

    static class Solution2 {
        public int magnificentSets(int n, int[][] edges) {
            List<List<Integer>> adjList = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                adjList.add(new ArrayList<>());
            }

            int[] parent = new int[n];
            int[] depth = new int[n];
            Arrays.fill(parent, -1);

            for (int[] edge : edges) {
                adjList.get(edge[0] - 1).add(edge[1] - 1);
                adjList.get(edge[1] - 1).add(edge[0] - 1);
                union(edge[0] - 1, edge[1] - 1, parent, depth);
            }

            Map<Integer, Integer> numOfGroupsForComponent = new HashMap<>();
            for (int node = 0; node < n; node++) {
                int numberOfGroups = getNumberOfGroups(adjList, node, n);
                if (numberOfGroups == -1)
                    return -1;
                int rootNode = find(node, parent);
                numOfGroupsForComponent.put(rootNode,
                        Math.max(numOfGroupsForComponent.getOrDefault(rootNode, 0), numberOfGroups));
            }

            int totalNumberOfGroups = 0;
            for (int numberOfGroups : numOfGroupsForComponent.values()) {
                totalNumberOfGroups += numberOfGroups;
            }
            return totalNumberOfGroups;
        }

        int getNumberOfGroups(List<List<Integer>> adjList, int srcNode, int n) {
            Queue<Integer> nodesQueue = new LinkedList<>();
            int[] layerSeen = new int[n];
            Arrays.fill(layerSeen, -1);
            nodesQueue.offer(srcNode);
            layerSeen[srcNode] = 0;
            int deepestLayer = 0;

            while (!nodesQueue.isEmpty()) {
                int numOfNodesInLayer = nodesQueue.size();
                for (int i = 0; i < numOfNodesInLayer; i++) {
                    int currentNode = nodesQueue.poll();
                    for (int neighbor : adjList.get(currentNode)) {
                        if (layerSeen[neighbor] == -1) {
                            layerSeen[neighbor] = deepestLayer + 1;
                            nodesQueue.offer(neighbor);
                        } else {
                            if (layerSeen[neighbor] == deepestLayer) {
                                return -1;
                            }
                        }
                    }
                }
                deepestLayer++;
            }
            return deepestLayer;
        }

        int find(int node, int[] parent) {
            while (parent[node] != -1)
                node = parent[node];
            return node;
        }

        void union(int node1, int node2, int[] parent, int[] depth) {
            node1 = find(node1, parent);
            node2 = find(node2, parent);
            if (node1 == node2)
                return;
            if (depth[node1] < depth[node2]) {
                int temp = node1;
                node1 = node2;
                node2 = temp;
            }
            parent[node2] = node1;
            if (depth[node1] == depth[node2])
                depth[node1]++;
        }
    }

    // https://leetcode.com/problems/graph-valid-tree/description/?envType=weekly-question&envId=2025-01-29
    static class Solution3 {
        List<List<Integer>> adjacencyList = new ArrayList<>();
        Set<Integer> seen = new HashSet<>();

        public boolean validTree(int n, int[][] edges) {
            if (edges.length != n - 1)
                return false;
            for (int i = 0; i < n; i++) {
                adjacencyList.add(new ArrayList<>());
            }
            for (int[] edge : edges) {
                adjacencyList.get(edge[0]).add(edge[1]);
                adjacencyList.get(edge[1]).add(edge[0]);
            }
            return dfs(0, -1) && seen.size() == n;
        }

        boolean dfs(int node, int parent) {
            if (seen.contains(node))
                return false;
            seen.add(node);
            for (int neighbour : adjacencyList.get(node)) {
                if (parent != neighbour) {
                    boolean result = dfs(neighbour, node);
                    if (!result)
                        return false;
                }
            }
            return true;
        }
    }

    static class Solution4 {
        public boolean validTree(int n, int[][] edges) {
            if (edges.length != n - 1)
                return false;
            List<List<Integer>> adjacencyList = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                adjacencyList.add(new ArrayList<>());
            }
            for (int[] edge : edges) {
                adjacencyList.get(edge[0]).add(edge[1]);
                adjacencyList.get(edge[1]).add(edge[0]);
            }

            Stack<Integer> stack = new Stack<>();
            Set<Integer> seen = new HashSet<>();
            stack.push(0);
            seen.add(0);

            while (!stack.isEmpty()) {
                int node = stack.pop();
                for (int neighbor : adjacencyList.get(node)) {
                    if (seen.contains(neighbor))
                        continue;
                    seen.add(neighbor);
                    stack.push(neighbor);
                }
            }

            return seen.size() == n;
        }
    }
}
