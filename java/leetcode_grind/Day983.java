package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.Deque;
import java.util.LinkedList;
import java.util.List;
import java.util.PriorityQueue;
import java.util.Queue;

public class Day983 {
    // https://leetcode.com/problems/smallest-subarrays-with-maximum-bitwise-or/description/?envType=daily-question&envId=2025-07-29
    static class Solution1 {
        public int[] smallestSubarrays(int[] nums) {
            int n = nums.length;
            int[] pos = new int[31];
            Arrays.fill(pos, -1);
            int[] ans = new int[n];

            for (int i = n - 1; i >= 0; i--) {
                int j = i;
                for (int bit = 0; bit < 31; bit++) {
                    if ((nums[i] & (1 << bit)) == 0) {
                        if (pos[bit] != -1) {
                            j = Math.max(j, pos[bit]);
                        }
                    } else {
                        pos[bit] = i;
                    }
                }
                ans[i] = j - i + 1;
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/closest-binary-search-tree-value-ii/description/?envType=weekly-question&envId=2025-07-29
    static public class TreeNode {
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

    static class Solution2 {
        public List<Integer> closestKValues(TreeNode root, double target, int k) {
            List<Integer> arr = new ArrayList<>();
            dfs(root, arr);
            Collections.sort(arr, (o1, o2) -> Math.abs(o1 - target) <= Math.abs(o2 - target) ? -1 : 1);
            return arr.subList(0, k);
        }

        void dfs(TreeNode node, List<Integer> arr) {
            if (node == null) {
                return;
            }
            arr.add(node.val);
            dfs(node.left, arr);
            dfs(node.right, arr);
        }
    }

    static class Solution3 {
        public List<Integer> closestKValues(TreeNode root, double target, int k) {
            Queue<Integer> heap = new PriorityQueue<>((a, b) -> Math.abs(a - target) > Math.abs(b - target) ? -1 : 1);
            dfs(root, heap, k);
            return new ArrayList<>(heap);
        }

        void dfs(TreeNode node, Queue<Integer> heap, int k) {
            if (node == null) {
                return;
            }
            heap.add(node.val);
            if (heap.size() > k) {
                heap.remove();
            }
            dfs(node.left, heap, k);
            dfs(node.right, heap, k);
        }
    }

    static class Solution4 {
        public List<Integer> closestKValues(TreeNode root, double target, int k) {
            List<Integer> arr = new ArrayList<>();
            dfs(root, arr);

            int start = 0;
            double minDiff = Double.MAX_VALUE;

            for (int i = 0; i < arr.size(); i++) {
                if (Math.abs(arr.get(i) - target) < minDiff) {
                    minDiff = Math.abs(arr.get(i) - target);
                    start = i;
                }
            }

            int left = start;
            int right = start + 1;

            while (right - left - 1 < k) {
                if (left < 0) {
                    right += 1;
                    continue;
                }

                if (right == arr.size() || Math.abs(arr.get(left) - target) <= Math.abs(arr.get(right) - target)) {
                    left -= 1;
                } else {
                    right += 1;
                }
            }
            return arr.subList(left + 1, right);
        }

        void dfs(TreeNode node, List<Integer> arr) {
            if (node == null) {
                return;
            }
            dfs(node.left, arr);
            arr.add(node.val);
            dfs(node.right, arr);
        }
    }

    static class Solution5 {
        public List<Integer> closestKValues(TreeNode root, double target, int k) {
            List<Integer> arr = new ArrayList<>();
            dfs(root, arr);
            int left = 0;
            int right = arr.size() - k;
            while (left < right) {
                int mid = (left + right) / 2;
                if (Math.abs(target - arr.get(mid + k)) < Math.abs(target - arr.get(mid))) {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            return arr.subList(left, left + k);
        }

        void dfs(TreeNode node, List<Integer> arr) {
            if (node == null) {
                return;
            }
            dfs(node.left, arr);
            arr.add(node.val);
            dfs(node.right, arr);
        }
    }

    static class Solution6 {
        public List<Integer> closestKValues(TreeNode root, double target, int k) {
            Deque<Integer> queue = new LinkedList<>();
            dfs(root, queue, k, target);
            return new ArrayList<>(queue);
        }

        void dfs(TreeNode node, Deque<Integer> queue, int k, double target) {
            if (node == null) {
                return;
            }
            dfs(node.left, queue, k, target);
            queue.add(node.val);
            if (queue.size() > k) {
                if (Math.abs(target - queue.peekFirst()) <= Math.abs(target - queue.peekLast())) {
                    queue.removeLast();
                    return;
                } else {
                    queue.removeFirst();
                }
            }
            dfs(node.right, queue, k, target);
        }
    }
}
