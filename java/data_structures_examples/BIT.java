package data_structures_examples;

public class BIT {
    // https://leetcode.com/problems/range-sum-query-mutable/description/
    class NumArray {
        static class Fenwick {
            int n;
            int[] t;

            Fenwick(int[] nums) {
                n = nums.length;
                t = new int[n];
                for (int i = 0; i < n; i++) {
                    update(i, nums[i]);
                }
            }

            void update(int i, int delta) {
                for (int j = i; j < n; j |= j + 1) {
                    t[j] += delta;
                }
            }

            private int sum(int r) {
                int s = 0;
                for (; r >= 0; r = (r & (r + 1)) - 1) {
                    s += t[r];
                }
                return s;
            }

            public int sum(int l, int r) {
                return sum(r) - sum(l - 1);
            }
        }

        static class Fenwick2 {
            int n;
            int[] t;

            Fenwick2(int[] nums) {
                n = nums.length;
                t = new int[n + 1];
                for (int i = 0; i < n; i++) {
                    update(i, nums[i]);
                }
            }

            void update(int i, int delta) {
                i++;
                while (i <= n) {
                    t[i] += delta;
                    i += i & -i;
                }
            }

            private int sum(int r) {
                r++;
                var s = 0;
                while (r > 0) {
                    s += t[r];
                    r -= r & -r;
                }
                return s;
            }

            int sum(int l, int r) {
                return sum(r + 1) - sum(l);
            }
        }

        int[] arr;
        Fenwick2 fenwick;

        public NumArray(int[] nums) {
            arr = nums;
            fenwick = new Fenwick2(nums);
        }

        public void update(int index, int val) {
            int delta = val - arr[index];
            arr[index] += delta;
            fenwick.update(index, delta);
        }

        public int sumRange(int left, int right) {
            return fenwick.sum(left, right);
        }
    }
}
