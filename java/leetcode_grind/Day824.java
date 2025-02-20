package leetcode_grind;

import java.util.HashSet;
import java.util.Random;
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

    static class Solution2 {
        public String findDifferentBinaryString(String[] nums) {
            Set<Integer> integers = new HashSet<>();
            for (String num : nums) {
                integers.add(Integer.parseInt(num, 2));
            }
            int n = nums.length;
            for (int num = 0; num <= n; num++) {
                if (!integers.contains(num)) {
                    String ans = Integer.toBinaryString(num);
                    while (ans.length() < n) {
                        ans = "0" + ans;
                    }
                    return ans;
                }
            }
            return "";
        }
    }

    static class Solution3 {
        public String findDifferentBinaryString(String[] nums) {
            Set<Integer> integers = new HashSet<>();
            for (String num : nums) {
                integers.add(Integer.parseInt(num, 2));
            }

            int ans = Integer.parseInt(nums[0], 2);
            int n = nums.length;
            Random rand = new Random();

            while (integers.contains(ans)) {
                ans = rand.nextInt((int) Math.pow(2, n));
            }

            String s = Integer.toBinaryString(ans);
            while (s.length() < n) {
                s = "0" + s;
            }
            return s;
        }
    }
}
