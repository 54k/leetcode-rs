package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.Queue;
import java.util.Set;
import java.util.TreeMap;

public class Day1126 {
    // https://leetcode.com/problems/find-all-people-with-secret/description/?envType=daily-question&envId=2025-12-19
    static class Solution1 {
        public List<Integer> findAllPeople(int n, int[][] meetings, int firstPerson) {
            Map<Integer, List<int[]>> graph = new HashMap<>();
            for (int[] meeting : meetings) {
                int x = meeting[0], y = meeting[1], t = meeting[2];
                graph.computeIfAbsent(x, k -> new ArrayList<>()).add(new int[] { t, y });
                graph.computeIfAbsent(y, k -> new ArrayList<>()).add(new int[] { t, x });
            }

            PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> a[0] - b[0]);
            pq.offer(new int[] { 0, 0 });
            pq.offer(new int[] { 0, firstPerson });

            boolean[] visited = new boolean[n];

            while (!pq.isEmpty()) {
                int[] timePerson = pq.poll();
                int time = timePerson[0], person = timePerson[1];
                if (visited[person]) {
                    continue;
                }
                visited[person] = true;

                for (int[] nextPersonTime : graph.getOrDefault(person, new ArrayList<>())) {
                    int t = nextPersonTime[0], nextPerson = nextPersonTime[1];
                    if (!visited[nextPerson] && t >= time) {
                        pq.offer(new int[] { t, nextPerson });
                    }
                }
            }

            List<Integer> ans = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                if (visited[i]) {
                    ans.add(i);
                }
            }
            return ans;
        }
    }

    static class Solution2 {
        public List<Integer> findAllPeople(int n, int[][] meetings, int firstPerson) {
            Arrays.sort(meetings, (a, b) -> a[2] - b[2]);
            Map<Integer, List<int[]>> sameTimeMeetings = new TreeMap<>();
            for (int[] meeting : meetings) {
                int x = meeting[0], y = meeting[1], t = meeting[2];
                sameTimeMeetings.computeIfAbsent(t, k -> new ArrayList<>()).add(new int[] { x, y });
            }

            boolean[] knowsSecret = new boolean[n];
            knowsSecret[0] = true;
            knowsSecret[firstPerson] = true;

            for (int t : sameTimeMeetings.keySet()) {
                Map<Integer, List<Integer>> meet = new HashMap<>();
                for (int[] meeting : sameTimeMeetings.get(t)) {
                    int x = meeting[0], y = meeting[1];
                    meet.computeIfAbsent(x, k -> new ArrayList<>()).add(y);
                    meet.computeIfAbsent(y, k -> new ArrayList<>()).add(x);
                }

                Set<Integer> start = new HashSet<>();
                for (int[] meeting : sameTimeMeetings.get(t)) {
                    int x = meeting[0], y = meeting[1];
                    if (knowsSecret[x]) {
                        start.add(x);
                    }
                    if (knowsSecret[y]) {
                        start.add(y);
                    }
                }

                Queue<Integer> q = new LinkedList<>(start);
                while (!q.isEmpty()) {
                    int person = q.poll();
                    for (int nextPerson : meet.getOrDefault(person, new ArrayList<>())) {
                        if (!knowsSecret[nextPerson]) {
                            knowsSecret[nextPerson] = true;
                            q.offer(nextPerson);
                        }
                    }
                }
            }

            List<Integer> ans = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                if (knowsSecret[i]) {
                    ans.add(i);
                }
            }
            return ans;
        }
    }

    static class Solution3 {
        static class UnionFind {
            private int[] parent;
            private int[] rank;

            UnionFind(int n) {
                parent = new int[n];
                rank = new int[n];
                for (int i = 0; i < n; i++) {
                    parent[i] = i;
                }
            }

            int find(int x) {
                if (parent[x] != x) {
                    parent[x] = find(parent[x]);
                }
                return parent[x];
            }

            void unite(int x, int y) {
                int px = find(x);
                int py = find(y);
                if (px != py) {
                    if (rank[px] > rank[py]) {
                        parent[py] = px;
                    } else if (rank[px] < rank[py]) {
                        parent[px] = py;
                    } else {
                        parent[py] = px;
                        rank[px] += 1;
                    }
                }
            }

            boolean connected(int x, int y) {
                return find(x) == find(y);
            }

            void reset(int x) {
                parent[x] = x;
                rank[x] = 0;
            }
        }

        public List<Integer> findAllPeople(int n, int[][] meetings, int firstPerson) {
            Arrays.sort(meetings, (a, b) -> a[2] - b[2]);
            Map<Integer, List<int[]>> sameTimeMeetings = new TreeMap<>();
            for (int[] meeting : meetings) {
                int x = meeting[0], y = meeting[1], t = meeting[2];
                sameTimeMeetings.computeIfAbsent(t, k -> new ArrayList<>()).add(new int[] { x, y });
            }

            UnionFind graph = new UnionFind(n);
            graph.unite(firstPerson, 0);

            for (int t : sameTimeMeetings.keySet()) {
                for (int[] meeting : sameTimeMeetings.get(t)) {
                    int x = meeting[0], y = meeting[1];
                    graph.unite(x, y);
                }

                for (int[] meeting : sameTimeMeetings.get(t)) {
                    int x = meeting[0], y = meeting[1];
                    if (!graph.connected(x, 0)) {
                        graph.reset(x);
                        graph.reset(y);
                    }
                }
            }

            List<Integer> ans = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                if (graph.connected(i, 0)) {
                    ans.add(i);
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/reachable-nodes-in-subdivided-graph/description/
    static class Solution4 {
        static class ANode {
            int node, dist;

            ANode(int n, int d) {
                node = n;
                dist = d;
            }
        }

        public int reachableNodes(int[][] edges, int M, int N) {
            Map<Integer, Map<Integer, Integer>> graph = new HashMap<>();
            for (int[] edge : edges) {
                int u = edge[0], v = edge[1], w = edge[2];
                graph.computeIfAbsent(u, x -> new HashMap<>()).put(v, w);
                graph.computeIfAbsent(v, x -> new HashMap<>()).put(u, w);
            }

            PriorityQueue<ANode> pq = new PriorityQueue<>((a, b) -> Integer.compare(a.dist, b.dist));
            pq.offer(new ANode(0, 0));

            Map<Integer, Integer> dist = new HashMap<>();
            dist.put(0, 0);
            Map<Integer, Integer> used = new HashMap<>();
            int ans = 0;

            while (!pq.isEmpty()) {
                ANode anode = pq.poll();
                int node = anode.node;
                int d = anode.dist;

                if (d > dist.getOrDefault(node, 0))
                    continue;
                ans++;

                if (!graph.containsKey(node))
                    continue;

                for (int nei : graph.get(node).keySet()) {
                    int weight = graph.get(node).get(nei);
                    int v = Math.min(weight, M - d);
                    used.put(N * node + nei, v);

                    int d2 = d + weight + 1;
                    if (d2 < dist.getOrDefault(nei, M + 1)) {
                        pq.offer(new ANode(nei, d2));
                        dist.put(nei, d2);
                    }
                }
            }

            for (int[] edge : edges) {
                ans += Math.min(edge[2],
                        used.getOrDefault(edge[0] * N + edge[1], 0) + used.getOrDefault(edge[1] * N + edge[0], 0));
            }
            return ans;
        }
    }
}
