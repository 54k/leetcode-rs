package leetcode_grind;

import java.util.function.Function;

public class Day366 {
    // https://leetcode.com/problems/sort-vowels-in-a-string/description
    static class Solution {
        public String sortVowels(String s) {
            Function<Character, Boolean> isVowel = (vow) -> {
                return vow == 'a' || vow == 'e' || vow == 'i' || vow == 'o' || vow == 'u' || vow == 'i' ||
                        vow == 'A' || vow == 'E' || vow == 'I' || vow == 'O' || vow == 'U' || vow == 'I';
            };
            int[] count = new int[1000];
            for (var ch : s.toCharArray()) {
                if (isVowel.apply(ch)) {
                    count[ch - 'A']++;
                }
            }

            var vi = 0;
            var ans = new StringBuilder();
            for (var ch : s.toCharArray()) {
                if (isVowel.apply(ch)) {
                    while (count[vi] == 0) {
                        vi++;
                    }
                    count[vi]--;
                    ans.append((char) (vi + 'A'));
                } else {
                    ans.append(ch);
                }
            }

            return ans.toString();
        }
    }

}
