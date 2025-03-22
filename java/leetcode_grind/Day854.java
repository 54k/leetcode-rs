package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
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

}
