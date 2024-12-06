package leetcode_grind;

import java.util.ArrayDeque;
import java.util.Arrays;
import java.util.HashSet;
import java.util.PriorityQueue;
import java.util.Set;

public class Day759 {
    // https://leetcode.com/problems/swap-adjacent-in-lr-string/description/
    static class Solution1 {
        public boolean canTransform(String start, String result) {
            class Val {
                int idx;
                int ch;

                Val(int i, int c) {
                    idx = i;
                    ch = c;
                }
            }

            var v1 = new ArrayDeque<Val>();
            var v2 = new ArrayDeque<Val>();
            for (int i = 0; i < start.length(); i++) {
                var ch1 = start.charAt(i);
                if (ch1 != 'X') {
                    v1.addLast(new Val(i, ch1));
                }
                var ch2 = result.charAt(i);
                if (ch2 != 'X') {
                    v2.addLast(new Val(i, ch2));
                }
            }

            if (v1.size() != v2.size()) {
                return false;
            }

            for (; !v1.isEmpty();) {
                var val1 = v1.removeFirst();
                var val2 = v2.removeFirst();

                if (val1.ch != val2.ch) {
                    return false;
                }

                if (val1.ch == 'L' && val1.idx < val2.idx) {
                    return false;
                }
                if (val1.ch == 'R' && val1.idx > val2.idx) {
                    return false;
                }
            }

            return true;
        }
    }

    // https://leetcode.com/problems/maximum-number-of-integers-to-choose-from-a-range-i/description/
    static class Solution2 {
        public int maxCount(int[] banned, int n, int maxSum) {
            int count = 0;
            Arrays.sort(banned);
            for (int num = 1; num <= n; num++) {
                if (customBinarySearch(banned, num))
                    continue;
                maxSum -= num;
                if (maxSum < 0)
                    break;
                count++;
            }
            return count;
        }

        boolean customBinarySearch(int[] arr, int target) {
            int lo = 0, hi = arr.length - 1;
            while (lo <= hi) {
                int mid = lo + (hi - lo) / 2;
                if (arr[mid] == target) {
                    return true;
                } else if (arr[mid] < target) {
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }
            return false;
        }
    }

    static class Solution3 {
        public int maxCount(int[] banned, int n, int maxSum) {
            Set<Integer> bannedSet = new HashSet<>();
            for (int num : banned) {
                bannedSet.add(num);
            }

            int count = 0;
            for (int num = 1; num <= n; num++) {
                if (bannedSet.contains(num))
                    continue;
                if (maxSum - num < 0)
                    return count;
                maxSum -= num;
                count++;
            }
            return count++;
        }
    }

    // https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination/description/
    static class Solution4 {
        static class StepState implements Comparable {
            int estimation, steps, row, col, k;
            int[] target;

            StepState(int steps, int row, int col, int k, int[] target) {
                this.steps = steps;
                this.row = row;
                this.col = col;
                this.k = k;
                this.target = target;
                int manhattanDistance = target[0] - row + target[1] - col;
                this.estimation = manhattanDistance + steps;
            }

            @Override
            public int hashCode() {
                return (this.row + 1) * (this.col + 1) * this.k;
            }

            @Override
            public int compareTo(Object o) {
                StepState other = (StepState) o;
                return this.estimation - other.estimation;
            }

            @Override
            public boolean equals(Object other) {
                /**
                 * only (row, col, k) matters as the state info
                 */
                if (!(other instanceof StepState)) {
                    return false;
                }
                StepState newState = (StepState) other;
                return (this.row == newState.row) && (this.col == newState.col) && (this.k == newState.k);
            }

            @Override
            public String toString() {
                return String.format("%d %d %d %d", this.steps, this.row, this.col, this.k);
            }
        }

        public int shortestPath(int[][] grid, int k) {
            int rows = grid.length, cols = grid[0].length;
            int[] target = { rows - 1, cols - 1 };

            PriorityQueue<StepState> queue = new PriorityQueue<>();
            HashSet<StepState> seen = new HashSet<>();

            StepState start = new StepState(0, 0, 0, k, target);
            queue.offer(start);
            seen.add(start);

            while (!queue.isEmpty()) {
                StepState curr = queue.poll();
                int remainingMinDistance = curr.estimation - curr.steps;
                if (remainingMinDistance <= curr.k) {
                    return curr.estimation;
                }

                if (target[0] == curr.row && target[1] == curr.col) {
                    return curr.steps;
                }

                int[] nextSteps = { curr.row, curr.col + 1, curr.row + 1, curr.col, curr.row, curr.col - 1,
                        curr.row - 1, curr.col };

                for (int i = 0; i < nextSteps.length; i += 2) {
                    int nextRow = nextSteps[i];
                    int nextCol = nextSteps[i + 1];

                    if (0 > nextRow || nextRow == rows || 0 > nextCol || nextCol == cols) {
                        continue;
                    }

                    int nextElimination = curr.k - grid[nextRow][nextCol];
                    StepState newState = new StepState(curr.steps + 1, nextRow, nextCol, nextElimination, target);

                    if (nextElimination >= 0 && !seen.contains(newState)) {
                        seen.add(newState);
                        queue.offer(newState);
                    }
                }
            }
            return -1;
        }
    }
}
