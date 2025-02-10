package leetcode_grind;

import java.util.*;

public class Day814 {
    // https://leetcode.com/problems/clear-digits/description/
    static class Solution1 {
        public String clearDigits(String s) {
            var n = s.length();
            var cnt = 0;
            var ans = new StringBuilder();
            for (int i = n - 1; i >= 0; i--) {
                var ch = s.charAt(i);
                if (Character.isDigit(ch)) {
                    cnt++;
                } else if (cnt > 0) {
                    cnt--;
                } else {
                    ans.append(ch);
                }
            }
            return ans.reverse().toString();
        }
    }

    // https://leetcode.com/problems/vowel-spellchecker/description/
    static class Solution2 {
        Set<String> words_perfect;
        Map<String, String> words_cap;
        Map<String, String> words_vow;

        public String[] spellchecker(String[] wordlist, String[] queries) {
            words_perfect = new HashSet<>();
            words_cap = new HashMap<>();
            words_vow = new HashMap<>();

            for (String word : wordlist) {
                words_perfect.add(word);

                String wordlow = word.toLowerCase();
                words_cap.putIfAbsent(wordlow, word);

                String wordlowDV = devowel(wordlow);
                words_vow.putIfAbsent(wordlowDV, word);
            }

            String[] ans = new String[queries.length];
            int t = 0;
            for (String query : queries) {
                ans[t++] = solve(query);
            }
            return ans;
        }

        String solve(String query) {
            if (words_perfect.contains(query)) {
                return query;
            }
            String queryL = query.toLowerCase();
            if (words_cap.containsKey(queryL)) {
                return words_cap.get(queryL);
            }
            String queryLV = devowel(queryL);
            if (words_vow.containsKey(queryLV)) {
                return words_vow.get(queryLV);
            }
            return "";
        }

        String devowel(String word) {
            StringBuilder ans = new StringBuilder();
            for (char c : word.toCharArray()) {
                ans.append(isVowel(c) ? '*' : c);
            }
            return ans.toString();
        }

        boolean isVowel(char c) {
            return (c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u');
        }
    }

    // https://leetcode.com/problems/find-x-sum-of-all-k-long-subarrays-ii/description/
    static class Solution3 {
        static class Pair implements Comparable<Pair> {
            int val;
            int freq;

            public Pair(int v, int f) {
                this.val = v;
                this.freq = f;
            }

            public int compareTo(Pair p) {
                if (this.freq == p.freq) {
                    return this.val - p.val;
                } else {

                    return this.freq - p.freq;
                }
            }
        }

        long sum = 0;
        HashMap<Integer, Integer> map = new HashMap<>();
        TreeSet<Pair> large = new TreeSet<>();
        TreeSet<Pair> small = new TreeSet<>();

        void update(int x, int v) {
            int freq = map.getOrDefault(x, 0);
            if (large.contains(new Pair(x, freq))) {
                large.remove(new Pair(x, freq));
                sum -= 1l * freq * x;
                map.put(x, freq + v);
                sum += 1l * map.get(x) * x;
                large.add(new Pair(x, map.get(x)));
            } else if (small.contains(new Pair(x, freq))) {
                small.remove(new Pair(x, freq));
                map.put(x, freq + v);
                small.add(new Pair(x, map.get(x)));
            }
        }

        void equilibrium(int x) {
            while (large.size() < x && !small.isEmpty()) {
                Pair second = small.last();
                large.add(second);
                sum += 1l * second.val * second.freq;
                small.remove(second);
            }

            if (small.isEmpty()) {
                return;
            }

            while (true) {
                Pair first = large.first();
                Pair second = small.last();

                if (first.freq < second.freq || (first.freq == second.freq && first.val < second.val)) {
                    large.remove(first);
                    small.remove(second);
                    large.add(second);
                    small.add(first);
                    sum -= 1l * first.val * first.freq;
                    sum += 1l * second.val * second.freq;
                } else {
                    break;
                }
            }
        }

        public long[] findXSum(int[] nums, int k, int x) {
            int n = nums.length;
            long ans[] = new long[n - k + 1];
            for (int i = 0; i < n; i++) {
                small.add(new Pair(nums[i], 0));
            }

            for (int i = 0; i < n; i++) {
                update(nums[i], 1);
                if (i >= k) {
                    update(nums[i - k], -1);
                }
                if (i >= k - 1) {
                    equilibrium(x);
                    ans[i - k + 1] = sum;
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/logger-rate-limiter/description/
    static class Logger {
        Map<String, Integer> map = new HashMap<>();

        public boolean shouldPrintMessage(int timestamp, String message) {
            if (map.getOrDefault(message, timestamp) <= timestamp) {
                map.put(message, timestamp + 10);
                return true;
            }
            return false;
        }
    }

    /**
     * Your Logger object will be instantiated and called as such:
     * Logger obj = new Logger();
     * boolean param_1 = obj.shouldPrintMessage(timestamp,message);
     */
}
