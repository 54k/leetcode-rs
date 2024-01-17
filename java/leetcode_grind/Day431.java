package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;

public class Day431 {
    // https://leetcode.com/problems/valid-word-abbreviation/description/
    static class Solution1 {
        public boolean validWordAbbreviation(String word, String abbr) {
            int j = 0, i = 0, cur = 0;
            for (; i < abbr.length(); i++) {
                if (abbr.charAt(i) == '0' && cur == 0) {
                    return false;
                }

                if (Character.isDigit(abbr.charAt(i))) {
                    cur = cur * 10 + (abbr.charAt(i) - '0');
                } else {
                    j += cur;
                    cur = 0;
                    if (j > word.length() - 1) {
                        return false;
                    }
                    if (word.charAt(j) != abbr.charAt(i)) {
                        System.out.printf("%s %s\n", word.charAt(j), abbr.charAt(i));
                        return false;
                    }
                    j++;
                }
            }
            j += cur;
            return j == word.length();
        }
    }

    // https://leetcode.com/problems/unique-binary-search-trees-ii/
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

    static class Solution2 {
        TreeNode clone(TreeNode node, int offset) {
            if (node == null) {
                return null;
            }
            TreeNode clonedNode = new TreeNode(node.val + offset);
            clonedNode.left = clone(node.left, offset);
            clonedNode.right = clone(node.right, offset);
            return clonedNode;
        }

        public List<TreeNode> generateTrees(int n) {
            List<List<TreeNode>> dp = new ArrayList<>(n + 1);
            for (int i = 0; i <= n; i++) {
                dp.add(new ArrayList<>());
            }
            dp.get(0).add(null);

            for (int numberOfNodes = 1; numberOfNodes <= n; numberOfNodes++) {
                for (int i = 1; i <= numberOfNodes; i++) {
                    int j = numberOfNodes - i;

                    for (TreeNode left : dp.get(i - 1)) {
                        for (TreeNode right : dp.get(j)) {
                            TreeNode root = new TreeNode(i, left, clone(right, i));
                            dp.get(numberOfNodes).add(root);
                        }
                    }
                }
            }

            return dp.get(n);
        }

        public List<TreeNode> generateTrees2(int n) {
            List<List<List<TreeNode>>> dp = new ArrayList<>(n + 1);
            for (int i = 0; i <= n; i++) {
                List<List<TreeNode>> innerList = new ArrayList<>(n + 1);
                for (int j = 0; j <= n; j++) {
                    innerList.add(new ArrayList<>());
                }
                dp.add(innerList);
            }

            for (int i = 1; i <= n; i++) {
                dp.get(i).get(i).add(new TreeNode(i));
            }

            for (int numberOfNodes = 2; numberOfNodes <= n; numberOfNodes++) {
                for (int start = 1; start <= n - numberOfNodes + 1; start++) {
                    int end = start + numberOfNodes - 1;
                    for (int i = start; i <= end; i++) {
                        List<TreeNode> leftSubtrees = (i != start) ? dp.get(start).get(i - 1) : new ArrayList<>();
                        if (leftSubtrees.isEmpty()) {
                            leftSubtrees.add(null);
                        }
                        List<TreeNode> rightSubtrees = (i != end) ? dp.get(i + 1).get(end) : new ArrayList<>();
                        if (rightSubtrees.isEmpty()) {
                            rightSubtrees.add(null);
                        }

                        for (TreeNode left : leftSubtrees) {
                            for (TreeNode right : rightSubtrees) {
                                TreeNode root = new TreeNode(i, left, right);
                                dp.get(start).get(end).add(root);
                            }
                        }
                    }
                }
            }

            return dp.get(1).get(n);
        }

        public List<TreeNode> generateTrees3(int n) {
            var memo = new HashMap<int[], List<TreeNode>>();

            var dfs = new Object() {
                List<TreeNode> apply(int left, int right) {
                    var result = new ArrayList<TreeNode>();
                    if (left > right) {
                        result.add(null);
                        return result;
                    }

                    if (memo.containsKey(new int[] { left, right })) {
                        return memo.get(new int[] { left, right });
                    }

                    for (int i = left; i <= right; i++) {
                        var leftSubTrees = apply(left, i - 1);
                        var rightSubTrees = apply(i + 1, right);

                        for (var l : leftSubTrees) {
                            for (var r : rightSubTrees) {
                                var root = new TreeNode(i, l, r);
                                result.add(root);
                            }
                        }
                    }
                    memo.put(new int[] { left, right }, result);
                    return result;
                }
            };

            return dfs.apply(1, n);
        }
    }

}
