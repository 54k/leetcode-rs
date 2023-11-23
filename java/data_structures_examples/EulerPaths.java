package data_structures_examples;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;

public class EulerPaths {
    // https://leetcode.com/problems/reconstruct-itinerary/
    static class Solution1 {
        public List<String> findItineraryBacktrack(List<List<String>> tickets) {
            var adj = new HashMap<String, List<String>>();
            for (var ticket : tickets) {
                adj.putIfAbsent(ticket.get(0), new ArrayList<>());
                adj.get(ticket.get(0)).add(ticket.get(1));
            }

            adj.values().forEach(Collections::sort);

            var visited = new HashSet<String>();
            var path = new ArrayList<String>() {
                {
                    add("JFK");
                }
            };

            var backtrack = new Object() {
                boolean apply(String v) {
                    if (path.size() == tickets.size() + 1) {
                        System.out.println("found!");
                        return true;
                    }
                    if (adj.get(v) == null) {
                        return false;
                    }
                    var i = 0; // for proper visited key
                    for (var u : adj.get(v)) {
                        var key = String.format("%s->%s", v, i);
                        if (visited.add(key)) {
                            path.add(u);
                            if (apply(u)) {
                                return true;
                            }
                            path.remove(path.size() - 1);
                            visited.remove(key);
                        }
                        i++;
                    }
                    return false;
                }
            };

            backtrack.apply("JFK");
            return path;
        }

        public List<String> findItineraryEuler(List<List<String>> tickets) {
            var adj = new HashMap<String, List<String>>();
            for (var ticket : tickets) {
                adj.putIfAbsent(ticket.get(0), new ArrayList<>());
                adj.get(ticket.get(0)).add(ticket.get(1));
            }

            adj.values().forEach(Collections::sort);

            var visited = new HashSet<String>();
            var path = new ArrayList<String>();
            var dfs = new Object() {
                void apply(String v) {
                    if (adj.containsKey(v)) {
                        var i = 0;
                        for (var u : adj.get(v)) {
                            if (visited.add(String.format("%s->%s", v, i))) {
                                apply(u);
                            }
                            i++;
                        }
                    }
                    path.add(v);
                }
            };
            dfs.apply("JFK");
            Collections.reverse(path);
            return path;
        }
    }

    // https://leetcode.com/problems/valid-arrangement-of-pairs/description/
    static class Solution2 {
        public int[][] validArrangement1(int[][] pairs) {
            var adj = new HashMap<Integer, List<Integer>>();
            var inDegree = new HashMap<Integer, Integer>();
            var outDegree = new HashMap<Integer, Integer>();

            for (var p : pairs) {
                adj.putIfAbsent(p[0], new ArrayList<>());
                adj.putIfAbsent(p[1], new ArrayList<>());

                adj.get(p[0]).add(p[1]);

                outDegree.putIfAbsent(p[0], 0);
                outDegree.putIfAbsent(p[1], 0);

                inDegree.putIfAbsent(p[0], 0);
                inDegree.putIfAbsent(p[1], 0);

                outDegree.put(p[0], outDegree.get(p[0]) + 1);
                inDegree.put(p[1], inDegree.get(p[1]) + 1);
            }

            class Euler {
                List<Integer> path = new ArrayList<>();

                void apply(int v) {
                    while (!adj.get(v).isEmpty()) {
                        var u = adj.get(v).remove(adj.get(v).size() - 1);
                        apply(u);
                    }
                    path.add(v);
                }
            }
            ;

            var start = pairs[0][0];
            for (var p : pairs) {
                if ((outDegree.get(p[0]) - inDegree.get(p[0])) == 1) {
                    start = p[0];
                    break;
                }
            }

            var euler = new Euler();
            euler.apply(start);

            var result = new ArrayList<int[]>();
            for (int i = euler.path.size() - 2; i >= 0; i--) {
                result.add(new int[] { euler.path.get(i + 1), euler.path.get(i) });
            }

            return result.toArray(new int[0][0]);
        }

        int k = 0;

        public int[][] validArrangement2(int[][] pairs) {
            var adj = new HashMap<Integer, List<Integer>>();
            var inDegree = new HashMap<Integer, Integer>();
            var outDegree = new HashMap<Integer, Integer>();
            var result = new int[pairs.length][2];
            k = pairs.length - 1;

            for (var p : pairs) {
                adj.putIfAbsent(p[0], new ArrayList<>());
                adj.putIfAbsent(p[1], new ArrayList<>());

                adj.get(p[0]).add(p[1]);

                outDegree.putIfAbsent(p[0], 0);
                outDegree.putIfAbsent(p[1], 0);

                inDegree.putIfAbsent(p[0], 0);
                inDegree.putIfAbsent(p[1], 0);

                outDegree.put(p[0], outDegree.get(p[0]) + 1);
                inDegree.put(p[1], inDegree.get(p[1]) + 1);
            }

            class Euler {
                void apply(int v) {
                    while (!adj.get(v).isEmpty()) {
                        var u = adj.get(v).remove(adj.get(v).size() - 1);
                        apply(u);
                        result[k--] = new int[] { v, u };
                    }
                }
            }
            ;

            var start = pairs[0][0];
            for (var p : pairs) {
                if (outDegree.get(p[0]) - inDegree.get(p[0]) == 1) {
                    start = p[0];
                    break;
                }
            }

            var euler = new Euler();
            euler.apply(start);
            return result;
        }
    }
}
