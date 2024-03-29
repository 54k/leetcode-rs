package leetcode_grind;

import java.util.ArrayList;
import java.util.LinkedList;

public class Day352 {
    // https://leetcode.com/problems/zigzag-conversion/description/
    static class Solution1 {
        public String convert(String s, int numRows) {
            return "";
        }
    }

    // https://leetcode.com/problems/interval-list-intersections/description/
    static class Solution2 {
        public int[][] intervalIntersection(int[][] firstList, int[][] secondList) {
            var ans = new ArrayList<int[]>();
            int i = 0, j = 0;
            while (i < firstList.length && j < secondList.length) {
                var lo = Math.max(firstList[i][0], secondList[j][0]);
                var hi = Math.min(firstList[i][1], secondList[j][1]);
                if (lo <= hi) {
                    ans.add(new int[] { lo, hi });
                }

                if (firstList[i][1] < secondList[j][1]) {
                    i++;
                } else {
                    j++;
                }
            }
            return ans.toArray(new int[][] {});
        }
    }

    // https://leetcode.com/problems/jump-game-iii/description/
    static class Solution3 {
        public boolean canReachBFS(int[] arr, int start) {
            var queue = new LinkedList<Integer>();
            queue.push(start);
            while (queue.size() > 0) {
                var top = queue.pop();
                if (0 <= top && top < arr.length && arr[top] >= 0) {
                    if (arr[top] == 0) {
                        return true;
                    }

                    arr[top] *= -1;
                    queue.push(top - arr[top]);
                    queue.push(top + arr[top]);
                }
            }
            return false;
        }

        public boolean canReachDFS(int[] arr, int start) {
            var dfs = new Object() {
                boolean apply(int start) {
                    if (0 <= start && start < arr.length && arr[start] >= 0) {
                        if (arr[start] == 0) {
                            return true;
                        }
                        arr[start] *= -1;
                        return apply(start - arr[start]) || apply(start + arr[start]);
                    }
                    return false;
                }
            };
            return dfs.apply(start);
        }
    }
}
