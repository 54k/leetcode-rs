package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day396 {
    public class TreeNode {
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

    // https://leetcode.com/problems/find-duplicate-subtrees/description/
    static class Solution1 {
        public List<TreeNode> findDuplicateSubtrees1(TreeNode root) {
            Map<String, Integer> treeCounts = new HashMap<>();
            List<TreeNode> ans = new ArrayList<>();
            var dfs = new Object() {
                String apply(TreeNode root) {
                    if (root == null) {
                        return "";
                    }
                    String key = String.format("(%s)%s(%s)", apply(root.left), root.val, apply(root.right));
                    treeCounts.put(key, treeCounts.getOrDefault(key, 0) + 1);
                    if (treeCounts.get(key) == 2) {
                        ans.add(root);
                    }
                    return key;
                }
            };
            dfs.apply(root);
            return ans;
        }

        public List<TreeNode> findDuplicateSubtrees2(TreeNode root) {
            Map<Integer, Integer> treeCounts = new HashMap<>();
            Map<String, Integer> tripletToId = new HashMap<>();

            List<TreeNode> ans = new ArrayList<>();
            var dfs = new Object() {
                int apply(TreeNode root) {
                    if (root == null) {
                        return 0;
                    }
                    String triplet = String.format("%s,%s,%s", apply(root.left), root.val, apply(root.right));
                    if (!tripletToId.containsKey(triplet)) {
                        tripletToId.put(triplet, tripletToId.size() + 1);
                    }
                    int id = tripletToId.get(triplet);

                    treeCounts.put(id, treeCounts.getOrDefault(id, 0) + 1);

                    if (treeCounts.get(id) == 2) {
                        ans.add(root);
                    }
                    return id;
                }
            };
            dfs.apply(root);
            return ans;
        }
    }
}
