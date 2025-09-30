package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;

public class Day1046 {
    // https://leetcode.com/problems/find-triangular-sum-of-an-array/description/?envType=daily-question&envId=2025-09-30
    static class Solution1 {
        public int triangularSum(int[] nums) {
            var lst = Arrays.stream(nums).boxed().toList();
            while (lst.size() > 1) {
                var newLst = new ArrayList<Integer>();
                for (int i = 0; i < lst.size() - 1; i++) {
                    newLst.add((lst.get(i) + lst.get(i + 1)) % 10);
                }
                lst = newLst;
            }
            return lst.get(0);
        }
    }

}
