package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day922 {
    static public class ListNode {
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

    // https://leetcode.com/problems/delete-n-nodes-after-m-nodes-of-a-linked-list/description/?envType=weekly-question&envId=2025-05-29
    static class Solution1 {
        public ListNode deleteNodes(ListNode head, int m, int n) {
            ListNode currentNode = head;
            ListNode lastMNode = head;
            while (currentNode != null) {
                int mCount = m, nCount = n;
                while (currentNode != null && mCount != 0) {
                    lastMNode = currentNode;
                    currentNode = currentNode.next;
                    mCount--;
                }
                while (currentNode != null && nCount != 0) {
                    currentNode = currentNode.next;
                    nCount--;
                }
                lastMNode.next = currentNode;
            }
            return head;
        }
    }

    // https://leetcode.com/problems/maximize-the-number-of-target-nodes-after-connecting-trees-ii/description/?envType=daily-question&envId=2025-05-29
    static class Solution2 {
        public int[] maxTargetNodes(int[][] edges1, int[][] edges2) {
            int n = edges1.length + 1, m = edges2.length + 1;
            int[] color1 = new int[n];
            int[] color2 = new int[m];
            int[] count1 = build(edges1, color1);
            int[] count2 = build(edges2, color2);
            int[] res = new int[n];
            for (int i = 0; i < n; i++) {
                res[i] = count1[color1[i]] + Math.max(count2[0], count2[1]);
            }
            return res;
        }

        int[] build(int[][] edges, int[] color) {
            int n = edges.length + 1;
            List<List<Integer>> children = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                children.add(new ArrayList<>());
            }
            for (int[] edge : edges) {
                children.get(edge[0]).add(edge[1]);
                children.get(edge[1]).add(edge[0]);
            }
            int res = dfs(0, -1, 0, children, color);
            return new int[] { res, n - res };
        }

        int dfs(int node, int parent, int depth, List<List<Integer>> children, int[] color) {
            int res = 1 - (depth % 2);
            color[node] = depth % 2;
            for (int child : children.get(node)) {
                if (child == parent) {
                    continue;
                }
                res += dfs(child, node, depth + 1, children, color);
            }
            return res;
        }
    }

}
