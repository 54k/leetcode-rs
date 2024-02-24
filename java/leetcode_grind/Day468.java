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
import java.util.Stack;

public class Day468 {
    // https://leetcode.com/problems/find-all-people-with-secret/
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
                int[] timeAndPerson = pq.poll();
                int time = timeAndPerson[0], person = timeAndPerson[1];
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
            var ppl = new HashSet<Integer>();
            ppl.add(0);
            ppl.add(firstPerson);
            Arrays.sort(meetings, (a, b) -> a[2] - b[2]);

            var prev = -1;

            for (int i = 0; i < meetings.length; i++) {
                var meets = new HashMap<Integer, List<Integer>>();
                var m = meetings[i];
                if (prev != m[2]) {
                    prev = m[2];
                    meets.computeIfAbsent(m[0], x -> new ArrayList<>()).add(m[1]);
                    meets.computeIfAbsent(m[1], x -> new ArrayList<>()).add(m[0]);
                    while (i < meetings.length - 1 && meetings[i + 1][2] == prev) {
                        i++;
                        meets.computeIfAbsent(meetings[i][0], x -> new ArrayList<>()).add(meetings[i][1]);
                        meets.computeIfAbsent(meetings[i][1], x -> new ArrayList<>()).add(meetings[i][0]);
                    }
                }

                var stack = new Stack<>();
                for (var p : meets.keySet()) {
                    if (ppl.contains(p))
                        stack.push(p);
                }

                while (!stack.isEmpty()) {
                    var pop = stack.pop();
                    for (var next : meets.getOrDefault(pop, new ArrayList<>())) {
                        if (!ppl.contains(next)) {
                            ppl.add(next);
                            stack.push(next);
                        }
                    }
                }
            }

            return ppl.stream().toList();
        }
    }

    static class Solution3 {
        public int networkDelayTime(int[][] times, int n, int k) {
            var INF = 1000_000_000;
            Map<Integer, List<int[]>> adj = new HashMap<>();
            for (var t : times) {
                adj.computeIfAbsent(t[0] - 1, (x) -> new ArrayList<>()).add(new int[] { t[1] - 1, t[2] });
            }

            // PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> a[0] - b[0]);
            Queue<int[]> pq = new LinkedList<>();
            int[] dist = new int[n];
            Arrays.fill(dist, INF);
            dist[--k] = 0;
            pq.add(new int[] { 0, k });

            while (!pq.isEmpty()) {
                var p = pq.poll();
                var d = p[0];
                var v = p[1];
                for (var nxt : adj.getOrDefault(v, new ArrayList<>())) {
                    var u = nxt[0];
                    var w = nxt[1];
                    if (dist[u] > w + d) {
                        dist[u] = w + d;
                        pq.add(new int[] { dist[u], u });
                    }
                }
            }
            int mx = 0;
            for (var d : dist) {
                if (d == INF) {
                    return -1;
                }
                mx = Math.max(mx, d);
            }
            return mx;
        }
    }
}
