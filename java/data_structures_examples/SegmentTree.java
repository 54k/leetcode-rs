package data_structures_examples;

import java.util.HashMap;

public class SegmentTree {
    // https://leetcode.com/problems/range-sum-query-mutable/description/
    static class NumArray {
        class SegTree1 {
            int[] tree;
            int n;

            SegTree1(int[] nums) {
                n = nums.length;
                tree = new int[n * 4];
                int i = 0;
                for (int num : nums) {
                    update(1, 0, n - 1, i++, num);
                }
            }

            void update(int x, int tl, int tr, int i, int v) {
                if (tl == tr) {
                    tree[x] = v;
                    return;
                }

                int tm = (tl + tr) / 2;
                if (i <= tm) {
                    update(x * 2, tl, tm, i, v);
                } else {
                    update(x * 2 + 1, tm + 1, tr, i, v);
                }

                tree[x] = tree[x * 2] + tree[x * 2 + 1];
            }

            int sum(int k, int tl, int tr, int l, int r) {
                if (r < tl || l > tr) {
                    return 0;
                }
                if (l <= tl && tr <= r) {
                    return tree[k];
                }
                int tm = (tl + tr) / 2;
                int sl = sum(k * 2, tl, tm, l, r);
                int sr = sum(k * 2 + 1, tm + 1, tr, l, r);
                return sl + sr;
            }
        }

        class SegTree2 {
            int[] tree;
            int n;

            SegTree2(int[] nums) {
                n = nums.length;
                tree = new int[n * 2];
                int i = 0;
                for (int num : nums) {
                    update(i++, num);
                }
            }

            void update(int i, int v) {
                i += n;
                tree[i] = v;
                while (i > 0) {
                    i /= 2;
                    tree[i] = tree[i * 2] + tree[i * 2 + 1];
                }
            }

            int sum(int a, int b) {
                int sum = 0;
                a += n;
                b += n;
                while (a <= b) {
                    if (a % 2 == 1) {
                        sum += tree[a++];
                    }
                    if (b % 2 == 0) {
                        sum += tree[b--];
                    }
                    a /= 2;
                    b /= 2;
                }
                return sum;
            }
        }

        static class SegTree3 {
            static class Node {
                int val;
                Node left, right;

                Node(int v) {
                    val = v;
                }

                Node(Node l, Node r) {
                    left = l;
                    right = r;
                    val += left != null ? left.val : 0;
                    val += right != null ? right.val : 0;
                }
            }

            Node root;
            int n;

            SegTree3(int[] nums) {
                n = nums.length;
                root = build(0, n - 1, nums);
            }

            Node build(int tl, int tr, int[] nums) {
                if (tl == tr) {
                    return new Node(nums[tl]);
                }
                int tm = (tl + tr) / 2;
                return new Node(build(tl, tm, nums), build(tm + 1, tr, nums));
            }

            void update(int i, int val) {
                root = update(root, 0, n - 1, i, val);
            }

            Node update(Node v, int tl, int tr, int i, int val) {
                if (tl == tr) {
                    return new Node(val);
                }
                int tm = (tl + tr) / 2;
                if (i <= tm) {
                    return new Node(update(v.left, tl, tm, i, val), v.right);
                } else {
                    return new Node(v.left, update(v.right, tm + 1, tr, i, val));
                }
            }

            int sum(int l, int r) {
                return sum(root, 0, n - 1, l, r);
            }

            int sum(Node v, int tl, int tr, int l, int r) {
                if (l > r) {
                    return 0;
                }
                if (tl == l && tr == r) {
                    return v.val;
                }
                int tm = (tl + tr) / 2;
                return sum(v.left, tl, tm, l, Math.min(tm, r)) + sum(v.right, tm + 1, tr, Math.max(l, tm + 1), r);
            }
        }

        SegTree3 tree;

        public NumArray(int[] nums) {
            tree = new SegTree3(nums);
        }

        public void update(int index, int val) {
            // tree.update(1, 0, tree.n-1, index, val);
            tree.update(index, val);
        }

        public int sumRange(int left, int right) {
            // return tree.sum(1, 0, tree.n-1, left, right);
            return tree.sum(left, right);
        }
    }

    // https://leetcode.com/problems/range-frequency-queries/description/
    class RangeFreqQuery { // TLE
        static class SegTree {
            int n;
            HashMap<Integer, Integer>[] tree;

            SegTree(int[] arr) {
                n = arr.length;
                tree = new HashMap[4 * n];
                for (int i = 0; i < n; i++) {
                    update(1, 0, n - 1, i, arr[i]);
                }
            }

            void update(int v, int tl, int tr, int i, int val) {
                if (tl == tr) {
                    if (tree[v] == null) {
                        tree[v] = new HashMap<>();
                    }
                    tree[v].put(val, tree[v].getOrDefault(val, 0) + 1);
                    return;
                }

                int tm = (tl + tr) / 2;
                if (i <= tm) {
                    update(v * 2, tl, tm, i, val);
                } else {
                    update(v * 2 + 1, tm + 1, tr, i, val);
                }

                if (tree[v] == null) {
                    tree[v] = new HashMap<>();
                }
                if (tree[v * 2] == null) {
                    tree[v * 2] = new HashMap<>();
                }
                if (tree[v * 2 + 1] == null) {
                    tree[v * 2 + 1] = new HashMap<>();
                }

                tree[v].clear();
                for (var e : tree[v * 2].entrySet()) {
                    tree[v].put(e.getKey(), e.getValue());
                }
                for (var e : tree[v * 2 + 1].entrySet()) {
                    tree[v].put(e.getKey(), tree[v].getOrDefault(e.getKey(), 0) + e.getValue());
                }
            }

            int query(int v, int tl, int tr, int l, int r, int val) {
                if (r < tl || l > tr) {
                    return 0;
                }
                if (l <= tl && tr <= r) {
                    var vval = tree[v].get(val);
                    return vval == null ? 0 : vval;
                }
                int tm = (tl + tr) / 2;
                int fl = query(v * 2, tl, tm, l, r, val);
                int fr = query(v * 2 + 1, tm + 1, tr, l, r, val);
                return fl + fr;
            }
        }

        SegTree tree;

        public RangeFreqQuery(int[] arr) {
            tree = new SegTree(arr);
        }

        public int query(int left, int right, int value) {
            return tree.query(1, 0, tree.n - 1, left, right, value);
        }
    }
}
