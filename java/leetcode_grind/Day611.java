package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class Day611 {

    // https://leetcode.com/problems/build-a-matrix-with-conditions/description/?envType=daily-question&envId=2024-07-21
    static class Solution1 {
        public int[][] buildMatrix(int k, int[][] rowConditions, int[][] colConditions) {
            List<Integer> orderRows = topoSort(rowConditions, k);
            List<Integer> orderColumns = topoSort(colConditions, k);

            if (orderRows.isEmpty() || orderColumns.isEmpty())
                return new int[0][0];

            int[][] matrix = new int[k][k];
            for (int i = 0; i < k; i++) {
                for (int j = 0; j < k; j++) {
                    if (orderRows.get(i).equals(orderColumns.get(j))) {
                        matrix[i][j] = orderRows.get(i);
                    }
                }
            }
            return matrix;
        }

        private List<Integer> topoSort(int[][] edges, int n) {
            List<List<Integer>> adj = new ArrayList<>();
            for (int i = 0; i <= n; i++) {
                adj.add(new ArrayList<>());
            }
            for (int[] edge : edges) {
                adj.get(edge[0]).add(edge[1]);
            }

            List<Integer> order = new ArrayList<>();
            int[] visited = new int[n + 1];
            boolean[] hasCycle = { false };

            for (int i = 1; i <= n; i++) {
                if (visited[i] == 0) {
                    dfs(i, adj, visited, order, hasCycle);
                    if (hasCycle[0])
                        return new ArrayList<>();
                }
            }

            Collections.reverse(order);
            return order;
        }

        private void dfs(int node, List<List<Integer>> adj, int[] visited, List<Integer> order, boolean[] hasCycle) {
            visited[node] = 1;
            for (int neighbor : adj.get(node)) {
                if (visited[neighbor] == 0) {
                    dfs(neighbor, adj, visited, order, hasCycle);
                    if (hasCycle[0])
                        return;
                } else if (visited[neighbor] == 1) {
                    hasCycle[0] = true;
                    return;
                }
            }
            visited[node] = 2;
            order.add(node);
        }
    }
}
