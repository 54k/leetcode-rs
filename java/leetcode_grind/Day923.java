package leetcode_grind;

import java.util.Arrays;
import java.util.LinkedList;
import java.util.Queue;

public class Day923 {
    // https://leetcode.com/problems/find-closest-node-to-given-two-nodes/description/?envType=daily-question&envId=2025-05-30
    static class Solution1 {
        void bfs(int startNode, int[] edges, int[] dist) {
            int n = edges.length;
            Queue<Integer> q = new LinkedList<>();
            q.offer(startNode);

            Boolean[] visit = new Boolean[n];
            Arrays.fill(visit, Boolean.FALSE);
            dist[startNode] = 0;

            while (!q.isEmpty()) {
                int node = q.poll();

                if (visit[node]) {
                    continue;
                }

                visit[node] = true;
                int neighbor = edges[node];
                if (neighbor != -1 && !visit[neighbor]) {
                    dist[neighbor] = 1 + dist[node];
                    q.offer(neighbor);
                }
            }
        }

        public int closestMeetingNode(int[] edges, int node1, int node2) {
            int n = edges.length;
            int[] dist1 = new int[n], dist2 = new int[n];
            Arrays.fill(dist1, Integer.MAX_VALUE);
            Arrays.fill(dist2, Integer.MAX_VALUE);

            bfs(node1, edges, dist1);
            bfs(node2, edges, dist2);

            int minDistNode = -1, minDistTillNow = Integer.MAX_VALUE;
            for (int currNode = 0; currNode < n; currNode++) {
                if (minDistTillNow > Math.max(dist1[currNode], dist2[currNode])) {
                    minDistNode = currNode;
                    minDistTillNow = Math.max(dist1[currNode], dist2[currNode]);
                }
            }
            return minDistNode;
        }
    }

    static class Solution2 {
        void dfs(int node, int[] edges, int[] dist, Boolean[] visit) {
            visit[node] = true;
            int neighbor = edges[node];
            if (neighbor != -1 && !visit[neighbor]) {
                dist[neighbor] = 1 + dist[node];
                dfs(neighbor, edges, dist, visit);
            }
        }

        public int closestMeetingNode(int[] edges, int node1, int node2) {
            int n = edges.length;
            int[] dist1 = new int[n], dist2 = new int[n];
            Arrays.fill(dist1, Integer.MAX_VALUE);
            Arrays.fill(dist2, Integer.MAX_VALUE);
            dist1[node1] = 0;
            dist2[node2] = 0;

            Boolean[] visit1 = new Boolean[n], visit2 = new Boolean[n];
            Arrays.fill(visit1, Boolean.FALSE);
            Arrays.fill(visit2, Boolean.FALSE);

            dfs(node1, edges, dist1, visit1);
            dfs(node2, edges, dist2, visit2);

            int minDistNode = -1, minDistTillNow = Integer.MAX_VALUE;
            for (int currNode = 0; currNode < n; currNode++) {
                if (minDistTillNow > Math.max(dist1[currNode], dist2[currNode])) {
                    minDistNode = currNode;
                    minDistTillNow = Math.max(dist1[currNode], dist2[currNode]);
                }
            }
            return minDistNode;
        }
    }

}
