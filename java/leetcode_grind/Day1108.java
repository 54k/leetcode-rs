package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;

public class Day1108 {
    // https://leetcode.com/problems/maximum-running-time-of-n-computers/description/?envType=daily-question&envId=2025-12-01
    static class Solution1 {
        public long maxRunTime(int n, int[] batteries) {
            Arrays.sort(batteries);
            long extra = 0;
            for (int i = 0; i < batteries.length - n; i++) {
                extra += batteries[i];
            }

            int[] live = Arrays.copyOfRange(batteries, batteries.length - n, batteries.length);

            for (int i = 0; i < n - 1; i++) {
                if (extra < (long) (i + 1) * (live[i + 1] - live[i])) {
                    return live[i] + extra / (long) (i + 1);
                }

                extra -= (long) (i + 1) * (live[i + 1] - live[i]);
            }

            return live[n - 1] + extra / n;
        }
    }

    static class Solution2 {
        public long maxRunTime(int n, int[] batteries) {
            long sumPower = 0;
            for (int power : batteries) {
                sumPower += power;
            }
            long left = 1, right = sumPower / n;

            while (left < right) {
                long target = right - (right - left) / 2;
                long extra = 0;

                for (int power : batteries) {
                    extra += Math.min(power, target);
                }

                if (extra >= (long) (n * target)) {
                    left = target;
                } else {
                    right = target - 1;
                }
            }

            return left;
        }
    }

    // https://leetcode.com/problems/two-sum-bsts/description/?envType=weekly-question&envId=2025-12-01
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

    static class Solution3 {
        boolean binarySearch(TreeNode root2, int target2) {
            if (root2 == null) {
                return false;
            }
            if (root2.val == target2) {
                return true;
            } else if (root2.val > target2) {
                return binarySearch(root2.left, target2);
            } else {
                return binarySearch(root2.right, target2);
            }
        }

        boolean dfs(TreeNode root1, TreeNode root2, int target) {
            if (root1 == null) {
                return false;
            }
            if (binarySearch(root2, target - root1.val)) {
                return true;
            }
            return dfs(root1.left, root2, target) || dfs(root1.right, root2, target);
        }

        public boolean twoSumBSTs(TreeNode root1, TreeNode root2, int target) {
            return dfs(root1, root2, target);
        }
    }

    // https://leetcode.com/problems/implement-magic-dictionary/description/
    static class MagicDictionary {
        Set<String> words;
        Map<String, Integer> count;

        public MagicDictionary() {
            words = new HashSet<>();
            count = new HashMap<>();
        }

        List<String> generalizeNeighbors(String word) {
            List<String> ans = new ArrayList<>();
            char[] ca = word.toCharArray();
            for (int i = 0; i < word.length(); i++) {
                char letter = ca[i];
                ca[i] = '*';
                String magic = new String(ca);
                ans.add(magic);
                ca[i] = letter;
            }
            return ans;
        }

        public void buildDict(String[] words) {
            for (String word : words) {
                this.words.add(word);
                for (String nei : generalizeNeighbors(word)) {
                    count.put(nei, count.getOrDefault(nei, 0) + 1);
                }
            }
        }

        public boolean search(String searchWord) {
            for (String nei : generalizeNeighbors(searchWord)) {
                int c = count.getOrDefault(nei, 0);
                if (c > 1 || c == 1 && !words.contains(searchWord)) {
                    return true;
                }
            }
            return false;
        }
    }

}
