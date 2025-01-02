package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;

public class Day775 {
    // https://leetcode.com/problems/perform-string-shifts/description/?envType=weekly-question&envId=2025-01-01
    static class Solution1 {
        public int[] vowelStrings(String[] words, int[][] queries) {
            int[] ans = new int[queries.length];
            HashSet<Character> vowels = new HashSet<>(Arrays.asList('a', 'e', 'i', 'o', 'u'));
            int[] prefixSum = new int[words.length];
            int sum = 0;
            for (int i = 0; i < words.length; i++) {
                String currentWord = words[i];
                if (vowels.contains(currentWord.charAt(0))
                        && vowels.contains(currentWord.charAt(currentWord.length() - 1))) {
                    sum++;
                }
                prefixSum[i] = sum;
            }

            for (int i = 0; i < queries.length; i++) {
                int[] currentQuery = queries[i];
                ans[i] = prefixSum[currentQuery[1]] - (currentQuery[0] == 0 ? 0 : prefixSum[currentQuery[0] - 1]);
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/jump-game-vii/description/
    static class Solution2 {
        public boolean canReach(String s, int minJump, int maxJump) {
            int n = s.length(), pre = 0;
            boolean[] dp = new boolean[n];
            dp[0] = true;
            for (int i = 1; i < n; ++i) {
                if (i >= minJump && dp[i - minJump]) {
                    pre++;
                }
                if (i > maxJump && dp[i - maxJump - 1]) {
                    pre--;
                }
                dp[i] = pre > 0 && s.charAt(i) == '0';
            }
            return dp[n - 1];
        }
    }

    // https://leetcode.com/problems/perform-string-shifts/description/?envType=weekly-question&envId=2025-01-01
    static class Solution3 {
        public String stringShift(String s, int[][] shift) {
            int n = s.length();
            for (int[] move : shift) {
                int direction = move[0];
                int amount = move[1] % n;
                if (direction == 0) {
                    s = s.substring(amount) + s.substring(0, amount);
                } else {
                    s = s.substring(n - amount) + s.substring(0, n - amount);
                }
            }
            return s;
        }
    }

    // https://leetcode.com/problems/encode-and-decode-tinyurl/description/
    static public class Codec {
        long id = 0l;
        Map<String, String> urlToId = new HashMap<>();
        Map<String, String> idToUrl = new HashMap<>();

        // Encodes a URL to a shortened URL.
        public String encode(String longUrl) {
            if (urlToId.containsKey(longUrl)) {
                return urlToId.get(longUrl);
            }
            var sid = String.format("%s", id++);
            urlToId.put(longUrl, sid);
            idToUrl.put(sid, longUrl);
            return sid;
        }

        // Decodes a shortened URL to its original URL.
        public String decode(String shortUrl) {
            return idToUrl.get(shortUrl);
        }
    }

    // Your Codec object will be instantiated and called as such:
    // Codec codec = new Codec();
    // codec.decode(codec.encode(url));
}
