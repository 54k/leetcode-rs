package data_structures_examples;

public class BIT {
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

        int[] arr;
        Fenwick fenwick;

        public NumArray(int[] nums) {
            arr = nums;
            fenwick = new Fenwick(nums);
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
