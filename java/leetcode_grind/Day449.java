package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day449 {

    // https://leetcode.com/problems/minimum-window-substring/description
    static class Solution1 {
        public String minWindow(String s, String t) {
            if (s.length() == 0 || t.length() == 0) {
                return "";
            }

            Map<Character, Integer> mapT = new HashMap<>();
            for (char ch : t.toCharArray()) {
                mapT.put(ch, mapT.getOrDefault(ch, 0) + 1);
            }

            int desired = mapT.size(), formed = 0;
            int[] ans = { Integer.MAX_VALUE, 0, 0 };

            Map<Character, Integer> mapS = new HashMap<>();

            for (int r = 0, l = 0; r < s.length(); ++r) {
                char ch = s.charAt(r);
                mapS.put(ch, mapS.getOrDefault(ch, 0) + 1);

                if (mapT.containsKey(ch) && mapS.get(ch).intValue() == mapT.get(ch).intValue()) {
                    formed++;
                }

                while (l <= r && formed == desired) {
                    if (r - l + 1 < ans[0]) {
                        ans[0] = r - l + 1;
                        ans[1] = l;
                        ans[2] = r;
                    }

                    ch = s.charAt(l);
                    mapS.put(ch, mapS.getOrDefault(ch, 0) - 1);
                    if (mapT.containsKey(ch) && mapS.get(ch) < mapT.get(ch)) {
                        formed--;
                    }

                    l++;
                }
            }

            if (ans[0] == Integer.MAX_VALUE) {
                return "";
            }
            return s.substring(ans[1], ans[2] + 1);
        }
    }
}
