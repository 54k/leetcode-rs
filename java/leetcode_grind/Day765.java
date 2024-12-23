package leetcode_grind;

import java.util.Arrays;
import java.util.Comparator;
import java.util.Deque;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.Queue;
import java.util.Set;
import java.util.Stack;
import java.util.TreeSet;

public class Day765 {
    // https://leetcode.com/problems/minimum-number-of-operations-to-sort-a-binary-tree-by-level/description/
    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;

        TreeNode() {
        }

        TreeNode(int val) {
            this.val = val;
        }

        TreeNode(int val, TreeNode left, TreeNode right) {
            this.val = val;
            this.left = left;
            this.right = right;
        }
    }

    static class Solution1 {
        public int minimumOperations(TreeNode root) {
            Queue<TreeNode> queue = new LinkedList<>();
            queue.add(root);
            int totalSwaps = 0;

            while (!queue.isEmpty()) {
                int levelSize = queue.size();
                int[] levelValues = new int[levelSize];

                for (int i = 0; i < levelSize; i++) {
                    TreeNode node = queue.poll();
                    levelValues[i] = node.val;

                    if (node.left != null)
                        queue.add(node.left);
                    if (node.right != null)
                        queue.add(node.right);
                }

                totalSwaps += getMinSwaps(levelValues);
            }
            return totalSwaps;
        }

        int getMinSwaps(int[] original) {
            int swaps = 0;
            int[] target = original.clone();
            Arrays.sort(target);

            Map<Integer, Integer> pos = new HashMap<>();
            for (int i = 0; i < original.length; i++) {
                pos.put(original[i], i);
            }

            for (int i = 0; i < original.length; i++) {
                if (original[i] != target[i]) {
                    swaps++;
                    int curPos = pos.get(target[i]);
                    pos.put(original[i], curPos);
                    original[curPos] = original[i];
                }
            }
            return swaps;
        }
    }

    static class Solution2 {
        int SHIFT = 20;
        int MASK = 0xFFFFF;

        public int minimumOperations(TreeNode root) {
            Queue<TreeNode> queue = new LinkedList<>();
            queue.add(root);
            int swaps = 0;

            while (!queue.isEmpty()) {
                int levelSize = queue.size();
                long[] nodes = new long[levelSize];

                for (int i = 0; i < levelSize; i++) {
                    TreeNode node = queue.poll();
                    nodes[i] = ((long) node.val << SHIFT) + i;
                    if (node.left != null)
                        queue.add(node.left);
                    if (node.right != null)
                        queue.add(node.right);
                }

                Arrays.sort(nodes);

                for (int i = 0; i < levelSize; i++) {
                    int origPos = (int) (nodes[i] & MASK);
                    if (origPos != i) {
                        long temp = nodes[i];
                        nodes[i--] = nodes[origPos];
                        nodes[origPos] = temp;
                        swaps++;
                    }
                }
            }
            return swaps;
        }
    }

    // https://leetcode.com/problems/complete-binary-tree-inserter/
    static class CBTInserter {
        TreeNode root;
        Deque<TreeNode> deque;

        public CBTInserter(TreeNode root) {
            this.root = root;

            deque = new LinkedList<>();
            Queue<TreeNode> queue = new LinkedList<>();
            queue.offer(root);

            while (!queue.isEmpty()) {
                TreeNode node = queue.poll();
                if (node.left == null || node.right == null) {
                    deque.offerLast(node);
                }
                if (node.left != null) {
                    queue.offer(node.left);
                }
                if (node.right != null) {
                    queue.offer(node.right);
                }
            }
        }

        public int insert(int val) {
            TreeNode node = deque.peekFirst();
            deque.offerLast(new TreeNode(val));
            if (node.left == null) {
                node.left = deque.peekLast();
            } else {
                node.right = deque.peekLast();
                deque.pollFirst();
            }
            return node.val;
        }

        public TreeNode get_root() {
            return root;
        }
    }

    // https://leetcode.com/problems/reshape-the-matrix/description/
    static class Solution3 {
        public int[][] matrixReshape(int[][] nums, int r, int c) {
            int[][] res = new int[r][c];
            if (nums.length == 0 || r * c != nums.length * nums[0].length) {
                return nums;
            }

            Queue<Integer> queue = new LinkedList<>();
            for (int i = 0; i < nums.length; i++) {
                for (int j = 0; j < nums[0].length; j++) {
                    queue.add(nums[i][j]);
                }
            }

            for (int i = 0; i < r; i++) {
                for (int j = 0; j < c; j++) {
                    res[i][j] = queue.remove();
                }
            }
            return res;
        }
    }

    // https://leetcode.com/problems/max-stack/
    static class MaxStack1 {
        TreeSet<int[]> stack;
        TreeSet<int[]> values;
        int cnt;

        public MaxStack1() {
            Comparator<int[]> comp = (a, b) -> {
                return a[0] == b[0] ? a[1] - b[1] : a[0] - b[0];
            };
            stack = new TreeSet<>(comp);
            values = new TreeSet<>(comp);
            cnt = 0;
        }

        public void push(int x) {
            stack.add(new int[] { cnt, x });
            values.add(new int[] { x, cnt });
            cnt++;
        }

        public int pop() {
            int[] pair = stack.pollLast();
            values.remove(new int[] { pair[1], pair[0] });
            return pair[1];
        }

        public int top() {
            return stack.last()[1];
        }

        public int peekMax() {
            return values.last()[0];
        }

        public int popMax() {
            int[] pair = values.pollLast();
            stack.remove(new int[] { pair[1], pair[0] });
            return pair[0];
        }
    }

    /**
     * Your MaxStack object will be instantiated and called as such:
     * MaxStack obj = new MaxStack();
     * obj.push(x);
     * int param_2 = obj.pop();
     * int param_3 = obj.top();
     * int param_4 = obj.peekMax();
     * int param_5 = obj.popMax();
     */

    static class MaxStack2 {
        Stack<int[]> stack;
        Queue<int[]> heap;
        Set<Integer> removed;
        int cnt;

        public MaxStack2() {
            stack = new Stack<>();
            heap = new PriorityQueue<>((a, b) -> b[0] - a[0] == 0 ? b[1] - a[1] : b[0] - a[0]);
            removed = new HashSet<>();
        }

        public void push(int x) {
            stack.add(new int[] { x, cnt });
            heap.add(new int[] { x, cnt });
            cnt++;
        }

        public int pop() {
            while (removed.contains(stack.peek()[1])) {
                stack.pop();
            }
            int[] top = stack.pop();
            removed.add(top[1]);
            return top[0];
        }

        public int top() {
            while (removed.contains(stack.peek()[1])) {
                stack.pop();
            }
            return stack.peek()[0];
        }

        public int peekMax() {
            while (removed.contains(heap.peek()[1])) {
                heap.poll();
            }
            return heap.peek()[0];
        }

        public int popMax() {
            while (removed.contains(heap.peek()[1])) {
                heap.poll();
            }
            int[] top = heap.poll();
            removed.add(top[1]);
            return top[0];
        }
    }

    /**
     * Your MaxStack object will be instantiated and called as such:
     * MaxStack obj = new MaxStack();
     * obj.push(x);
     * int param_2 = obj.pop();
     * int param_3 = obj.top();
     * int param_4 = obj.peekMax();
     * int param_5 = obj.popMax();
     */

    // https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree-ii/
    /**
     * Definition for a binary tree node.
     * public class TreeNode {
     * int val;
     * TreeNode left;
     * TreeNode right;
     * TreeNode(int x) { val = x; }
     * }
     */
    static class Solution4 {
        public TreeNode lowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q) {
            TreeNode ans = LCA(root, p, q);
            if (ans == p) {
                return dfs(p, q) ? p : null;
            } else if (ans == q) {
                return dfs(q, p) ? q : null;
            }
            return ans;
        }

        TreeNode LCA(TreeNode node, TreeNode p, TreeNode q) {
            if (node == null || node == p || node == q)
                return node;
            TreeNode left = LCA(node.left, p, q);
            TreeNode right = LCA(node.right, p, q);
            if (left != null && right != null)
                return node;
            else if (left != null)
                return left;
            return right;
        }

        boolean dfs(TreeNode node, TreeNode target) {
            if (node == target)
                return true;
            if (node == null)
                return false;
            return dfs(node.left, target) || dfs(node.right, target);
        }
    }

    /**
     * Definition for a binary tree node.
     * public class TreeNode {
     * int val;
     * TreeNode left;
     * TreeNode right;
     * TreeNode(int x) { val = x; }
     * }
     */
    static class Solution5 {

        boolean nodesFound = false;

        public TreeNode lowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q) {
            TreeNode ans = dfs(root, p, q);
            return nodesFound ? ans : null;
        }

        TreeNode dfs(TreeNode node, TreeNode p, TreeNode q) {
            if (node == null)
                return null;
            TreeNode left = dfs(node.left, p, q);
            TreeNode right = dfs(node.right, p, q);

            int conditions = 0;
            if (node == p || node == q)
                conditions++;
            if (left != null)
                conditions++;
            if (right != null)
                conditions++;
            if (conditions == 2)
                nodesFound = true;

            if ((left != null && right != null) || node == p || node == q)
                return node;
            return left != null ? left : right;
        }
    }
}
