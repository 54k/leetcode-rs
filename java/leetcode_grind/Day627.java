package leetcode_grind;

import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;

public class Day627 {
    // https://leetcode.com/problems/kth-distinct-string-in-an-array/description/?envType=daily-question&envId=2024-08-05
    static class Solution1 {
        public String kthDistinct(String[] arr, int k) {
            var m = new LinkedHashMap<String, Integer>();
            for (var s : arr) {
                m.put(s, m.getOrDefault(s, 0) + 1);
            }
            for (var e : m.entrySet()) {
                if (e.getValue() == 1) {
                    k--;
                    if (k == 0) {
                        return e.getKey();
                    }
                }
            }
            return "";
        }
    }

    // https://leetcode.com/problems/alternating-groups-iii/description/
    static class Solution2 {
        private static final int SZ = 63333;
        private static final int OFFSET = SZ - 10;

        static class BIT {
            int[] bs = new int[SZ];

            // Update BIT: add value y to index x
            void update(int x, int y) {
                x = OFFSET - x;
                for (; x < SZ; x += x & -x) {
                    bs[x] += y;
                }
            }

            // Query BIT: get the prefix sum up to index x
            int query(int x) {
                x = OFFSET - x;
                int ans = 0;
                for (; x > 0; x -= x & -x) {
                    ans += bs[x];
                }
                return ans;
            }

            void clear(int x) {
                x = OFFSET - x;
                for (; x < SZ; x += x & -x) {
                    bs[x] = 0;
                }
            }
        }

        static BIT[] B = { new BIT(), new BIT() };

        static void edt(int x, int y) {
            B[1].update(x, x * y);
            B[0].update(x, y);
        }

        static int qry(int x) {
            return B[1].query(x) + (1 - x) * B[0].query(x);
        }

        static int len(int x, int y) {
            return y - x + 1;
        }

        static int len(Map.Entry<Integer, Integer> s) {
            return len(s.getKey(), s.getValue());
        }

        public List<Integer> numberOfAlternatingGroups(int[] colors, int[][] queries) {
            TreeMap<Integer, Integer> C = new TreeMap<>();
            int n = colors.length;

            for (int i = 0; i < colors.length; i++) {
                int r = i;
                while (r < colors.length && (colors[r] + colors[i] + r + i) % 2 == 0) {
                    ++r;
                }
                C.put(i, r - 1);
                edt(r - i, 1);
                i = r - 1;
            }

            List<Integer> results = new ArrayList<>();

            for (int[] q : queries) {
                if (q[0] == 1) {
                    int ans = qry(q[1]);
                    Map.Entry<Integer, Integer> A = C.firstEntry(), B = C.lastEntry();
                    if (A != B) {
                        if (colors[0] != colors[colors.length - 1]) {
                            int l1 = len(A.getKey(), A.getValue());
                            int l2 = len(B.getKey(), B.getValue());
                            ans -= Math.max(l1 - q[1] + 1, 0);
                            ans -= Math.max(l2 - q[1] + 1, 0);
                            ans += Math.max(l1 + l2 - q[1] + 1, 0);
                        }
                    } else if (colors[0] != colors[colors.length - 1]) {
                        ans = n;
                    }
                    results.add(ans);
                } else {
                    int x = q[1], y = q[2];
                    if (colors[x] == y) {
                        continue;
                    }
                    colors[x] = y;
                    Map.Entry<Integer, Integer> it = C.floorEntry(x);
                    assert it != null && it.getKey() <= x && it.getValue() >= x;
                    int L = it.getKey(), R = it.getValue();
                    edt(len(it.getKey(), it.getValue()), -1);
                    C.remove(it.getKey());
                    int ML = x, MR = x;
                    if (L != ML) {
                        C.put(L, x - 1);
                        edt(len(L, x - 1), 1);
                    } else {
                        if (x > 0 && colors[x] != colors[x - 1]) {
                            it = C.floorEntry(x - 1);
                            if (it != null) {
                                ML = it.getKey();
                                edt(len(it.getKey(), it.getValue()), -1);
                                C.remove(it.getKey());
                            }
                        }
                    }
                    if (R != MR) {
                        C.put(x + 1, R);
                        edt(len(x + 1, R), 1);
                    } else {
                        if (x + 1 < colors.length && colors[x + 1] != colors[x]) {
                            it = C.ceilingEntry(x + 1);
                            if (it != null) {
                                MR = it.getValue();
                                edt(len(it.getKey(), it.getValue()), -1);
                                C.remove(it.getKey());
                            }
                        }
                    }
                    C.put(ML, MR);
                    edt(len(ML, MR), 1);
                }
            }

            for (int i = 0; i <= n + 2; i++) {
                B[0].clear(i);
                B[1].clear(i);
            }

            return results;
        }
    }
}
