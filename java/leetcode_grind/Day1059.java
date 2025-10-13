package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.LinkedList;
import java.util.List;

public class Day1059 {
    // https://leetcode.com/problems/find-resultant-array-after-removing-anagrams/description/?envType=daily-question&envId=2025-10-13
    static class Solution1 {
        public List<String> removeAnagrams(String[] words) {
            List<String> res = new ArrayList<>();
            res.add(words[0]);
            int n = words.length;
            for (int i = 1; i < n; i++) {
                if (!compare(words[i], words[i - 1])) {
                    res.add(words[i]);
                }
            }
            return res;
        }

        boolean compare(String word1, String word2) {
            int[] freq = new int[26];
            for (char ch : word1.toCharArray()) {
                freq[ch - 'a']++;
            }
            for (char ch : word2.toCharArray()) {
                freq[ch - 'a']--;
            }
            for (int x : freq) {
                if (x != 0) {
                    return false;
                }
            }
            return true;
        }
    }

    // https://leetcode.com/problems/serialize-and-deserialize-binary-tree/description/
    /**
     * Definition for a binary tree node.
     * public class TreeNode {
     * int val;
     * TreeNode left;
     * TreeNode right;
     * TreeNode(int x) { val = x; }
     * }
     */
    static public class Codec {

        // Encodes a tree to a single string.
        public String serialize(TreeNode root) {
            return rserialize(root, "");
        }

        String rserialize(TreeNode root, String str) {
            if (root == null) {
                str += "null,";
            } else {
                str += String.valueOf(root.val) + ",";
                str = rserialize(root.left, str);
                str = rserialize(root.right, str);
            }
            return str;
        }

        // Decodes your encoded data to tree.
        public TreeNode deserialize(String data) {
            String[] data_array = data.split(",");
            List<String> data_list = new LinkedList<>(Arrays.asList(data_array));
            return rdeserialize(data_list);
        }

        TreeNode rdeserialize(List<String> l) {
            if (l.get(0).equals("null")) {
                l.remove(0);
                return null;
            }

            TreeNode root = new TreeNode(Integer.valueOf(l.get(0)));
            l.remove(0);
            root.left = rdeserialize(l);
            root.right = rdeserialize(l);
            return root;
        }
    }

    // Your Codec object will be instantiated and called as such:
    // Codec ser = new Codec();
    // Codec deser = new Codec();
    // TreeNode ans = deser.deserialize(ser.serialize(root));

}
