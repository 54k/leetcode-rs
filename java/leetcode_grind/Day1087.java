package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day1087 {
    // https://leetcode.com/problems/minimum-operations-to-convert-all-elements-to-zero/description/?envType=daily-question&envId=2025-11-10
    static class Solution1 {
        public int minOperations(int[] nums) {
            List<Integer> s = new ArrayList<>();
            int res = 0;
            for (int a : nums) {
                while (!s.isEmpty() && s.get(s.size() - 1) > a) {
                    s.remove(s.size() - 1);
                }
                if (a == 0)
                    continue;
                if (s.isEmpty() || s.get(s.size() - 1) < a) {
                    res++;
                    s.add(a);
                }
            }
            return res;
        }
    }
}
