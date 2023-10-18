package leetcode_grind;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.PriorityQueue;
import java.util.Stack;

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

        public int scheduleCourseHeap(int[][] courses) {
            Arrays.sort(courses, (a, b) -> a[1] - b[1]);
            int time = 0;
            PriorityQueue<Integer> heap = new PriorityQueue<>((a, b) -> b - a);

            for (var c : courses) {
                if (time + c[0] <= c[1]) {
                    heap.offer(c[0]);
                    time += c[0];
                } else if (heap.size() > 0 && heap.peek() > c[0]) {
                    time += c[0] - heap.poll();
                    heap.offer(c[0]);
                }
            }

            return heap.size();
        }

        public int scheduleCourseIterativeOptimized(int[][] courses) {
            Arrays.sort(courses, (a, b) -> a[1] - b[1]);
            int time = 0, count = 0;
            for (int i = 0; i < courses.length; i++) {
                if (time + courses[i][0] <= courses[i][1]) {
                    time += courses[i][0];
                    courses[count++] = courses[i];
                } else {
                    int max_i = i;
                    for (int j = 0; j < count; j++) {
                        if (courses[j][0] > courses[max_i][0]) {
                            max_i = j;
                        }
                    }

                    if (courses[max_i][0] > courses[i][0]) {
                        time += courses[i][0] - courses[max_i][0];
                        courses[max_i] = courses[i];
                    }
                }
            }
            return count;
        }

        public int scheduleCourseSecondList(int[][] courses) {
            Arrays.sort(courses, (a, b) -> a[1] - b[1]);
            List<Integer> validList = new ArrayList<>();
            int time = 0;
            for (int[] c : courses) {
                if (time + c[0] <= c[1]) {
                    validList.add(c[0]);
                    time += c[0];
                } else {
                    int max_i = 0;
                    for (int j = 1; j < validList.size(); j++) {
                        if (validList.get(j) > validList.get(max_i)) {
                            max_i = j;
                        }
                    }

                    if (validList.size() > 0 && validList.get(max_i) > c[0]) {
                        time += c[0] - validList.get(max_i);
                        validList.set(max_i, c[0]);
                    }
                }
            }
            return validList.size();
        }
    }

    // https://leetcode.com/problems/basic-calculator-ii/description/
    static class Solution3 {
        public int calculateStack(String s) {
            if (s == null || s.isEmpty())
                return 0;
            int len = s.length();
            Stack<Integer> stack = new Stack<>();
            int currentNumber = 0;
            char operation = '+';
            for (int i = 0; i < len; i++) {
                char currentChar = s.charAt(i);
                if (Character.isDigit(currentChar)) {
                    currentNumber = (currentNumber * 10) + (currentChar - '0');
                }
                if (!Character.isDigit(currentChar) && !Character.isWhitespace(currentChar) || i == len - 1) {
                    if (operation == '-') {
                        stack.push(-currentNumber);
                    } else if (operation == '+') {
                        stack.push(currentNumber);
                    } else if (operation == '*') {
                        stack.push(stack.pop() * currentNumber);
                    } else if (operation == '/') {
                        stack.push(stack.pop() / currentNumber);
                    }
                    operation = currentChar;
                    currentNumber = 0;
                }
            }

            int result = 0;
            while (!stack.isEmpty()) {
                result += stack.pop();
            }
            return result;
        }

        public int calculateOptimized(String s) {
            if (s == null || s.isEmpty())
                return 0;
            int length = s.length();
            int currentNumber = 0, lastNumber = 0, result = 0;
            char operation = '+';
            for (int i = 0; i < length; i++) {
                char currentChar = s.charAt(i);
                if (Character.isDigit(currentChar)) {
                    currentNumber = (currentNumber * 10) + (currentChar - '0');
                }
                if (!Character.isDigit(currentChar) && !Character.isWhitespace(currentChar) || i == length - 1) {
                    if (operation == '+' || operation == '-') {
                        result += lastNumber;
                        lastNumber = (operation == '+') ? currentNumber : -currentNumber;
                    } else if (operation == '*') {
                        lastNumber = lastNumber * currentNumber;
                    } else if (operation == '/') {
                        lastNumber = lastNumber / currentNumber;
                    }
                    operation = currentChar;
                    currentNumber = 0;
                }
            }
            result += lastNumber;
            return result;
        }
    }
}
