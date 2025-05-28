package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

public class Day921 {
    // https://leetcode.com/problems/maximize-the-number-of-target-nodes-after-connecting-trees-i/description/?envType=daily-question&envId=2025-05-28
    static class Solution1 {
        public int[] maxTargetNodes(int[][] edges1, int[][] edges2, int k) {
            int n = edges1.length + 1;
            int m = edges2.length + 1;

            int[] count1 = build(edges1, k);
            int[] count2 = build(edges2, k - 1);

            int maxCount2 = 0;
            for (int c : count2) {
                if (c > maxCount2) {
                    maxCount2 = c;
                }
            }
            int[] res = new int[n];
            for (int i = 0; i < n; i++) {
                res[i] = count1[i] + maxCount2;
            }
            return res;
        }

        int[] build(int[][] edges, int k) {
            int n = edges.length + 1;
            List<List<Integer>> children = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                children.add(new ArrayList<>());
            }
            for (int[] edge : edges) {
                children.get(edge[0]).add(edge[1]);
                children.get(edge[1]).add(edge[0]);
            }
            int[] res = new int[n];
            for (int i = 0; i < n; i++) {
                res[i] = dfs(i, -1, children, k);
            }
            return res;
        }

        int dfs(int node, int parent, List<List<Integer>> children, int k) {
            if (k < 0) {
                return 0;
            }
            int res = 1;
            for (int child : children.get(node)) {
                if (child == parent) {
                    continue;
                }
                res += dfs(child, node, children, k - 1);
            }
            return res;
        }
    }

    static class Solution2 {
        List<String> combinations = new ArrayList<>();
        Map<Character, String> letters = Map.of(
                '2',
                "abc",
                '3',
                "def",
                '4',
                "ghi",
                '5',
                "jkl",
                '6',
                "mno",
                '7',
                "pqrs",
                '8',
                "tuv",
                '9',
                "wxyz");

        String phoneDigits;

        public List<String> letterCombinations(String digits) {
            if (digits.length() == 0) {
                return combinations;
            }

            phoneDigits = digits;
            backtrack(0, new StringBuilder());
            return combinations;
        }

        void backtrack(int index, StringBuilder path) {
            if (path.length() == phoneDigits.length()) {
                combinations.add(path.toString());
                return;
            }

            String possibleLetters = letters.get(phoneDigits.charAt(index));
            for (char letter : possibleLetters.toCharArray()) {
                path.append(letter);
                backtrack(index + 1, path);
                path.deleteCharAt(path.length() - 1);
            }
        }
    }

    // https://leetcode.com/problems/permutations/description/
    static class Solution3 {
        public List<List<Integer>> permute(int[] nums) {
            List<List<Integer>> ans = new ArrayList<>();
            backtrack(new ArrayList<>(), ans, nums);
            return ans;
        }

        void backtrack(List<Integer> curr, List<List<Integer>> ans, int[] nums) {
            if (curr.size() == nums.length) {
                ans.add(new ArrayList<>(curr));
                return;
            }

            for (int num : nums) {
                if (!curr.contains(num)) {
                    curr.add(num);
                    backtrack(curr, ans, nums);
                    curr.remove(curr.size() - 1);
                }
            }
        }
    }

    // https://leetcode.com/problems/combinations/description/
    static class Solution4 {
        int n;
        int k;

        public List<List<Integer>> combine(int n, int k) {
            this.n = n;
            this.k = k;

            List<List<Integer>> ans = new ArrayList<>();
            backtrack(new ArrayList<>(), 1, ans);
            return ans;
        }

        void backtrack(List<Integer> curr, int firstNum, List<List<Integer>> ans) {
            if (curr.size() == k) {
                ans.add(new ArrayList<>(curr));
                return;
            }

            int need = k - curr.size();
            int remain = n - firstNum + 1;
            int available = remain - need;

            for (int num = firstNum; num <= firstNum + available; num++) {
                curr.add(num);
                backtrack(curr, num + 1, ans);
                curr.remove(curr.size() - 1);
            }
        }
    }

    // https://leetcode.com/problems/lru-cache/description/
    static class LRUCache1 {
        int capacity;
        LinkedHashMap<Integer, Integer> dic;

        public LRUCache1(int capacity) {
            this.capacity = capacity;
            dic = new LinkedHashMap<>(5, 0.75f, true) {
                @Override
                protected boolean removeEldestEntry(
                        Map.Entry<Integer, Integer> eldest) {
                    return size() > capacity;
                }
            };
        }

        public int get(int key) {
            return dic.getOrDefault(key, -1);
        }

        public void put(int key, int value) {
            dic.put(key, value);
        }
    }

    static class ListNode {
        int key;
        int val;
        ListNode next;
        ListNode prev;

        ListNode(int key, int val) {
            this.key = key;
            this.val = val;
        }
    }

    static class LRUCache2 {
        int capacity;
        Map<Integer, ListNode> dic;
        ListNode head;
        ListNode tail;

        public LRUCache2(int capacity) {
            this.capacity = capacity;
            dic = new HashMap<>();
            head = new ListNode(-1, -1);
            tail = new ListNode(-1, -1);
            head.next = tail;
            tail.prev = head;
        }

        public int get(int key) {
            if (!dic.containsKey(key)) {
                return -1;
            }
            ListNode node = dic.get(key);
            remove(node);
            add(node);
            return node.val;
        }

        public void put(int key, int value) {
            if (dic.containsKey(key)) {
                ListNode oldNode = dic.get(key);
                remove(oldNode);
            }

            ListNode node = new ListNode(key, value);
            dic.put(key, node);
            add(node);

            if (dic.size() > capacity) {
                ListNode nodeToDelete = head.next;
                remove(nodeToDelete);
                dic.remove(nodeToDelete.key);
            }
        }

        void add(ListNode node) {
            ListNode previousEnd = tail.prev;
            previousEnd.next = node;
            node.prev = previousEnd;
            node.next = tail;
            tail.prev = node;
        }

        void remove(ListNode node) {
            node.prev.next = node.next;
            node.next.prev = node.prev;
        }
    }

}
