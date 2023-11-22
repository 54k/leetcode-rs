package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Collections;

public class Day375 {
    // https://leetcode.com/problems/maximum-sum-of-distinct-subarrays-with-length-k/
    static class Solution1 {
        public long maximumSubarraySum1(int[] nums, int k) {
            var d = new HashSet<Integer>();
            var n = nums.length;
            var r = 0l;
            var ans = 0l;
            for (int right = 0, left = 0; right < n; right++) {
                while (d.contains(nums[right]) || d.size() >= k) {
                    d.remove(nums[left]);
                    r -= nums[left];
                    left++;
                }
                d.add(nums[right]);
                r += nums[right];
                if (d.size() == k) {
                    ans = Math.max(ans, r);
                }
            }
            return ans;
        }

        public long maximumSubarraySum2(int[] nums, int k) {
            long ans = 0;
            long r = 0;
            var n = nums.length;
            var len = 0;
            var cnt = new HashMap<Integer, Integer>();
            for (int i = 0; i < n; i++) {
                r += nums[i];
                if (cnt.getOrDefault(nums[i], 0) == 0) {
                    len += 1;
                }
                cnt.put(nums[i], cnt.getOrDefault(nums[i], 0) + 1);

                if (i >= k) {
                    r -= nums[i - k];
                    cnt.put(nums[i - k], cnt.get(nums[i - k]) - 1);
                    if (cnt.get(nums[i - k]) == 0) {
                        len -= 1;
                    }
                }

                if (i >= k - 1) {
                    if (len == k) {
                        ans = Math.max(ans, r);
                    }
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/reconstruct-itinerary/
    static class Solution2 {
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
    static class Solution3 {
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
