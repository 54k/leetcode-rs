package leetcode_grind;

import java.util.*;

public class Day662 {
    public class ListNode {
        int val;
        ListNode next;

        ListNode() {
        }

        ListNode(int val) {
            this.val = val;
        }

        ListNode(int val, ListNode next) {
            this.val = val;
            this.next = next;
        }
    }

    // https://leetcode.com/problems/spiral-matrix-iv/description/?envType=daily-question&envId=2024-09-09
    static class Solution1 {
        public int[][] spiralMatrix(int m, int n, ListNode head) {
            int i = 0, j = 0, cur_d = 0, movement[][] = {
                    { 0, 1 },
                    { 1, 0 },
                    { 0, -1 },
                    { -1, 0 },
            };

            int[][] res = new int[m][n];
            for (int[] row : res) {
                Arrays.fill(row, -1);
            }
            while (head != null) {
                res[i][j] = head.val;
                int newi = i + movement[cur_d][0], newj = j + movement[cur_d][1];
                if (Math.min(newi, newj) < 0 || newi >= m || newj >= n || res[newi][newj] != -1) {
                    cur_d = (cur_d + 1) % 4;
                }
                i += movement[cur_d][0];
                j += movement[cur_d][1];
                head = head.next;
            }
            return res;
        }
    }
}
