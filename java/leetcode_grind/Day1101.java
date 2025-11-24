package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day1101 {
    // https://leetcode.com/problems/binary-prefix-divisible-by-5/description/?envType=daily-question&envId=2025-11-24
    static class Solution1 {
        public List<Boolean> prefixesDivBy5(int[] nums) {
            List<Boolean> answer = new ArrayList<>();
            int prefix = 0;
            int length = nums.length;
            for (int i = 0; i < length; i++) {
                prefix = ((prefix << 1) + nums[i]) % 5;
                answer.add(prefix == 0);
            }
            return answer;
        }
    }
}
