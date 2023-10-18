package leetcode_grind;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;

public class Day340 {
    // https://leetcode.com/problems/parallel-courses-iii/description/
    static class Solution1 {
        public int minimumTime(int n, int[][] relations, int[] time) {
            var inDegree = new int[n];
            var adj = new HashMap<Integer, List<Integer>>();
            for (var i = 0; i < n; i++) {
                adj.put(i, new ArrayList<>());
            }

            for (var relation : relations) {
                var from = relation[0] - 1;
                var to = relation[1] - 1;
                inDegree[to]++;
                adj.get(from).add(to);
            }

            var ans = 0;
            var dist = new int[n];
            var queue = new ArrayDeque<Integer>();
            for (var i = 0; i < inDegree.length; i++) {
                if (inDegree[i] == 0) {
                    queue.addLast(i);
                    dist[i] = time[i];
                    ans = Math.max(dist[i], ans);
                }
            }
            while (queue.size() > 0) {
                var curr = queue.pollFirst();
                for (var next : adj.get(curr)) {
                    dist[next] = Math.max(dist[next], time[next] + dist[curr]);
                    ans = Math.max(dist[next], ans);
                    if (--inDegree[next] == 0) {
                        queue.addLast(next);
                    }
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/course-schedule-iii/description/
    static class Solution2 {
        public int scheduleCourseRecursive(int[][] courses) {
            Arrays.sort(courses, (a, b) -> a[1] - b[1]);
            Integer[][] memo = new Integer[courses.length][courses[courses.length - 1][1] + 1];
            return schedule(courses, 0, 0, memo);
        }

        public int schedule(int[][] courses, int i, int time, Integer[][] memo) {
            if (i == courses.length) {
                return 0;
            }
            if (memo[i][time] != null) {
                return memo[i][time];
            }

            int taken = 0;
            if (time + courses[i][0] <= courses[i][1]) {
                taken = 1 + schedule(courses, i + 1, time + courses[i][0], memo);
            }

            int notTaken = schedule(courses, i + 1, time, memo);
            memo[i][time] = Math.max(taken, notTaken);
            return memo[i][time];
        }

        public int scheduleCourseIterative(int[][] courses) {
            Arrays.sort(courses, (a, b) -> a[1] - b[1]);
            var time = 0;
            var count = 0;

            for (var i = 0; i < courses.length; i++) {
                if (time + courses[i][0] <= courses[i][1]) {
                    time += courses[i][0];
                    count++;
                } else {
                    int max_i = i;
                    for (var j = 0; j < i; j++) {
                        if (courses[j][0] > courses[max_i][0]) {
                            max_i = j;
                        }
                    }
                    if (courses[max_i][0] > courses[i][0]) {
                        time += courses[i][0] - courses[max_i][0];
                    }
                    courses[max_i][0] = -1;
                }
            }
            return count;
        }
    }
}
