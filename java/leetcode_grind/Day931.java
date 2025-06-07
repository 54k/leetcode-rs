package leetcode_grind;

import java.util.ArrayDeque;
import java.util.Deque;

public class Day931 {
    // https://leetcode.com/problems/lexicographically-minimum-string-after-removing-stars/description/?envType=daily-question&envId=2025-06-07
    static class Solution {
        public String clearStars(String s) {
            Deque<Integer>[] cnt = new Deque[26];
            for (int i = 0; i < 26; i++) {
                cnt[i] = new ArrayDeque<>();
            }
            char[] arr = s.toCharArray();
            for (int i = 0; i < arr.length; i++) {
                if (arr[i] != '*') {
                    cnt[arr[i] - 'a'].push(i);
                } else {
                    for (int j = 0; j < 26; j++) {
                        if (!cnt[j].isEmpty()) {
                            arr[cnt[j].pop()] = '*';
                            break;
                        }
                    }
                }
            }

            StringBuilder ans = new StringBuilder();
            for (char c : arr) {
                if (c != '*') {
                    ans.append(c);
                }
            }
            return ans.toString();
        }
    }
}
