package leetcode_grind;

import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;

public class Day433 {
    // https://leetcode.com/problems/longest-substring-with-at-most-k-distinct-characters/
    static class Solution1 {
        public int lengthOfLongestSubstringKDistinct(String s, int k) {
            var map = new HashMap<Character, Integer>();
            var ans = 0;
            for (int i = 0, j = 0; i < s.length(); i++) {
                map.put(s.charAt(i), i);

                if (map.size() > k) {
                    if (map.get(s.charAt(j)) == j) {
                        map.remove(s.charAt(j));
                    }
                    j++;
                }

                if (map.size() <= k) {
                    ans = Math.max(ans, i - j + 1);
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/longest-substring-without-repeating-characters/
    static class Solution2 {
        public int lengthOfLongestSubstring(String s) {
            var set = new HashSet<Character>();
            var ans = 0;
            for (int i = 0, j = 0; i < s.length(); i++) {
                while (set.contains(s.charAt(i))) {
                    set.remove(s.charAt(j));
                    j++;
                }
                set.add(s.charAt(i));
                ans = Math.max(ans, set.size());
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/subarrays-with-k-different-integers/
    static class Solution3 {
        public int subarraysWithKDistinct(int[] nums, int k) {
            return atMostK(nums, k) - atMostK(nums, k - 1);
        }

        int atMostK(int[] nums, int k) {
            int i = 0, res = 0;
            Map<Integer, Integer> count = new HashMap<>();
            for (int j = 0; j < nums.length; j++) {
                if (count.getOrDefault(nums[j], 0) == 0) {
                    k--;
                }
                count.put(nums[j], count.getOrDefault(nums[j], 0) + 1);

                while (k < 0) {
                    count.put(nums[i], count.get(nums[i]) - 1);
                    if (count.get(nums[i]) == 0) {
                        k++;
                    }
                    i++;
                }

                res += j - i + 1;
            }
            return res;

        }
    }

    // https://leetcode.com/problems/house-robber-iii/
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

    static class Solution4 {
        public int rob(TreeNode root) {
            var dfs = new Object() {
                int[] apply(TreeNode root) {
                    if (root == null) {
                        return new int[] { 0, 0 };
                    }
                    int[] l = apply(root.left);
                    int[] r = apply(root.right);

                    var t = root.val + l[1] + r[1];
                    var nt = Math.max(l[0], l[1]) + Math.max(r[0], r[1]);

                    return new int[] { t, nt };
                }
            };

            int[] ans = dfs.apply(root);
            return Math.max(ans[0], ans[1]);
        }
    }

    // https://leetcode.com/problems/reverse-linked-list/description/
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

    static class Solution5 {
        public ListNode reverseList(ListNode head) {
            if (head == null || head.next == null) {
                return head;
            }
            var p = reverseList(head.next);
            head.next.next = head;
            head.next = null;
            return p;
        }
    }
    // https://leetcode.com/problems/binary-tree-upside-down/

}
