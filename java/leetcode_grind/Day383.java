package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;

public class Day383 {
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

    // https://leetcode.com/problems/average-of-levels-in-binary-tree/description/
    static class Solution1 {
        public List<Double> averageOfLevels(TreeNode root) {
            List<Double> ans = new ArrayList<>();
            List<TreeNode> cur = new ArrayList<>();
            cur.add(root);
            while (cur.size() > 0) {
                List<TreeNode> next = new ArrayList<>();
                long sum = 0;
                int n = cur.size();
                for (TreeNode tn : cur) {
                    sum += tn.val;
                    if (tn.left != null) {
                        next.add(tn.left);
                    }
                    if (tn.right != null) {
                        next.add(tn.right);
                    }
                }
                ans.add((double) sum / n);
                cur = next;
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/kill-process/description/
    static class Solution2 {
        public List<Integer> killProcess(List<Integer> pid, List<Integer> ppid, int kill) {
            var hm = new HashMap<Integer, List<Integer>>();
            for (var i = 0; i < pid.size(); i++) {
                hm.putIfAbsent(ppid.get(i), new ArrayList<>());
                hm.get(ppid.get(i)).add(pid.get(i));
            }
            var ans = new ArrayList<Integer>();
            var q = new ArrayList<Integer>();
            q.add(kill);
            while (q.size() > 0) {
                var n = new ArrayList<Integer>();
                for (var p : q) {
                    ans.add(p);
                    if (hm.containsKey(p)) {
                        for (var ch : hm.get(p)) {
                            n.add(ch);
                        }
                    }
                }
                q = n;
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/univalued-binary-tree/description/
    static class Solution3 {
        public boolean isUnivalTree(TreeNode root) {
            var leftOk = root.left == null || (root.left.val == root.val && isUnivalTree(root.left));
            var rightOk = root.right == null || (root.right.val == root.val && isUnivalTree(root.right));
            return leftOk && rightOk;
        }
    }
}
