package data_structures_examples;

public class BinaryTreeOperations {
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

    // https://leetcode.com/problems/search-in-a-binary-search-tree/description/
    static class Solution1 {
        public TreeNode searchBST1(TreeNode root, int val) {
            if (root == null) {
                return root;
            }
            if (root.val > val) {
                return searchBST1(root.left, val);
            } else if (root.val < val) {
                return searchBST1(root.right, val);
            }
            return root;
        }

        public TreeNode searchBST2(TreeNode root, int val) {
            if (root == null) {
                return root;
            }

            while (root != null) {
                if (root.val > val) {
                    root = root.left;
                } else if (root.val < val) {
                    root = root.right;
                } else {
                    break;
                }
            }
            return root;
        }
    }

    // https://leetcode.com/problems/insert-into-a-binary-search-tree/description/
    static class Solution2 {
        public TreeNode insertIntoBST1(TreeNode root, int val) {
            if (root == null) {
                return new TreeNode(val);
            }
            if (root.val < val) {
                root.right = insertIntoBST1(root.right, val);
            } else if (root.val > val) {
                root.left = insertIntoBST1(root.left, val);
            }
            return root;
        }

        public TreeNode insertIntoBST2(TreeNode root, int val) {
            if (root == null) {
                return new TreeNode(val);
            }
            var r = root;
            while (r != null) {
                if (r.val > val) {
                    if (r.left == null) {
                        r.left = new TreeNode(val);
                        break;
                    }
                    r = r.left;
                } else if (r.val < val) {
                    if (r.right == null) {
                        r.right = new TreeNode(val);
                        break;
                    }
                    r = r.right;
                }
            }
            return root;
        }
    }

    // https://leetcode.com/problems/delete-node-in-a-bst/
    static class Solution3 {
        TreeNode findSuccessor(TreeNode root) {
            root = root.right;
            while (root.left != null) {
                root = root.left;
            }
            return root;
        }

        TreeNode findPredecessor(TreeNode root) {
            root = root.left;
            while (root.right != null) {
                root = root.right;
            }
            return root;
        }

        public TreeNode deleteNode1(TreeNode root, int key) {
            if (root == null) {
                return root;
            }

            if (root.val < key) {
                root.right = deleteNode1(root.right, key);
            } else if (root.val > key) {
                root.left = deleteNode1(root.left, key);
            } else {
                if (root.left == null) {
                    return root.right;
                } else if (root.right == null) {
                    return root.left;
                } else {
                    TreeNode successor = findSuccessor(root);
                    deleteNode1(root, successor.val);
                    root.val = successor.val;
                }
            }

            return root;
        }

        public TreeNode deleteNode2(TreeNode root, int key) {
            if (root == null) {
                return root;
            }

            if (root.val < key) {
                root.right = deleteNode2(root.right, key);
            } else if (root.val > key) {
                root.left = deleteNode2(root.left, key);
            } else {
                if (root.left == null && root.right == null) { // node is a leaf
                    return null;
                } else {
                    if (root.right != null) {
                        TreeNode succ = findSuccessor(root);
                        root.val = succ.val;
                        root.right = deleteNode2(root.right, root.val);
                    } else {
                        TreeNode pred = findPredecessor(root);
                        root.val = pred.val;
                        root.left = deleteNode2(root.left, root.val);
                    }
                }
            }

            return root;
        }
    }
}
