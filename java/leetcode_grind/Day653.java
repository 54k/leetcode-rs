package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
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

    // https://leetcode.com/problems/find-the-key-of-the-numbers/description/
    static class Solution2 {
        public int generateKey(int num1, int num2, int num3) {
            int res = 0;
            for (int i = 3; i >= 0; i--) {
                int p = (int) Math.pow(10, i);
                int k = Math.min(num1 / p % 10, Math.min(
                        num2 / p % 10, num3 / p % 10));
                res += k * p;
            }
            return res;
        }
    }

    // https://leetcode.com/problems/hash-divided-string/description/
    static class Solution3 {
        public String stringHash(String s, int k) {
            StringBuilder result = new StringBuilder();
            int blocks = s.length() / k;
            for (int b = 0; b < blocks; b++) {
                String ss = s.substring(b * k, b * k + k);
                // System.out.println(ss);
                int h = 0;
                for (int j = 0; j < ss.length(); j++) {
                    h += (int) ss.charAt(j) - 'a';
                }
                result.append((char) (h % 26 + 'a'));
            }
            return result.toString();
        }
    }

    // https://leetcode.com/problems/minimum-amount-of-damage-dealt-to-bob/description/
    static class Solution4 {
        public long minDamage(int power, int[] D, int[] H) {
            Integer[] A = new Integer[D.length];
            Arrays.setAll(A, i -> i);
            Arrays.sort(A, (i, j) -> {
                int v1 = (H[i] + power - 1) / power * D[j];
                int v2 = (H[j] + power - 1) / power * D[i];
                return v1 - v2;
            });

            long res = 0, t = 0;
            for (int i : A) {
                t += (H[i] + power - 1) / power;
                res += D[i] * t;
            }
            return res;
        }
    }

    static class Solution5 {
        public long minDamage(int power, int[] damage, int[] health) {
            int n = damage.length;
            for (int i = 0; i < n; i++) {
                health[i] = (health[i] + power - 1) / power;
            }

            List<Integer> indexes = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                indexes.add(i);
            }

            indexes.sort((i, j) -> {
                return Integer.compare(health[i] * damage[j], health[j] * damage[i]);
            });

            long res = 0;
            int curTime = 0;
            for (int i : indexes) {
                curTime += health[i];
                res += curTime * damage[i];
            }
            return res;
        }
    }
}
