package leetcode_grind;

import java.util.HashSet;
import java.util.Set;

public class Day824 {
    // https://leetcode.com/problems/find-unique-binary-string/description/?envType=daily-question&envId=2025-02-20
    static class Solution1 {
        int n;
        Set<String> numSet = new HashSet<>();

        String generate(String curr) {
            if (curr.length() == n) {
                if (!numSet.contains(curr)) {
                    return curr;
                }
                return "";
            }
            String addZero = generate(curr + "0");
            if (addZero.length() > 0) {
                return addZero;
            }
            return generate(curr + "1");
        }

        public String findDifferentBinaryString(String[] nums) {
            n = nums.length;
            for (String s : nums) {
                numSet.add(s);
            }
            return generate("");
        }
    }

}
