package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day654 {
    // https://leetcode.com/problems/combinations/description/
    static class Solution1 {
        public List<List<Integer>> combine(int n, int k) {
            var res = new ArrayList<List<Integer>>();
            var rec = new Object() {
                void apply(List<Integer> curr, int start) {
                    if (curr.size() == k) {
                        res.add(new ArrayList<>(curr));
                        return;
                    }
                    for (int i = start; i <= n; i++) {
                        curr.add(i);
                        apply(curr, i + 1);
                        curr.remove(curr.size() - 1);
                    }
                }
            };
            rec.apply(new ArrayList<>(), 1);
            return res;
        }
    }

    static class Solution2 {
        public List<List<Integer>> combine(int n, int k) {
            var res = new ArrayList<List<Integer>>();
            var rec = new Object() {
                void apply(List<Integer> curr, int start) {
                    if (curr.size() == k) {
                        res.add(new ArrayList<>(curr));
                        return;
                    }

                    int need = k - curr.size();
                    int remain = n - start + 1;
                    int available = remain - need;

                    for (int i = start; i <= start + available; i++) {
                        curr.add(i);
                        apply(curr, i + 1);
                        curr.remove(curr.size() - 1);
                    }
                }
            };
            rec.apply(new ArrayList<>(), 1);
            return res;
        }
    }

    // https://leetcode.com/problems/permutations/description/
    static class Solution3 {
        public List<List<Integer>> permute(int[] nums) {
            var cur = new ArrayList<Integer>();
            var res = new ArrayList<List<Integer>>();
            var rec = new Object() {
                void apply(int seen) {
                    if (cur.size() == nums.length) {
                        res.add(new ArrayList<>(cur));
                        return;
                    }
                    for (int i = 0; i < nums.length; i++) {
                        if (((seen >> i) & 1) == 0) {
                            cur.add(nums[i]);
                            apply(seen | (1 << i));
                            cur.remove(cur.size() - 1);
                        }
                    }
                }
            };
            rec.apply(0);
            return res;
        }
    }
}
