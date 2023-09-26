package leetcode_grind;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;

public class Day315 {
    static class Solution1 {
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

    // https://leetcode.com/problems/minimum-window-subsequence/description/
    static class Solution2 {
        public String minWindow(String s1, String s2) {
            var n = s1.length();
            var m = s2.length();

            var answer = "";
            var indices = new HashMap<Character, List<Integer>>();

            for (var i = 0; i < n; i++) {
                var c = s1.charAt(i);
                indices.putIfAbsent(c, new ArrayList<>());
                indices.get(c).add(i);
            }

            var ind = new int[m];
            for (var start = 0; start < n; start++) {
                var prev = start - 1;
                for (var j = 0; j < m; j++) {
                    if (!indices.containsKey(s2.charAt(j))) {
                        return "";
                    }
                    var curIndices = indices.get(s2.charAt(j));
                    while (ind[j] < curIndices.size() && curIndices.get(ind[j]) <= prev) {
                        ind[j]++;
                    }
                    if (ind[j] == curIndices.size()) {
                        return answer;
                    }
                    prev = curIndices.get(ind[j]);
                }

                if (answer.isEmpty() || prev - start + 1 < answer.length()) {
                    answer = s1.substring(start, prev + 1);
                }
            }
            return answer;
        }
    }
}
