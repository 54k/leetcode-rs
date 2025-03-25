package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.Set;
import java.util.Stack;

public class Day857 {
    // https://leetcode.com/problems/check-if-grid-can-be-cut-into-sections/description/
    static class Solution1 {
        public boolean checkValidCuts(int n, int[][] rectangles) {
            return checkCuts(rectangles, 0) || checkCuts(rectangles, 1);
        }

        boolean checkCuts(int[][] rectangles, int dim) {
            int gapCount = 0;

            Arrays.sort(rectangles, (a, b) -> Integer.compare(a[dim], b[dim]));

            int furthestEnd = rectangles[0][dim + 2];

            for (int i = 1; i < rectangles.length; i++) {
                int[] rect = rectangles[i];

                if (furthestEnd <= rect[dim]) {
                    gapCount++;
                }

                furthestEnd = Math.max(furthestEnd, rect[dim + 2]);
            }

            return gapCount >= 2;
        }
    }

    // https://leetcode.com/problems/merge-intervals/description/
    static class Solution2 {
        Map<int[], List<int[]>> graph;
        Map<Integer, List<int[]>> nodesInComp;
        Set<int[]> visited;

        boolean overlap(int[] a, int[] b) {
            return a[0] <= b[1] && b[0] <= a[1];
        }

        public int[][] merge(int[][] intervals) {
            buildGraph(intervals);
            buildComponents(intervals);

            List<int[]> merged = new LinkedList<>();
            for (int comp = 0; comp < nodesInComp.size(); comp++) {
                merged.add(mergeNodes(nodesInComp.get(comp)));
            }
            int[][] ans = new int[merged.size()][2];
            for (int i = 0; i < ans.length; i++) {
                ans[i] = merged.get(i);
            }
            return ans;
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

        void markComponentDFS(int[] start, int compNumber) {
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
                    markComponentDFS(interval, compNumber);
                    compNumber++;
                }
            }
        }
    }

    static class Solution3 {
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
            int[][] ans = new int[merged.size()][];
            for (int i = 0; i < ans.length; i++) {
                ans[i] = merged.get(i);
            }
            return ans;
        }
    }

    static class Interval {
        public int start;
        public int end;

        public Interval() {
        }

        public Interval(int _start, int _end) {
            start = _start;
            end = _end;
        }
    };

    // https://leetcode.com/problems/employee-free-time/description/
    static class Solution4 {
        public List<Interval> employeeFreeTime(List<List<Interval>> schedule) {
            int OPEN = 0, CLOSE = 1;
            List<int[]> events = new ArrayList<>();
            for (List<Interval> employee : schedule) {
                for (Interval iv : employee) {
                    events.add(new int[] { iv.start, OPEN });
                    events.add(new int[] { iv.end, CLOSE });
                }
            }

            Collections.sort(events, (a, b) -> a[0] != b[0] ? a[0] - b[0] : a[1] - b[1]);
            List<Interval> ans = new ArrayList<>();

            int prev = -1, bal = 0;
            for (int[] event : events) {
                if (bal == 0 && prev >= 0) {
                    ans.add(new Interval(prev, event[0]));
                }
                bal += event[1] == OPEN ? 1 : -1;
                prev = event[0];
            }

            return ans;
        }
    }

    static class Solution5 {
        static class Job {
            int eid, index;

            Job(int e, int i) {
                eid = e;
                index = i;
            }
        }

        public List<Interval> employeeFreeTime(List<List<Interval>> schedule) {
            List<Interval> ans = new ArrayList<>();
            PriorityQueue<Job> pq = new PriorityQueue<>(
                    (a, b) -> schedule.get(a.eid).get(a.index).start - schedule.get(b.eid).get(b.index).start);
            int ei = 0, anchor = Integer.MAX_VALUE;

            for (List<Interval> employee : schedule) {
                pq.offer(new Job(ei++, 0));
                anchor = Math.min(anchor, employee.get(0).start);
            }

            while (!pq.isEmpty()) {
                Job job = pq.poll();
                Interval iv = schedule.get(job.eid).get(job.index);
                if (anchor < iv.start) {
                    ans.add(new Interval(anchor, iv.start));
                }
                anchor = Math.max(anchor, iv.end);
                if (++job.index < schedule.get(job.eid).size()) {
                    pq.offer(job);
                }
            }

            return ans;
        }
    }
}
