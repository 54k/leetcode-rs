package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day783 {
    // https://leetcode.com/problems/word-subsets/description/?envType=daily-question&envId=2025-01-10
    static class Solution1 {
        public List<String> wordSubsets(String[] A, String[] B) {
            int[] bmax = count("");
            for (String b : B) {
                int[] bCount = count(b);
                for (int i = 0; i < 26; i++) {
                    bmax[i] = Math.max(bmax[i], bCount[i]);
                }
            }

            List<String> ans = new ArrayList<>();
            search: for (String a : A) {
                int[] aCount = count(a);
                for (int i = 0; i < 26; i++) {
                    if (aCount[i] < bmax[i]) {
                        continue search;
                    }
                }
                ans.add(a);
            }
            return ans;
        }

        int[] count(String S) {
            int[] ans = new int[26];
            for (char c : S.toCharArray()) {
                ans[c - 'a']++;
            }
            return ans;
        }
    }

}
