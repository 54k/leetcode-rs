package leetcode_grind;

import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.Map;
import java.util.Queue;
import java.util.Set;

public class Day513 {

    // https://leetcode.com/problems/wiggle-subsequence/description/
    static class Solution3 {
        public int wiggleMaxLength(int[] nums) {
            if (nums.length < 2) {
                return nums.length;
            }

            int[] up = new int[nums.length];
            int[] down = new int[nums.length];

            for (int i = 1; i < nums.length; i++) {
                for (int j = 0; j < i; j++) {
                    if (nums[i] > nums[j]) {
                        up[i] = Math.max(up[i], down[j] + 1);
                    } else if (nums[i] < nums[j]) {
                        down[i] = Math.max(down[i], up[j] + 1);
                    }
                }
            }
            return 1 + Math.max(down[nums.length - 1], up[nums.length - 1]);
        }
    }

    // https://leetcode.com/problems/longest-string-chain/description
    static class Solution2 {
        public int longestStrChain(String[] words) {
            Map<String, Integer> memo = new HashMap<>();
            Set<String> wordsPresent = new HashSet<>();
            Collections.addAll(wordsPresent, words);
            int ans = 0;

            var dfs = new Object() {
                int apply(String currentWord) {
                    if (memo.containsKey(currentWord)) {
                        return memo.get(currentWord);
                    }

                    int maxLength = 1;
                    StringBuilder sb = new StringBuilder(currentWord);

                    for (int i = 0; i < currentWord.length(); i++) {
                        sb.deleteCharAt(i);
                        String newWord = sb.toString();

                        if (wordsPresent.contains(newWord)) {
                            int currentLength = 1 + apply(newWord);
                            maxLength = Math.max(maxLength, currentLength);
                        }
                        sb.insert(i, currentWord.charAt(i));
                    }

                    memo.put(currentWord, maxLength);
                    return maxLength;
                }
            };

            for (String word : words) {
                ans = Math.max(ans, dfs.apply(word));
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/time-needed-to-buy-tickets/
    static class Solution1 {
        public int timeRequiredToBuy(int[] tickets, int k) {
            Queue<Integer> queue = new LinkedList<>();

            for (int i = 0; i < tickets.length; i++) {
                queue.add(i);
            }

            int time = 0;

            while (!queue.isEmpty()) {
                time++;

                int front = queue.poll();
                tickets[front]--;

                if (k == front && tickets[front] == 0) {
                    return time;
                }

                if (tickets[front] != 0) {
                    queue.add(front);
                }
            }

            return time;
        }

        public int timeRequiredToBuy2(int[] tickets, int k) {
            int time = 0;

            for (int i = 0; i < tickets.length; i++) {
                if (i <= k) {
                    time += Math.min(tickets[k], tickets[i]);
                } else {
                    time += Math.min(tickets[k] - 1, tickets[i]);
                }
            }

            return time;
        }
    }
}
