package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Queue;

public class Day919 {
    // https://leetcode.com/problems/largest-color-value-in-a-directed-graph/description/?envType=daily-question&envId=2025-05-26
    static class Solution1 {
        public int largestPathValue(String colors, int[][] edges) {
            int n = colors.length();
            Map<Integer, List<Integer>> adj = new HashMap<>();
            int[] indegree = new int[n];

            for (int[] edge : edges) {
                adj.computeIfAbsent(edge[0], k -> new ArrayList<Integer>()).add(edge[1]);
                indegree[edge[1]]++;
            }

            int[][] count = new int[n][26];
            Queue<Integer> q = new LinkedList<>();

            for (int i = 0; i < n; i++) {
                if (indegree[i] == 0) {
                    q.offer(i);
                }
            }

            int answer = 1, nodeSeen = 0;
            while (!q.isEmpty()) {
                int node = q.poll();
                answer = Math.max(answer, ++count[node][colors.charAt(node) - 'a']);
                nodeSeen++;

                if (!adj.containsKey(node)) {
                    continue;
                }

                for (int neighbor : adj.get(node)) {
                    for (int i = 0; i < 26; i++) {
                        count[neighbor][i] = Math.max(count[neighbor][i], count[node][i]);
                    }

                    indegree[neighbor]--;
                    if (indegree[neighbor] == 0) {
                        q.offer(neighbor);
                    }
                }
            }

            return nodeSeen < n ? -1 : answer;
        }
    }

    static class Solution2 {
        int dfs(int node, String colors, Map<Integer, List<Integer>> adj, int[][] count, boolean[] visit,
                boolean[] inStack) {
            if (inStack[node]) {
                return Integer.MAX_VALUE;
            }

            if (visit[node]) {
                return count[node][colors.charAt(node) - 'a'];
            }
            visit[node] = true;
            inStack[node] = true;

            if (adj.containsKey(node)) {
                for (int neighbor : adj.get(node)) {
                    if (dfs(neighbor, colors, adj, count, visit, inStack) == Integer.MAX_VALUE) {
                        return Integer.MAX_VALUE;
                    }
                    for (int i = 0; i < 26; i++) {
                        count[node][i] = Math.max(count[node][i], count[neighbor][i]);
                    }
                }
            }

            count[node][colors.charAt(node) - 'a']++;
            inStack[node] = false;
            return count[node][colors.charAt(node) - 'a'];
        }

        public int largestPathValue(String colors, int[][] edges) {
            int n = colors.length();
            Map<Integer, List<Integer>> adj = new HashMap<>();
            for (int[] edge : edges) {
                adj.computeIfAbsent(edge[0], k -> new ArrayList<Integer>()).add(edge[1]);
            }

            int[][] count = new int[n][26];
            boolean[] visit = new boolean[n];
            boolean[] inStack = new boolean[n];
            int answer = 0;
            for (int i = 0; i < n; i++) {
                answer = Math.max(answer, dfs(i, colors, adj, count, visit, inStack));
            }
            return answer == Integer.MAX_VALUE ? -1 : answer;
        }
    }

}
