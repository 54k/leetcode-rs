package leetcode_grind;

import java.util.Arrays;

public class Day991 {
    // https://leetcode.com/problems/fruits-into-baskets-iii/description/?envType=daily-question&envId=2025-08-06
    static class Solution1 {
        public int numOfUnplacedFruits(int[] fruits, int[] baskets) {
            int n = baskets.length;
            int m = (int) Math.sqrt(n);
            int section = (n + m - 1) / m;
            int count = 0;
            int[] maxV = new int[section];
            Arrays.fill(maxV, 0);

            for (int i = 0; i < n; i++) {
                maxV[i / m] = Math.max(maxV[i / m], baskets[i]);
            }

            for (int fruit : fruits) {
                int sec;
                int unset = 1;
                for (sec = 0; sec < section; sec++) {
                    if (maxV[sec] < fruit) {
                        continue;
                    }
                    int choose = 0;
                    maxV[sec] = 0;
                    for (int i = 0; i < m; i++) {
                        int pos = sec * m + i;
                        if (pos < n && baskets[pos] >= fruit && choose == 0) {
                            baskets[pos] = 0;
                            choose = 1;
                        }
                        if (pos < n) {
                            maxV[sec] = Math.max(maxV[sec], baskets[pos]);
                        }
                    }
                    unset = 0;
                    break;
                }
                count += unset;
            }
            return count;
        }
    }

    static class Solution2 {
        int[] segTree = new int[400007];
        int[] baskets;

        void build(int p, int l, int r) {
            if (l == r) {
                segTree[p] = baskets[l];
                return;
            }
            int mid = (l + r) >> 1;
            build(p << 1, l, mid);
            build((p << 1) | 1, mid + 1, r);
            segTree[p] = Math.max(segTree[p << 1], segTree[(p << 1) | 1]);
        }

        int query(int p, int l, int r, int ql, int qr) {
            if (ql > r || qr < l) {
                return Integer.MIN_VALUE;
            }
            if (ql <= l && r <= qr) {
                return segTree[p];
            }
            int mid = (l + r) >> 1;
            return Math.max(query(p << 1, l, mid, ql, qr), query((p << 1) | 1, mid + 1, r, ql, qr));
        }

        void update(int p, int l, int r, int pos, int val) {
            if (l == r) {
                segTree[p] = val;
                return;
            }
            int mid = (l + r) >> 1;
            if (pos <= mid) {
                update(p << 1, l, mid, pos, val);
            } else {
                update((p << 1) | 1, mid + 1, r, pos, val);
            }
            segTree[p] = Math.max(segTree[p << 1], segTree[(p << 1) | 1]);
        }

        public int numOfUnplacedFruits(int[] fruits, int[] baskets) {
            this.baskets = baskets;
            int m = baskets.length;
            int count = 0;
            if (m == 0) {
                return fruits.length;
            }
            Arrays.fill(segTree, Integer.MIN_VALUE);
            build(1, 0, m - 1);
            for (int i = 0; i < fruits.length; i++) {
                int l = 0;
                int r = m - 1;
                int res = -1;
                while (l <= r) {
                    int mid = (l + r) >> 1;
                    if (query(1, 0, m - 1, 0, mid) >= fruits[i]) {
                        res = mid;
                        r = mid - 1;
                    } else {
                        l = mid + 1;
                    }
                }
                if (res != -1 && baskets[res] >= fruits[i]) {
                    update(1, 0, m - 1, res, Integer.MIN_VALUE);
                } else {
                    count++;
                }
            }
            return count;
        }
    }
}
