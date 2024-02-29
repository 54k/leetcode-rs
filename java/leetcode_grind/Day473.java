package leetcode_grind;

import java.util.HashMap;
import java.util.Map;
import java.util.Objects;

public class Day473 {
    // https://leetcode.com/problems/path-sum-iv/description/
    static class Solution1 {
        Map<Integer, Integer> mp = new HashMap<>();
        int ans = 0;

        int getIndex(int num) {
            var s = Objects.toString(num);
            return 10 * (int) (s.charAt(0) - '0') + (int) (s.charAt(1) - '0');
        }

        void traverse(int i, int sum) {
            if (!mp.containsKey(i)) {
                return;
            }
            int pos = 10 + ((i % 10) - 1);
            int l = i + pos;
            int r = i + pos + 1;

            sum += mp.get(i);
            // leaf
            if (!mp.containsKey(l) && !mp.containsKey(r)) {
                ans += sum;
                return;
            }

            traverse(l, sum);
            traverse(r, sum);
        }

        public int pathSum(int[] nums) {
            ans = 0;
            mp.clear();
            for (var i : nums) {
                int idx = getIndex(i);
                mp.put(idx, i % 10);
            }
            traverse(11, 0);
            return ans;
        }
    }
}
