package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day557 {
    // https://leetcode.com/problems/word-break-ii/description
    static class Solution1 {
        public List<String> wordBreak(String s, List<String> wordDict) {
            Map<Integer, List<String>> dp = new HashMap<>();
            for (int startIdx = s.length(); startIdx >= 0; startIdx--) {
                List<String> validSentences = new ArrayList<>();

                for (int endIdx = startIdx; endIdx < s.length(); endIdx++) {
                    String currentWord = s.substring(startIdx, endIdx + 1);

                    if (wordDict.contains(currentWord)) {
                        if (endIdx == s.length() - 1) {
                            validSentences.add(currentWord);
                        } else {
                            List<String> sentencesFromNextIndex = dp.get(endIdx + 1);
                            for (String sentence : sentencesFromNextIndex) {
                                validSentences.add(currentWord + " " + sentence);
                            }
                        }
                    }
                    dp.put(startIdx, validSentences);
                }
            }
            return dp.getOrDefault(0, new ArrayList<>());
        }
    }
}
