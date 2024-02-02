package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day447 {
    // https://leetcode.com/problems/sequential-digits/description/
    static class Solution1 {
        public List<Integer> sequentialDigits(int low, int high) {
            String sample = "123456789";
            int n = 10;

            List<Integer> ans = new ArrayList<>();
            int lowLen = String.valueOf(low).length();
            int highLen = String.valueOf(high).length();

            for (int length = lowLen; length < highLen + 1; ++length) {
                for (int start = 0; start < n - length; ++start) {
                    int end = start + length;
                    int num = Integer.parseInt(sample.substring(start, end));

                    if (low <= num && num <= high) {
                        ans.add(num);
                    }
                }
            }

            return ans;
        }
    }
}
