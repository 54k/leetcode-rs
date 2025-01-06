package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day779 {
    // https://leetcode.com/problems/minimum-number-of-operations-to-move-all-balls-to-each-box/description/?envType=daily-question&envId=2025-01-06
    static class Solution1 {
        public int[] minOperations(String boxes) {
            int[] answer = new int[boxes.length()];
            for (int currentBox = 0; currentBox < boxes.length(); currentBox++) {
                if (boxes.charAt(currentBox) == '1') {
                    for (int newPosition = 0; newPosition < boxes.length(); newPosition++) {
                        answer[newPosition] += Math.abs(newPosition - currentBox);
                    }
                }
            }
            return answer;
        }
    }

    // https://leetcode.com/problems/online-majority-element-in-subarray/description/?envType=problem-list-v2&envId=binary-indexed-tree
    class MajorityChecker {
        static class Node {
            int val;
            int freq;

            Node(int v, int f) {
                val = v;
                freq = f;
            }
        }

        int n;
        Node[] tree;
        Map<Integer, List<Integer>> idx = new HashMap<>();

        public MajorityChecker(int[] arr) {
            n = arr.length;
            tree = new Node[n * 4];
            build(0, 0, n, arr);
        }

        void build(int v, int l, int r, int[] arr) {
            if (l + 1 >= r) {
                tree[v] = new Node(arr[l], 1);
                idx.computeIfAbsent(arr[l], $ -> new ArrayList<>()).add(l);
                return;
            }

            int m = (l + r) / 2;
            build(v * 2 + 1, l, m, arr);
            build(v * 2 + 2, m, r, arr);
            tree[v] = merge(tree[v * 2 + 1], tree[v * 2 + 2]);
        }

        public int query(int left, int right, int threshold) {
            Node res = query(0, 0, n, left, right + 1);
            if (res.freq == 0)
                return -1;
            var s = search_lb(idx.get(res.val), left);
            var e = search_ub(idx.get(res.val), right);
            if (e - s < threshold)
                return -1;
            return res.val;
        }

        int search_lb(List<Integer> arr, int t) {
            int left = 0;
            int right = arr.size();
            while (left < right) {
                int mid = (left + right) / 2;
                if (arr.get(mid) < t) {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            return left;
        }

        int search_ub(List<Integer> arr, int t) {
            int left = 0;
            int right = arr.size();
            while (left < right) {
                int mid = (left + right) / 2;
                if (arr.get(mid) <= t) {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            return left;
        }

        Node query(int v, int l, int r, int x, int y) {
            if (l >= y || r <= x) {
                return new Node(0, 0);
            }
            if (x <= l && r <= y) {
                return tree[v];
            }
            int m = (l + r) / 2;
            Node ln = query(v * 2 + 1, l, m, x, y);
            Node rn = query(v * 2 + 2, m, r, x, y);
            return merge(ln, rn);
        }

        Node merge(Node l, Node r) {
            if (l.val == r.val) {
                return new Node(r.val, l.freq + r.freq);
            } else if (l.freq > r.freq) {
                return new Node(l.val, l.freq - r.freq);
            } else {
                return new Node(r.val, r.freq - l.freq);
            }
        }
    }

    /**
     * Your MajorityChecker object will be instantiated and called as such:
     * MajorityChecker obj = new MajorityChecker(arr);
     * int param_1 = obj.query(left,right,threshold);
     */
}
