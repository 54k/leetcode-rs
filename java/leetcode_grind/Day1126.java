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

}
