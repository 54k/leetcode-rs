package leetcode_grind;

import java.util.*;

public class Day755 {
    // https://leetcode.com/problems/find-score-of-an-array-after-marking-all-elements/description/class
    static class Solution1 {
        public long findScore(int[] nums) {
            var marked = new HashSet<Integer>();
            var n = nums.length;
            var pq = new PriorityQueue<int[]>((a, b) -> {
                if (a[0] == b[0]) {
                    return Integer.compare(a[1], b[1]);
                }
                return a[0] - b[0];
            });
            for (int i = 0; i < n; i++) {
                pq.add(new int[] { nums[i], i });
            }
            var score = 0l;
            while (!pq.isEmpty()) {
                while (!pq.isEmpty() && marked.contains(pq.peek()[1])) {
                    pq.poll();
                }
                var cur = pq.poll();
                if (cur == null)
                    continue;
                score += cur[0];
                marked.add(cur[1]);
                marked.add(cur[1] + 1);
                marked.add(cur[1] - 1);
            }
            return score;
        }
    }

    // https://leetcode.com/problems/sort-integers-by-the-power-value/description/
    static class Solution2 {
        public int getKth(int lo, int hi, int k) {
            var comp = new int[1001][2];
            comp[0] = new int[] { 0, 0 };
            comp[1] = new int[] { 1, 0 };
            var compute = new Object() {
                int apply(int num) {
                    if (num < 2)
                        return 0;
                    if (num % 2 == 0) {
                        return 1 + apply(num / 2);
                    } else {
                        return 1 + apply(num * 3 + 1);
                    }
                }
            };

            for (int i = 2; i <= 1000; i++) {
                comp[i] = new int[] { i, compute.apply(i) };
            }

            var qs = new Object() {
                int apply(int l, int r, int kth) {
                    var pivi = r - 1;
                    int j = l;
                    for (int i = l; i < r; i++) {
                        if (compare(comp[i], comp[pivi]) < 0) {
                            var t = comp[i];
                            comp[i] = comp[j];
                            comp[j] = t;
                            j++;
                        }
                    }

                    var t = comp[pivi];
                    comp[pivi] = comp[j];
                    comp[j] = t;

                    if (j == kth) {
                        return comp[kth][0];
                    } else if (j > kth) {
                        return apply(l, j, kth);
                    } else {
                        return apply(j, r, kth);
                    }
                }

                int compare(int[] a, int[] b) {
                    if (a[1] == b[1]) {
                        return a[0] - b[0];
                    }
                    return a[1] - b[1];
                }
            };

            return qs.apply(lo, hi + 1, k + lo - 1);
        }
    }
}
