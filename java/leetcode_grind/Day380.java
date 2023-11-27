package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Stream;

public class Day380 {
    // https://leetcode.com/problems/shortest-word-distance-ii/description/
    static class WordDistance {
        Map<String, List<Integer>> dist = new HashMap<>();
        int n;

        public WordDistance(String[] wordsDict) {
            n = wordsDict.length;
            var i = 0;
            for (var w : wordsDict) {
                dist.putIfAbsent(w, new ArrayList<>());
                dist.get(w).add(i++);
            }
        }

        public int shortest(String word1, String word2) {
            var ans = n;
            var i = 0;
            var j = 0;

            var w1 = dist.get(word1);
            var w2 = dist.get(word2);

            while (i < w1.size() && j < w2.size()) {
                ans = Math.min(ans, Math.abs(w1.get(i) - w2.get(j)));
                if (i < w1.size() && w1.get(i) < w2.get(j)) {
                    i++;
                } else if (j < w2.size()) {
                    j++;
                }
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/shortest-word-distance/description/
    static class Solution1 {
        public int shortestDistance(String[] wordsDict, String word1, String word2) {
            var i1 = -1;
            var i2 = -1;
            var ans = wordsDict.length;
            for (int i = 0; i < wordsDict.length; i++) {
                if (wordsDict[i].equals(word1)) {
                    i1 = i;
                } else if (wordsDict[i].equals(word2)) {
                    i2 = i;
                }

                if (i1 != -1 && i2 != -1) {
                    ans = Math.min(ans, Math.abs(i1 - i2));
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/shortest-word-distance-iii/description/
    static class Solution2 {
        public int shortestWordDistance(String[] wordsDict, String word1, String word2) {
            var ind1 = new ArrayList<Integer>();
            var ind2 = new ArrayList<Integer>();

            for (int i = 0; i < wordsDict.length; i++) {
                if (wordsDict[i].equals(word1)) {
                    ind1.add(i);
                }
                if (wordsDict[i].equals(word2)) {
                    ind2.add(i);
                }
            }

            var upperBound = new Object() {
                int apply(int target) {
                    var index = ind2.size();

                    var lo = 0;
                    var hi = ind2.size() - 1;

                    while (lo <= hi) {
                        var mid = (lo + hi) / 2;

                        if (ind2.get(mid) > target) {
                            index = mid;
                            hi = mid - 1;
                        } else {
                            lo = mid + 1;
                        }
                    }
                    return index;
                }
            };

            var shortestDistance = Integer.MAX_VALUE;
            for (int idx : ind1) {
                int x = upperBound.apply(idx);

                if (x != ind2.size()) {
                    shortestDistance = Math.min(shortestDistance, ind2.get(x) - idx);
                }

                if (x != 0 && ind2.get(x - 1) != idx) {
                    shortestDistance = Math.min(shortestDistance, idx - ind2.get(x - 1));
                }
            }

            return shortestDistance;
        }
    }
}
