package leetcode_grind;

import java.util.Arrays;

public class Day771 {
    // https://leetcode.com/problems/number-of-ways-to-form-a-target-string-given-a-dictionary/description/?envType=daily-question&envId=2024-12-29
    static class Solution1 {
        public int numWays(String[] words, String target) {
            int wordLength = words[0].length();
            int targetLength = target.length();

            int[][] dp = new int[wordLength][targetLength];
            for (int i = 0; i < wordLength; i++) {
                Arrays.fill(dp[i], -1);
            }

            int[][] charFrequency = new int[wordLength][26];

            for (String word : words) {
                for (int j = 0; j < wordLength; j++) {
                    int character = word.charAt(j) - 'a';
                    charFrequency[j][character]++;
                }
            }

            return (int) getWords(words, target, 0, 0, dp, charFrequency);
        }

        long getWords(String[] words, String target, int wordsIndex, int targetIndex, int[][] dp,
                int[][] charFrequency) {
            int MOD = 1000000007;

            if (targetIndex == target.length())
                return 1;

            if (wordsIndex == words[0].length() || words[0].length() - wordsIndex < target.length() - targetIndex) {
                return 0;
            }

            if (dp[wordsIndex][targetIndex] != -1) {
                return dp[wordsIndex][targetIndex];
            }

            long countWays = 0;
            int curPos = target.charAt(targetIndex) - 'a';

            countWays += getWords(words, target, wordsIndex + 1, targetIndex, dp, charFrequency);
            countWays += charFrequency[wordsIndex][curPos]
                    * getWords(words, target, wordsIndex + 1, targetIndex + 1, dp, charFrequency);

            dp[wordsIndex][targetIndex] = (int) (countWays % MOD);
            return dp[wordsIndex][targetIndex];
        }
    }

    static class Solution2 {
        public int numWays(String[] words, String target) {
            int wordLength = words[0].length();
            int targetLength = target.length();
            final int MOD = 1000000007;

            int[][] charFrequency = new int[wordLength][26];
            for (String word : words) {
                for (int j = 0; j < wordLength; j++) {
                    charFrequency[j][word.charAt(j) - 'a']++;
                }
            }

            long[][] dp = new long[wordLength + 1][targetLength + 1];
            for (int currWord = 0; currWord <= wordLength; ++currWord) {
                dp[currWord][0] = 1;
            }

            for (int currWord = 1; currWord <= wordLength; currWord++) {
                for (int currTarget = 1; currTarget <= targetLength; ++currTarget) {
                    dp[currWord][currTarget] = dp[currWord - 1][currTarget];

                    int curPos = target.charAt(currTarget - 1) - 'a';
                    dp[currWord][currTarget] += (charFrequency[currWord - 1][curPos] * dp[currWord - 1][currTarget - 1])
                            % MOD;
                    dp[currWord][currTarget] %= MOD;
                }
            }

            return (int) dp[wordLength][targetLength];
        }
    }
}
