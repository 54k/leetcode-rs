package leetcode_grind;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;
import java.util.Queue;

public class Day797 {
    // https://leetcode.com/problems/find-eventual-safe-states/description/?envType=daily-question&envId=2025-01-24
    static class Solution1 {
        public List<Integer> eventualSafeNodes(int[][] graph) {
            int n = graph.length;
            int[] indegree = new int[n];
            List<List<Integer>> adj = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                adj.add(new ArrayList<>());
            }
            for (int i = 0; i < n; i++) {
                for (int node : graph[i]) {
                    adj.get(node).add(i);
                    indegree[i]++;
                }
            }

            Queue<Integer> q = new LinkedList<>();
            for (int i = 0; i < n; i++) {
                if (indegree[i] == 0) {
                    q.add(i);
                }
            }

            boolean[] safe = new boolean[n];
            while (!q.isEmpty()) {
                int node = q.poll();
                safe[node] = true;

                for (int neighbor : adj.get(node)) {
                    indegree[neighbor]--;
                    if (indegree[neighbor] == 0) {
                        q.add(neighbor);
                    }
                }
            }

            List<Integer> safeNodes = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                if (safe[i]) {
                    safeNodes.add(i);
                }
            }
            return safeNodes;
        }
    }

    static class Solution2 {
        boolean dfs(int node, int[][] adj, boolean[] visit, boolean[] inStack) {
            if (inStack[node]) {
                return true;
            }
            if (visit[node]) {
                return false;
            }
            visit[node] = true;
            inStack[node] = true;
            for (int neighbor : adj[node]) {
                if (dfs(neighbor, adj, visit, inStack)) {
                    return true;
                }
            }

            inStack[node] = false;
            return false;
        }

        public List<Integer> eventualSafeNodes(int[][] graph) {
            int n = graph.length;
            boolean[] visit = new boolean[n];
            boolean[] inStack = new boolean[n];
            for (int i = 0; i < n; i++) {
                dfs(i, graph, visit, inStack);
            }

            List<Integer> safeNodes = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                if (!inStack[i]) {
                    safeNodes.add(i);
                }
            }
            return safeNodes;
        }
    }
}
