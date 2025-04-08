package leetcode_grind;

public class Day871 {
    // https://leetcode.com/problems/loud-and-rich/description/
    static class Solution1 {
        List<Integer>[] graph;
        int[] answer;
        int[] quiet;

        public int[] loudAndRich(int[][] richer, int[] quiet) {
            int N = quiet.length;     
            graph = new ArrayList[N];
            answer = new int[N];
            this.quiet = quiet;

            for (int node = 0; node < N; node++) {
                graph[node] = new ArrayList<>();
            }
            for (int[] edge : richer) {
                graph[edge[1]].add(edge[0]);
            }

            Arrays.fill(answer, -1);
            for (int node = 0; node < N; node++) {
                dfs(node);
            }
            return answer;
        }

        int dfs(int node) {
            if (answer[node] == -1) {
                answer[node] = node;
                for (int child : graph[node]) {
                    int cand = dfs(child);
                    if (quiet[cand] < quiet[answer[node]]) {
                        answer[node] = cand;
                    }
                }
            }
            return answer[node];
        }
    }
}