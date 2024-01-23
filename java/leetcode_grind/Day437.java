package leetcode_grind;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

class Solution {
    public int maxLength(List<String> arr) {
        List<String> results = new ArrayList<>();
        results.add("");
        int best = 0;

        for (String word : arr) {
            int resultsLen = results.size();
            for (int i = 0; i < resultsLen; i++) {
                String newRes = results.get(i) + word;
                Set<Character> newResSet = new HashSet<>();
                for (char c : newRes.toCharArray()) {
                    newResSet.add(c);
                }
                if (newRes.length() != newResSet.size()) {
                    continue;
                }

                results.add(newRes);
                best = Math.max(best, newRes.length());
            }
        }

        return best;
    }
}