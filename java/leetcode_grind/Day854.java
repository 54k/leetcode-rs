package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;
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

    // https://leetcode.com/problems/find-the-closest-marked-node/description/?envType=weekly-question&envId=2025-03-22
    static class Solution5 {
        public int minimumDistance(int n, List<List<Integer>> edges, int s, int[] marked) {
            Set<Integer> markSet = new HashSet<>();
            for (int node : marked) {
                markSet.add(node);
            }

            List<List<int[]>> adj = new ArrayList<>();
            for (int i = 0; i < n; i++)
                adj.add(new ArrayList<>());
            for (List<Integer> edge : edges) {
                adj.get(edge.get(0)).add(new int[] { edge.get(1), edge.get(2) });
            }

            int[] dist = new int[n];
            Arrays.fill(dist, Integer.MAX_VALUE);
            dist[s] = 0;

            PriorityQueue<int[]> minHeap = new PriorityQueue<>(
                    (a, b) -> a[0] - b[0]);
            minHeap.offer(new int[] { 0, s });

            while (!minHeap.isEmpty()) {
                int[] current = minHeap.poll();
                int node = current[1];
                int distance = current[0];

                if (markSet.contains(node)) {
                    return dist[node];
                }

                for (int[] edge : adj.get(node)) {
                    int nextNode = edge[0];
                    int weight = edge[1];
                    int newDist = distance + weight;

                    if (newDist < dist[nextNode]) {
                        dist[nextNode] = newDist;
                        minHeap.offer(new int[] { newDist, nextNode });
                    }
                }
            }

            return -1;
        }
    }

    static class Solution6 {
        public int minimumDistance(int n, List<List<Integer>> edges, int s, int[] marked) {
            int[] dist = new int[n];
            Arrays.fill(dist, Integer.MAX_VALUE);
            dist[s] = 0;

            for (int iter = 0; iter < n - 1; iter++) {
                for (List<Integer> edge : edges) {
                    int from = edge.get(0);
                    int to = edge.get(1);
                    int weight = edge.get(2);

                    if (dist[from] != Integer.MAX_VALUE && dist[from] + weight < dist[to]) {
                        dist[to] = dist[from] + weight;
                    }
                }
            }

            int minDist = Integer.MAX_VALUE;
            for (int node : marked) {
                if (dist[node] < minDist) {
                    minDist = dist[node];
                }
            }

            return minDist == Integer.MAX_VALUE ? -1 : minDist;
        }
    }

    static class Solution7 {
        public int minimumDistance(int n, List<List<Integer>> edges, int s, int[] marked) {
            List<List<int[]>> graph = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                graph.add(new ArrayList<>());
            }

            for (List<Integer> edge : edges) {
                int from = edge.get(0);
                int to = edge.get(1);
                int weight = edge.get(2);
                graph.get(from).add(new int[] { to, weight });
            }

            int[] dist = new int[n];
            Arrays.fill(dist, Integer.MAX_VALUE);
            dist[s] = 0;

            Queue<Integer> queue = new LinkedList<>();
            queue.offer(s);

            boolean[] inQueue = new boolean[n];
            inQueue[s] = true;

            while (!queue.isEmpty()) {
                int current = queue.poll();
                inQueue[current] = false;

                for (int[] next : graph.get(current)) {
                    int nextNode = next[0];
                    int weight = next[1];

                    if (dist[nextNode] > dist[current] + weight) {
                        dist[nextNode] = dist[current] + weight;

                        if (!inQueue[nextNode]) {
                            queue.offer(nextNode);
                            inQueue[nextNode] = true;
                        }
                    }
                }
            }

            int minDist = Integer.MAX_VALUE;
            for (int node : marked) {
                minDist = Math.min(minDist, dist[node]);
            }
            return minDist == Integer.MAX_VALUE ? -1 : minDist;
        }
    }

    // https://leetcode.com/problems/burst-balloons/description/
    static class Solution8 {
        public int maxCoins(int[] nums) {
            int n = nums.length + 2;
            int[] newNums = new int[n];
            System.arraycopy(nums, 0, newNums, 1, n - 2);
            newNums[0] = 1;
            newNums[n - 1] = 1;
            int[][] memo = new int[n][n];
            return dp(memo, newNums, 1, n - 2);
        }

        int dp(int[][] memo, int[] nums, int left, int right) {
            if (right - left < 0) {
                return 0;
            }
            if (memo[left][right] > 0) {
                return memo[left][right];
            }

            int result = 0;
            for (int i = left; i <= right; i++) {
                int gain = nums[left - 1] * nums[i] * nums[right + 1];
                int remaining = dp(memo, nums, left, i - 1) + dp(memo, nums, i + 1, right);
                result = Math.max(result, remaining + gain);
            }
            memo[left][right] = result;
            return result;
        }
    }

    static class Solution9 {
        public int maxCoins(int[] nums) {
            int n = nums.length + 2;
            int[] newNums = new int[n];
            System.arraycopy(nums, 0, newNums, 1, n - 2);
            newNums[0] = 1;
            newNums[n - 1] = 1;
            int[][] dp = new int[n][n];

            for (int left = n - 2; left >= 1; left--) {
                for (int right = left; right <= n - 2; right++) {
                    for (int i = left; i <= right; i++) {
                        int gain = newNums[left - 1] * newNums[i] * newNums[right + 1];
                        int remaining = dp[left][i - 1] + dp[i + 1][right];
                        dp[left][right] = Math.max(remaining + gain, dp[left][right]);
                    }
                }
            }

            return dp[1][n - 2];
        }
    }
}
