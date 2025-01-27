package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Queue;

public class Day800 {
    // https://leetcode.com/problems/course-schedule-iv/description/?envType=daily-question&envId=2025-01-27
    static class Solution1 {
        boolean isPrerequisite(
                Map<Integer, List<Integer>> adjList,
                boolean[] visited,
                int src,
                int target) {
            visited[src] = true;
            if (src == target) {
                return true;
            }
            boolean answer = false;
            List<Integer> neighbors = adjList.getOrDefault(src, new ArrayList<>());
            for (int adj : neighbors) {
                if (!visited[adj]) {
                    answer = answer || isPrerequisite(adjList, visited, adj, target);
                }
            }
            return answer;
        }

        public List<Boolean> checkIfPrerequisite(int numCourses, int[][] prerequisites, int[][] queries) {
            Map<Integer, List<Integer>> adjList = new HashMap<>();
            for (int[] edge : prerequisites) {
                adjList.computeIfAbsent(edge[0], k -> new ArrayList<>()).add(edge[1]);
            }

            List<Boolean> result = new ArrayList<>();
            for (int i = 0; i < queries.length; i++) {
                boolean[] visited = new boolean[numCourses];
                result.add(isPrerequisite(adjList, visited, queries[i][0], queries[i][1]));
            }
            return result;
        }
    }

    static class Solution2 {
        void preprocess(int numCourses, int[][] prerequisites, Map<Integer, List<Integer>> adjList,
                boolean[][] isPrerequisite) {
            for (int i = 0; i < numCourses; i++) {
                Queue<Integer> q = new LinkedList<>();
                q.offer(i);
                while (!q.isEmpty()) {
                    int node = q.poll();
                    for (int adj : adjList.getOrDefault(node, new ArrayList<>())) {
                        if (!isPrerequisite[i][adj]) {
                            isPrerequisite[i][adj] = true;
                            q.offer(adj);
                        }
                    }
                }
            }
        }

        public List<Boolean> checkIfPrerequisite(int numCourses, int[][] prerequisites, int[][] queries) {
            Map<Integer, List<Integer>> adjList = new HashMap<>();
            for (int[] edge : prerequisites) {
                adjList.computeIfAbsent(edge[0], k -> new ArrayList<>()).add(edge[1]);
            }
            boolean[][] isPrerequisite = new boolean[numCourses][numCourses];
            preprocess(numCourses, prerequisites, adjList, isPrerequisite);

            List<Boolean> answer = new ArrayList<>();
            for (int[] query : queries) {
                answer.add(isPrerequisite[query[0]][query[1]]);
            }
            return answer;
        }
    }

}
