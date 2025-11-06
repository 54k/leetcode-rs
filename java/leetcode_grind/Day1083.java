package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;

public class Day1083 {
    // https://leetcode.com/problems/power-grid-maintenance/description/?envType=daily-question&envId=2025-11-06
    static class Solution1 {
        static class DSU {
            int[] parent;

            DSU(int size) {
                parent = new int[size];
                for (int i = 0; i < size; i++) {
                    parent[i] = i;
                }
            }

            int find(int x) {
                return parent[x] == x ? x : (parent[x] = find(parent[x]));
            }

            void join(int u, int v) {
                parent[find(v)] = find(u);
            }
        }

        public int[] processQueries(int c, int[][] connections, int[][] queries) {
            DSU dsu = new DSU(c + 1);
            for (int[] p : connections) {
                dsu.join(p[0], p[1]);
            }

            boolean[] online = new boolean[c + 1];
            int[] offlineCounts = new int[c + 1];
            Arrays.fill(online, true);
            Map<Integer, Integer> minimumOnlineStations = new HashMap<>();

            for (int[] q : queries) {
                int op = q[0];
                int x = q[1];
                if (op == 2) {
                    online[x] = false;
                    offlineCounts[x]++;
                }
            }

            for (int i = 1; i <= c; i++) {
                int root = dsu.find(i);
                if (!minimumOnlineStations.containsKey(root)) {
                    minimumOnlineStations.put(root, -1);
                }

                int station = minimumOnlineStations.get(root);
                if (online[i]) {
                    if (station == -1 || station > i) {
                        minimumOnlineStations.put(root, i);
                    }
                }
            }

            List<Integer> ans = new ArrayList<>();
            for (int i = queries.length - 1; i >= 0; i--) {
                int op = queries[i][0];
                int x = queries[i][1];
                int root = dsu.find(x);
                int station = minimumOnlineStations.get(root);

                if (op == 1) {
                    if (online[x]) {
                        ans.add(x);
                    } else {
                        ans.add(station);
                    }
                }

                if (op == 2) {
                    if (offlineCounts[x] > 1) {
                        offlineCounts[x]--;
                    } else {
                        online[x] = true;
                        if (station == -1 || station > x) {
                            minimumOnlineStations.put(root, x);
                        }
                    }
                }
            }

            Collections.reverse(ans);
            return ans.stream().mapToInt(Integer::intValue).toArray();
        }
    }

    static class Solution2 {
        static class Vertex {
            int vertexId;
            boolean offline = false;
            int powerGridId = -1;

            Vertex() {

            }

            Vertex(int id) {
                this.vertexId = id;
            }
        }

        static class Graph {
            Map<Integer, List<Integer>> adj;
            Map<Integer, Vertex> vertices;

            Graph() {
                this.adj = new HashMap<>();
                this.vertices = new HashMap<>();
            }

            void addVertex(int id, Vertex value) {
                this.vertices.put(id, value);
                this.adj.put(id, new ArrayList<>());
            }

            void addEdge(int u, int v) {
                this.adj.get(u).add(v);
                this.adj.get(v).add(u);
            }

            Vertex getVertexValue(int id) {
                return this.vertices.get(id);
            }

            List<Integer> getConnectedVertices(int id) {
                return this.adj.get(id);
            }
        }

        void traverse(Vertex u, int powerGridId, PriorityQueue<Integer> powerGrid, Graph graph) {
            u.powerGridId = powerGridId;
            powerGrid.add(u.vertexId);
            for (int vid : graph.getConnectedVertices(u.vertexId)) {
                Vertex v = graph.getVertexValue(vid);
                if (v.powerGridId == -1) {
                    traverse(v, powerGridId, powerGrid, graph);
                }
            }
        }

        public int[] processQueries(int c, int[][] connections, int[][] queries) {
            Graph graph = new Graph();
            for (int i = 0; i < c; i++) {
                Vertex v = new Vertex(i + 1);
                graph.addVertex(i + 1, v);
            }

            for (int[] conn : connections) {
                graph.addEdge(conn[0], conn[1]);
            }

            List<PriorityQueue<Integer>> powerGrids = new ArrayList<>();
            for (int i = 1, powerGridId = 0; i <= c; i++) {
                Vertex v = graph.getVertexValue(i);
                if (v.powerGridId == -1) {
                    PriorityQueue<Integer> powerGrid = new PriorityQueue<>();
                    traverse(v, powerGridId, powerGrid, graph);
                    powerGrids.add(powerGrid);
                    powerGridId++;
                }
            }

            List<Integer> ans = new ArrayList<>();
            for (int[] q : queries) {
                int op = q[0];
                int x = q[1];
                if (op == 1) {
                    Vertex vertex = graph.getVertexValue(x);
                    if (!vertex.offline) {
                        ans.add(x);
                    } else {
                        PriorityQueue<Integer> powerGrid = powerGrids.get(
                                vertex.powerGridId);

                        while (!powerGrid.isEmpty() && graph.getVertexValue(powerGrid.peek()).offline) {
                            powerGrid.poll();
                        }
                        ans.add(!powerGrid.isEmpty() ? powerGrid.peek() : -1);
                    }
                } else if (op == 2) {
                    graph.getVertexValue(x).offline = true;
                }
            }
            return ans.stream().mapToInt(Integer::valueOf).toArray();
        }
    }

}
