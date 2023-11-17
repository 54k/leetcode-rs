package leetcode_grind;

import java.util.ArrayDeque;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;

public class Day369 {
    // https://leetcode.com/problems/find-unique-binary-string/description/
    static class Solution {
        public String findDifferentBinaryString(String[] nums) {
            var len = new int[] { 0 };
            Set<String> set = new HashSet<>();
            for (var s : nums) {
                len[0] = s.length();
                set.add(s);
            }

            var bc = new Object() {
                String go(String cur) {
                    if (cur.length() == len[0] && !set.contains(cur)) {
                        return cur;
                    }
                    if (cur.length() == len[0]) {
                        return null;
                    }

                    for (var n : new String[] { "0", "1" }) {
                        var res = go(cur + n);
                        if (res != null)
                            return res;
                    }
                    return null;
                }
            };
            return bc.go("");
        }
    }

    // https://leetcode.com/problems/word-break/description
    static class Solution2 {
        // https://leetcode.com/problems/word-break/description/
        public boolean wordBreak1(String s, List<String> wordDict) {
            var words = new HashSet<String>(wordDict);

            var seen = new HashSet<Integer>();
            var queue = new ArrayDeque<Integer>();
            queue.addLast(0);

            while (!queue.isEmpty()) {
                var start = queue.pollFirst();
                if (start == s.length()) {
                    return true;
                }
                if (!seen.add(start)) {
                    continue;
                }

                for (var w : words) {
                    var ss = s.substring(start, Math.min(start + w.length(), s.length()));
                    if (ss.equals(w)) {
                        queue.addLast(start + w.length());
                    }
                }
            }

            return false;
        }

        public boolean wordBreak2(String s, List<String> wordDict) {
            var memo = new Boolean[s.length()];
            var dp = new Object() {
                boolean apply(int start) {
                    if (start >= s.length()) {
                        return start == s.length();
                    }
                    if (memo[start] != null) {
                        return memo[start];
                    }
                    for (var word : wordDict) {
                        if (s.substring(start, Math.min(s.length(), start + word.length())).equals(word)) {
                            if (apply(start + word.length())) {
                                return memo[start] = true;
                            }
                        }
                    }
                    return memo[start] = false;
                }
            };

            return dp.apply(0);
        }

        public boolean wordBreak3(String s, List<String> wordDict) {
            var dp = new boolean[s.length()];

            for (int i = 0; i < s.length(); i++) {
                for (var word : wordDict) {
                    if (i < word.length() - 1)
                        continue;

                    if (i == word.length() - 1 || dp[i - word.length()]) {
                        var ss = s.substring(i - word.length() + 1, i + 1);
                        dp[i] |= ss.equals(word);
                    }
                }
            }

            return dp[s.length() - 1];
        }

        public boolean wordBreak4(String s, List<String> wordDict) {
            class TrieNode {
                Map<Character, TrieNode> children = new HashMap<>();
                boolean isEnd;
            }

            var trieRoot = new TrieNode();
            for (var word : wordDict) {
                var node = trieRoot;
                for (var ch : word.toCharArray()) {
                    if (!node.children.containsKey(ch)) {
                        node.children.put(ch, new TrieNode());
                    }
                    node = node.children.get(ch);
                }
                node.isEnd = true;
            }

            var dp = new boolean[s.length()];
            for (int i = 0; i < s.length(); i++) {
                if (i > 0 && !dp[i - 1])
                    continue;

                var node = trieRoot;
                for (int j = i; j < s.length(); j++) {
                    if (!node.children.containsKey(s.charAt(j))) {
                        break;
                    }
                    node = node.children.get(s.charAt(j));
                    dp[j] |= node.isEnd;
                }
            }
            return dp[s.length() - 1];
        }

        public boolean wordBreak5(String s, List<String> wordDict) {
            var set = new HashSet<String>(wordDict);
            var dp = new boolean[s.length() + 1];
            dp[s.length()] = true;

            for (int i = s.length() - 1; i >= 0; i--) {
                for (int j = i + 1; j <= s.length(); j++) {
                    var ss = s.substring(i, j);
                    if (set.contains(ss)) {
                        dp[i] |= dp[j];
                    }
                }
            }
            return dp[0];
        }

        public boolean wordBreak6(String s, List<String> wordDict) {
            var set = new HashSet<String>(wordDict);
            var dp = new boolean[s.length() + 1];
            dp[0] = true;

            for (int i = 1; i <= s.length(); i++) {
                for (int j = 0; j < i; j++) {
                    var ss = s.substring(j, i);
                    if (dp[j] && set.contains(ss)) {
                        dp[i] = true;
                        break;
                    }
                }
            }
            return dp[s.length()];
        }
    }
}
