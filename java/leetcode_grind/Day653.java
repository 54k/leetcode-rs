package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;

public class Day653 {
    static class Pair<F, S> {
        F key;
        S value;

        Pair(F f, S s) {
            key = f;
            value = s;
        }

        F getKey() {
            return key;
        }

        S getValue() {
            return value;
        }
    }

    // https://leetcode.com/problems/path-with-maximum-probability/description/?envType=daily-question&envId=2024-08-31
    static class Solution1 {
        public double maxProbability(int n, int[][] edges, double[] succProb, int start_node, int end_node) {
            Map<Integer, List<Pair<Integer, Double>>> graph = new HashMap<>();
            for (int i = 0; i < edges.length; i++) {
                int u = edges[i][0], v = edges[i][1];
                double pathProb = succProb[i];
                graph.computeIfAbsent(u, k -> new ArrayList<>()).add(new Pair<>(v, pathProb));
                graph.computeIfAbsent(v, k -> new ArrayList<>()).add(new Pair<>(u, pathProb));
            }

            double[] maxProb = new double[n];
            maxProb[start_node] = 1.0;

            PriorityQueue<Pair<Double, Integer>> pq = new PriorityQueue<>(
                    (a, b) -> -Double.compare(a.getKey(), b.getKey()));
            pq.add(new Pair<>(1.0, start_node));

            while (!pq.isEmpty()) {
                Pair<Double, Integer> cur = pq.poll();
                double curProb = cur.getKey();
                int curNode = cur.getValue();

                if (curNode == end_node) {
                    return curProb;
                }

                for (Pair<Integer, Double> nxt : graph.getOrDefault(curNode, new ArrayList<>())) {
                    int nxtNode = nxt.getKey();
                    double pathProb = nxt.getValue();
                    if (curProb * pathProb > maxProb[nxtNode]) {
                        maxProb[nxtNode] = curProb * pathProb;
                        pq.add(new Pair<>(maxProb[nxtNode], nxtNode));
                    }
                }
            }

            return 0d;
        }
    }

}
