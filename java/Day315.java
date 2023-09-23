import java.util.Arrays;
import java.util.HashMap;

public class Day315 {
    static class Solution {
        // https://leetcode.com/problems/longest-string-chain/description
        public int longestStrChain(String[] words) {
            var dp = new HashMap<String, Integer>();
            Arrays.sort(words, (a, b) -> a.length() - b.length());
            var longestSubSeqLen = 0;
            for (var word : words) {
                var presentLength = 1;
                for (var i = 0; i < word.length(); i++) {
                    var temp = new StringBuilder(word);
                    temp.deleteCharAt(i);
                    var prevSubSeqLen = dp.getOrDefault(temp.toString(), 0);
                    presentLength = Math.max(presentLength, prevSubSeqLen + 1);
                }
                dp.put(word, presentLength);
                longestSubSeqLen = Math.max(longestSubSeqLen, presentLength);
            }
            return longestSubSeqLen;
        }
    }
}
