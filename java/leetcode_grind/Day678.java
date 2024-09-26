package leetcode_grind;

import java.util.*;

public class Day678 {
    // https://leetcode.com/problems/sum-of-prefix-scores-of-strings/description/?envType=daily-question&envId=2024-09-25
    static class Solution1 {
        static class TrieNode {
            TrieNode[] next = new TrieNode[26];
            int cnt = 0;
        }

        TrieNode root = new TrieNode();

        void insert(String word) {
            TrieNode node = root;
            for (char c : word.toCharArray()) {
                if (node.next[c - 'a'] == null) {
                    node.next[c - 'a'] = new TrieNode();
                }
                node.next[c - 'a'].cnt++;
                node = node.next[c - 'a'];
            }
        }

        int count(String s) {
            TrieNode node = root;
            int ans = 0;
            for (char c : s.toCharArray()) {
                ans += node.next[c - 'a'].cnt;
                node = node.next[c - 'a'];
            }
            return ans;
        }

        public int[] sumPrefixScores(String[] words) {
            int N = words.length;
            for (int i = 0; i < N; i++) {
                insert(words[i]);
            }
            int[] scores = new int[N];
            for (int i = 0; i < N; i++) {
                scores[i] = count(words[i]);
            }
            return scores;
        }
    }

    // https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/description/
    static class Solution2 {
        public int findMaximumXOR(int[] nums) {
            int maxNum = nums[0];
            for (int num : nums)
                maxNum = Math.max(maxNum, num);
            int L = (Integer.toBinaryString(maxNum)).length();
            int maxXor = 0, currXor;
            Set<Integer> prefixes = new HashSet<>();
            for (int i = L - 1; i > -1; i--) {
                maxXor <<= 1;
                currXor = maxXor | 1;
                prefixes.clear();

                for (int num : nums)
                    prefixes.add(num >> i);

                for (int p : prefixes) {
                    if (prefixes.contains(currXor ^ p)) {
                        maxXor = currXor;
                        break;
                    }
                }
            }
            return maxXor;
        }
    }
}
