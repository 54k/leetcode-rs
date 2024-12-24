package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.PriorityQueue;
import java.util.Queue;
import java.util.Set;

public class Day766 {
    // https://leetcode.com/problems/maximum-star-sum-of-a-graph/description/
    static class Solution1 {
        public int maxStarSum(int[] vals, int[][] edges, int k) {
            var n = vals.length;
            var adj = new HashMap<Integer, List<Integer>>();

            for (int v = 0; v < n; v++) {
                adj.put(v, new ArrayList<>());
            }
            for (var edge : edges) {
                if (vals[edge[1]] > 0) {
                    adj.get(edge[0]).add(vals[edge[1]]);
                }
                if (vals[edge[0]] > 0) {
                    adj.get(edge[1]).add(vals[edge[0]]);
                }
            }

            var ans = Integer.MIN_VALUE;
            for (int v = 0; v < n; v++) {
                var pq = new PriorityQueue<Integer>();
                for (var u : adj.get(v)) {
                    if (pq.size() < k) {
                        pq.add(u);
                    } else if (!pq.isEmpty() && pq.peek() < u) {
                        pq.poll();
                        pq.add(u);
                    }
                }
                var sum = 0;
                while (!pq.isEmpty()) {
                    sum += pq.poll();
                }

                ans = Math.max(ans, vals[v] + sum);
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/find-minimum-diameter-after-merging-two-trees/description/?envType=daily-question&envId=2024-12-24
    static class Solution2 {
        public int minimumDiameterAfterMerge(int[][] edges1, int[][] edges2) {
            int n = edges1.length + 1;
            int m = edges2.length + 1;

            List<List<Integer>> adjList1 = buildAdjList(n, edges1);
            List<List<Integer>> adjList2 = buildAdjList(m, edges2);

            int diameter1 = findDiameter(n, adjList1);
            int diameter2 = findDiameter(m, adjList2);

            int combinedDiameter = (int) Math.ceil(diameter1 / 2.0) + (int) Math.ceil(diameter2 / 2.0) + 1;
            return Math.max(Math.max(diameter1, diameter2), combinedDiameter);
        }

        List<List<Integer>> buildAdjList(int size, int[][] edges) {
            List<List<Integer>> adjList = new ArrayList<>();
            for (int i = 0; i < size; i++) {
                adjList.add(new ArrayList<>());
            }
            for (int[] edge : edges) {
                adjList.get(edge[0]).add(edge[1]);
                adjList.get(edge[1]).add(edge[0]);
            }
            return adjList;
        }

        int findDiameter(int n, List<List<Integer>> adjList) {
            Pair firstBFS = findFarthestNode(n, adjList, 0);
            int farthestNode = firstBFS.getFirst();
            Pair secondBFS = findFarthestNode(n, adjList, farthestNode);
            return secondBFS.getSecond();
        }

        Pair findFarthestNode(int n, List<List<Integer>> adjList, int sourceNode) {
            Queue<Integer> queue = new LinkedList<>();
            boolean[] visited = new boolean[n];
            queue.add(sourceNode);
            visited[sourceNode] = true;

            int maximumDistance = 0, farthestNode = sourceNode;
            while (!queue.isEmpty()) {
                int size = queue.size();
                for (int i = 0; i < size; i++) {
                    int currentNode = queue.poll();

                    farthestNode = currentNode;

                    for (int neighbor : adjList.get(currentNode)) {
                        if (!visited[neighbor]) {
                            visited[neighbor] = true;
                            queue.add(neighbor);
                        }
                    }
                }
                if (!queue.isEmpty())
                    maximumDistance++;
            }
            return new Pair(farthestNode, maximumDistance);
        }

        // Simple Pair class for storing results of the findDiameter function
        class Pair {
            private int first;
            private int second;

            public Pair(int first, int second) {
                this.first = first;
                this.second = second;
            }

            public int getFirst() {
                return first;
            }

            public int getSecond() {
                return second;
            }
        }
    }

    static class Solution3 {
        public int minimumDiameterAfterMerge(int[][] edges1, int[][] edges2) {
            int n = edges1.length + 1;
            int m = edges2.length + 1;

            List<List<Integer>> adjList1 = buildAdjList(n, edges1);
            List<List<Integer>> adjList2 = buildAdjList(m, edges2);

            int diameter1 = findDiameter(adjList1, 0, -1).getFirst();
            int diameter2 = findDiameter(adjList2, 0, -1).getFirst();

            int combinedDiameter = (int) Math.ceil(diameter1 / 2.0) + (int) Math.ceil(diameter2 / 2.0) + 1;
            return Math.max(Math.max(diameter1, diameter2), combinedDiameter);
        }

        List<List<Integer>> buildAdjList(int size, int[][] edges) {
            List<List<Integer>> adjList = new ArrayList<>();
            for (int i = 0; i < size; i++) {
                adjList.add(new ArrayList<>());
            }
            for (int[] edge : edges) {
                adjList.get(edge[0]).add(edge[1]);
                adjList.get(edge[1]).add(edge[0]);
            }
            return adjList;
        }

        Pair findDiameter(List<List<Integer>> adjList, int node, int parent) {
            int maxDepth1 = 0, maxDepth2 = 0;
            int diameter = 0;

            for (int neighbor : adjList.get(node)) {
                if (neighbor == parent)
                    continue;
                Pair result = findDiameter(adjList, neighbor, node);

                int childDiameter = result.getFirst();
                int depth = result.getSecond() + 1;

                diameter = Math.max(diameter, childDiameter);

                if (depth > maxDepth1) {
                    maxDepth2 = maxDepth1;
                    maxDepth1 = depth;
                } else if (depth > maxDepth2) {
                    maxDepth2 = depth;
                }
            }

            diameter = Math.max(diameter, maxDepth1 + maxDepth2);
            return new Pair(diameter, maxDepth1);
        }

        // Simple Pair class for storing results of the findDiameter function
        class Pair {
            private int first;
            private int second;

            public Pair(int first, int second) {
                this.first = first;
                this.second = second;
            }

            public int getFirst() {
                return first;
            }

            public int getSecond() {
                return second;
            }
        }
    }

    static class Solution4 {
        public int minimumDiameterAfterMerge(int[][] edges1, int[][] edges2) {
            var find = new Object() {
                int apply(int[][] edges) {
                    var adj = new HashMap<Integer, Set<Integer>>();
                    for (int i = 0; i < edges.length; i++) {
                        adj.put(i, new Hashet<>());
                    }
                    for (var e : edges) {
                        adj.computeIfAbsent(e[0], x -> new HashSet<>()).add(e[1]);
                        adj.computeIfAbsent(e[1], x -> new HashSet<>()).add(e[0]);
                    }

                    int diam = 0;
                    Queue<Integer> queue = new LinkedList<>();
                    for (var e : adj.entrySet()) {
                        if (adj.get(e.getKey()).size() == 1) {
                            queue.add(e.getKey());
                        }
                    }

                    int rem = edges.length + 1;
                    while (rem > 2) {
                        diam++;
                        var s = queue.size();
                        rem -= s;
                        while (s-- > 0) {
                            var u = queue.poll();
                            for (var v : adj.get(u)) {
                                adj.get(v).remove(u);
                                if (adj.get(v).size() == 1) {
                                    queue.add(v);
                                }
                            }
                        }
                    }
                    return rem == 2 ? diam * 2 + 1 : diam * 2;
                }
            };

            var d1 = find.apply(edges1);
            var d2 = find.apply(edges2);
            var comb = (int) Math.ceil(d1 / 2.0) + 1 + (int) Math.ceil(d2 / 2.0);
            return Math.max(comb, Math.max(d1, d2));
        }
    }
}
