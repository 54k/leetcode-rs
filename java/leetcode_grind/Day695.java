package leetcode_grind;

import java.util.*;

public class Day695 {
    // https://leetcode.com/problems/meeting-rooms/description/
    static class Solution1 {
        public boolean canAttendMeetings(int[][] intervals) {
            Arrays.sort(intervals, (a, b) -> a[0] - b[0]);
            for (int i = 1; i < intervals.length; i++) {
                if (intervals[i - 1][1] > intervals[i][0]) {
                    return false;
                }
            }
            return true;
        }
    }

    // https://leetcode.com/problems/points-that-intersect-with-cars/description/
    static class Solution2 {
        public int numberOfPoints(List<List<Integer>> nums) {
            var c = new int[102];
            for (var n : nums) {
                c[n.get(0)] += 1;
                c[n.get(1) + 1] -= 1;
            }
            for (int i = 1; i < c.length; i++) {
                c[i] += c[i - 1];
            }
            int ans = 0;
            for (var p : c) {
                if (p > 0) {
                    ans++;
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/merge-intervals/description/
    static class Solution3 {
        public int[][] merge(int[][] intervals) {
            Arrays.sort(intervals, (a, b) -> a[0] - b[0]);
            var ints = new ArrayList<int[]>();
            for (var i : intervals) {
                if (ints.isEmpty() || ints.get(ints.size() - 1)[1] < i[0]) {
                    ints.add(i);
                } else if (i[1] > ints.get(ints.size() - 1)[1]) {
                    ints.get(ints.size() - 1)[1] = i[1];
                }
            }
            int[][] ans = new int[ints.size()][];
            for (int i = 0; i < ints.size(); i++) {
                ans[i] = ints.get(i);
            }
            return ans;
        }
    }

    static class Solution4 {

        Map<int[], List<int[]>> graph;
        Map<Integer, List<int[]>> nodesInComp;
        Set<int[]> visited;

        boolean overlap(int[] a, int[] b) {
            return a[0] <= b[1] && b[0] <= a[1];
        }

        void buildGraph(int[][] intervals) {
            graph = new HashMap<>();
            for (int[] interval : intervals) {
                graph.put(interval, new LinkedList<>());
            }
            for (int[] interval1 : intervals) {
                for (int[] interval2 : intervals) {
                    if (overlap(interval1, interval2)) {
                        graph.get(interval1).add(interval2);
                        graph.get(interval2).add(interval1);
                    }
                }
            }
        }

        int[] mergeNodes(List<int[]> nodes) {
            int minStart = nodes.get(0)[0];
            for (int[] node : nodes) {
                minStart = Math.min(minStart, node[0]);
            }
            int maxEnd = nodes.get(0)[1];
            for (int[] node : nodes) {
                maxEnd = Math.max(maxEnd, node[1]);
            }
            return new int[] { minStart, maxEnd };
        }

        void markComponentsDFS(int[] start, int compNumber) {
            Stack<int[]> stack = new Stack<>();
            stack.add(start);

            while (!stack.isEmpty()) {
                int[] node = stack.pop();
                if (!visited.contains(node)) {
                    visited.add(node);
                    if (nodesInComp.get(compNumber) == null) {
                        nodesInComp.put(compNumber, new LinkedList<>());
                    }
                    nodesInComp.get(compNumber).add(node);
                    for (int[] child : graph.get(node)) {
                        stack.add(child);
                    }
                }
            }
        }

        void buildComponents(int[][] intervals) {
            nodesInComp = new HashMap<>();
            visited = new HashSet<>();
            int compNumber = 0;

            for (int[] interval : intervals) {
                if (!visited.contains(interval)) {
                    markComponentsDFS(interval, compNumber);
                    compNumber++;
                }
            }
        }

        public int[][] merge(int[][] intervals) {
            buildGraph(intervals);
            buildComponents(intervals);
            List<int[]> merged = new LinkedList<>();
            for (int comp = 0; comp < nodesInComp.size(); comp++) {
                merged.add(mergeNodes(nodesInComp.get(comp)));
            }
            return merged.toArray(new int[merged.size()][]);
        }
    }

    static class Solution5 {
        public int[][] merge(int[][] intervals) {
            Arrays.sort(intervals, (a, b) -> Integer.compare(a[0], b[0]));
            LinkedList<int[]> merged = new LinkedList<>();
            for (int[] interval : intervals) {
                if (merged.isEmpty() || merged.getLast()[1] < interval[0]) {
                    merged.add(interval);
                } else {
                    merged.getLast()[1] = Math.max(merged.getLast()[1], interval[1]);
                }
            }
            return merged.toArray(new int[merged.size()][]);
        }
    }

    
}
