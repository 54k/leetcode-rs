package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.Queue;
import java.util.Set;
import java.util.Stack;

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

    static class Solution3 {
        public List<Boolean> checkIfPrerequisite(int numCourses, int[][] prerequisites, int[][] queries) {
            Map<Integer, List<Integer>> adjList = new HashMap<>();
            int[] indegree = new int[numCourses];

            for (int[] edge : prerequisites) {
                adjList.computeIfAbsent(edge[0], k -> new ArrayList<>()).add(edge[1]);
                indegree[edge[1]]++;
            }

            Queue<Integer> q = new LinkedList<>();
            for (int i = 0; i < numCourses; i++) {
                if (indegree[i] == 0) {
                    q.offer(i);
                }
            }

            Map<Integer, Set<Integer>> nodePrerequisites = new HashMap<>();
            while (!q.isEmpty()) {
                int node = q.poll();
                for (int adj : adjList.getOrDefault(node, new ArrayList<>())) {
                    nodePrerequisites.computeIfAbsent(adj, k -> new HashSet<>()).add(node);
                    for (int prereq : nodePrerequisites.getOrDefault(node, new HashSet<>())) {
                        nodePrerequisites.get(adj).add(prereq);
                    }
                    indegree[adj]--;
                    if (indegree[adj] == 0) {
                        q.offer(adj);
                    }
                }
            }

            List<Boolean> answer = new ArrayList<>();
            for (int[] query : queries) {
                answer.add(nodePrerequisites.getOrDefault(query[1], new HashSet<>()).contains(query[0]));
            }
            return answer;
        }
    }

    static class Solution4 {
        public List<Boolean> checkIfPrerequisite(int numCourses, int[][] prerequisites, int[][] queries) {
            boolean[][] isPrerequisite = new boolean[numCourses][numCourses];
            for (int[] edge : prerequisites) {
                isPrerequisite[edge[0]][edge[1]] = true;
            }

            for (int intermediate = 0; intermediate < numCourses; intermediate++) {
                for (int src = 0; src < numCourses; src++) {
                    for (int target = 0; target < numCourses; target++) {
                        isPrerequisite[src][target] = isPrerequisite[src][target] ||
                                (isPrerequisite[src][intermediate] && isPrerequisite[intermediate][target]);
                    }
                }
            }

            List<Boolean> answer = new ArrayList<>();
            for (int[] query : queries) {
                answer.add(isPrerequisite[query[0]][query[1]]);
            }
            return answer;
        }
    }

    // https://leetcode.com/problems/binary-tree-pruning/description/
    static public class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;

        TreeNode() {
        }

        TreeNode(int val) {
            this.val = val;
        }

        TreeNode(int val, TreeNode left, TreeNode right) {
            this.val = val;
            this.left = left;
            this.right = right;
        }
    }

    static class Solution5 {
        public TreeNode pruneTree(TreeNode root) {
            return containsOne(root) ? root : null;
        }

        boolean containsOne(TreeNode node) {
            if (node == null)
                return false;
            boolean leftContainsOne = containsOne(node.left);
            boolean rightContainsOne = containsOne(node.right);
            if (!leftContainsOne)
                node.left = null;
            if (!rightContainsOne)
                node.right = null;
            return node.val == 1 || leftContainsOne || rightContainsOne;
        }
    }

    // https://leetcode.com/problems/swim-in-rising-water/description/
    static class Solution6 {
        public int swimInWater(int[][] grid) {
            int N = grid.length;

            Set<Integer> seen = new HashSet<>();
            PriorityQueue<Integer> pq = new PriorityQueue<>((k1, k2) -> {
                return grid[k1 / N][k1 % N] - grid[k2 / N][k2 % N];
            });
            pq.offer(0);
            int ans = 0;

            int[] dr = new int[] { 1, -1, 0, 0 };
            int[] dc = new int[] { 0, 0, 1, -1 };

            while (!pq.isEmpty()) {
                int k = pq.poll();
                int r = k / N, c = k % N;
                ans = Math.max(ans, grid[r][c]);
                if (r == N - 1 && c == N - 1)
                    return ans;

                for (int i = 0; i < 4; i++) {
                    int cr = r + dr[i], cc = c + dc[i];
                    int ck = cr * N + cc;
                    if (0 <= cr && cr < N && 0 <= cc && cc < N && !seen.contains(ck)) {
                        pq.offer(ck);
                        seen.add(ck);
                    }
                }
            }

            throw null;
        }
    }

    static class Solution7 {
        public int swimInWater(int[][] grid) {
            int N = grid.length;
            int lo = grid[0][0], hi = N * N;
            while (lo < hi) {
                int mi = lo + (hi - lo) / 2;
                if (!possible(mi, grid)) {
                    lo = mi + 1;
                } else {
                    hi = mi;
                }
            }
            return lo;
        }

        boolean possible(int T, int[][] grid) {
            int N = grid.length;
            Set<Integer> seen = new HashSet<>();
            seen.add(0);

            int[] dr = new int[] { 1, -1, 0, 0 };
            int[] dc = new int[] { 0, 0, 1, -1 };

            Stack<Integer> stack = new Stack<>();
            stack.add(0);

            while (!stack.isEmpty()) {
                int k = stack.pop();
                int r = k / N, c = k % N;
                if (r == N - 1 && c == N - 1)
                    return true;

                for (int i = 0; i < 4; i++) {
                    int cr = r + dr[i], cc = c + dc[i];
                    int ck = cr * N + cc;
                    if (0 <= cr && cr < N && 0 <= cc && cc < N && !seen.contains(ck) && grid[cr][cc] <= T) {
                        stack.add(ck);
                        seen.add(ck);
                    }
                }
            }

            return false;
        }
    }
}
