package leetcode_grind;

import java.util.*;

public class Day821 {

    // https://leetcode.com/problems/letter-tile-possibilities/submissions/1545784560/?envType=daily-question&envId=2025-02-17
    static class Solution1 {
        public int numTilePossibilities(String tiles) {
            int[] m = new int[26];
            for (var t : tiles.toCharArray()) {
                m[t - 'A']++;
            }
            Set<String> cache = new HashSet<>();
            var rec = new Object() {
                int apply(String seq) {
                    if (seq.length() >= tiles.length()) {
                        return 0;
                    }
                    int res = 0;
                    for (var letter : tiles.toCharArray()) {
                        if (m[letter - 'A']-- > 0) {
                            res += apply(seq + letter) + (cache.add(seq + letter) ? 1 : 0);
                        }
                        m[letter - 'A']++;
                    }

                    return res;
                }
            };

            return rec.apply("");
        }
    }
}
