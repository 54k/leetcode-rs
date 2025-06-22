package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day946 {
    // https://leetcode.com/problems/divide-a-string-into-groups-of-size-k/description/?envType=daily-question&envId=2025-06-22
    static class Solution1 {
        public String[] divideString(String s, int k, char fill) {
            List<String> res = new ArrayList<>(); // grouped string
            int n = s.length();
            int curr = 0; // starting index of each group
            // split string
            while (curr < n) {
                int end = Math.min(curr + k, n);
                res.add(s.substring(curr, end));
                curr += k;
            }
            // try to fill in the last group
            String last = res.get(res.size() - 1);
            if (last.length() < k) {
                last += String.valueOf(fill).repeat(k - last.length());
                res.set(res.size() - 1, last);
            }
            return res.toArray(new String[0]);
        }
    }
}
