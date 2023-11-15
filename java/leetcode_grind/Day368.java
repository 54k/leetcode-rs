package leetcode_grind;

public class Day368 {
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

        SegTree2 tree;

        public NumArray(int[] nums) {
            tree = new SegTree2(nums);
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
}
