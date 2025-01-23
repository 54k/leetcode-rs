package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class Day796 {
    // https://leetcode.com/problems/count-servers-that-communicate/description/?envType=daily-question&envId=2025-01-23
    static class Solution1 {
        public int countServers(int[][] grid) {
            int numRows = grid.length;
            int numCols = numRows > 0 ? grid[0].length : 0;
            int communicableServersCount = 0;

            for (int row = 0; row < numRows; ++row) {
                for (int col = 0; col < numCols; ++col) {
                    if (grid[row][col] == 1) {
                        boolean canCommunicate = false;

                        for (int otherCol = 0; otherCol < numCols; ++otherCol) {
                            if (otherCol != col && grid[row][otherCol] == 1) {
                                canCommunicate = true;
                                break;
                            }
                        }

                        if (canCommunicate) {
                            communicableServersCount++;
                        } else {
                            for (int otherRow = 0; otherRow < numRows; ++otherRow) {
                                if (otherRow != row && grid[otherRow][col] == 1) {
                                    canCommunicate = true;
                                    break;
                                }
                            }

                            if (canCommunicate) {
                                communicableServersCount++;
                            }
                        }
                    }
                }
            }
            return communicableServersCount;
        }
    }

    static class Solution2 {
        public int countServers(int[][] grid) {
            int communicableServersCount = 0;
            int[] rowCounts = new int[grid[0].length];
            int[] lastServerInCol = new int[grid.length];
            for (int i = 0; i < lastServerInCol.length; i++) {
                lastServerInCol[i] = -1;
            }

            for (int row = 0; row < grid.length; row++) {
                int serverCountInRow = 0;
                for (int col = 0; col < grid[0].length; col++) {
                    if (grid[row][col] == 1) {
                        serverCountInRow++;
                        rowCounts[col]++;
                        lastServerInCol[row] = col;
                    }
                }

                if (serverCountInRow != 1) {
                    communicableServersCount += serverCountInRow;
                    lastServerInCol[row] = -1;
                }
            }

            for (int row = 0; row < grid.length; row++) {
                if (lastServerInCol[row] != -1 && rowCounts[lastServerInCol[row]] > 1) {
                    communicableServersCount++;
                }
            }

            return communicableServersCount;
        }
    }

    // https://leetcode.com/problems/unique-substrings-with-equal-digit-frequency/description/
    static class Solution3 {
        public int equalDigitFrequency(String s) {
            int n = s.length();
            Set<String> validSubStrings = new HashSet<>();

            for (int start = 0; start < n; start++) {
                int[] digitFrequency = new int[10];

                for (int end = start; end < n; end++) {
                    digitFrequency[s.charAt(end) - '0']++;

                    int commonFrequency = 0;
                    boolean isValid = true;

                    for (int i = 0; i < digitFrequency.length; i++) {
                        if (digitFrequency[i] == 0)
                            continue;

                        if (commonFrequency == 0) {
                            commonFrequency = digitFrequency[i];
                        }
                        if (commonFrequency != digitFrequency[i]) {
                            isValid = false;
                            break;
                        }
                    }

                    if (isValid) {
                        String substring = s.substring(start, end + 1);
                        validSubStrings.add(substring);
                    }
                }
            }

            return validSubStrings.size();
        }
    }

    static class Solution4 {
        public int equalDigitFrequency(String s) {
            int n = s.length();
            int prime = 31;
            long mod = 1000_000_000L;
            Set<Long> validSubstringHashes = new HashSet<>();

            for (int start = 0; start < n; start++) {
                int[] digitFrequency = new int[10];

                int uniqueDigitsCount = 0;
                long substringHash = 0;
                int maxDigitFrequency = 0;

                for (int end = start; end < n; end++) {
                    int currentDigit = s.charAt(end) - '0';

                    if (digitFrequency[currentDigit] == 0) {
                        uniqueDigitsCount++;
                    }

                    digitFrequency[currentDigit]++;
                    maxDigitFrequency = Math.max(maxDigitFrequency, digitFrequency[currentDigit]);

                    substringHash = (prime * substringHash + currentDigit + 1) % mod;

                    if (maxDigitFrequency * uniqueDigitsCount == end - start + 1) {
                        validSubstringHashes.add(substringHash);
                    }
                }
            }
            return validSubstringHashes.size();
        }
    }

    static class Solution5 {
        static class TrieNode {
            TrieNode[] children = new TrieNode[10];
            boolean isVisited;
        }

        public int equalDigitFrequency(String s) {
            TrieNode root = new TrieNode();
            int totalValidSubstrings = 0;

            for (int start = 0; start < s.length(); start++) {
                TrieNode currentNode = root;
                int[] digitFrequency = new int[10];
                int uniqueDigitsCount = 0;
                int maxDigitFrequency = 0;

                for (int end = start; end < s.length(); end++) {
                    int currentDigit = s.charAt(end) - '0';

                    if (digitFrequency[currentDigit]++ == 0) {
                        uniqueDigitsCount++;
                    }
                    maxDigitFrequency = Math.max(maxDigitFrequency, digitFrequency[currentDigit]);

                    if (currentNode.children[currentDigit] == null) {
                        currentNode.children[currentDigit] = new TrieNode();
                    }
                    currentNode = currentNode.children[currentDigit];

                    if (uniqueDigitsCount * maxDigitFrequency == end - start + 1 && !currentNode.isVisited) {
                        totalValidSubstrings++;
                        currentNode.isVisited = true;
                    }
                }
            }
            return totalValidSubstrings;
        }
    }

    // https://leetcode.com/problems/number-of-equal-count-substrings/description/
    static class Solution6 {
        public int equalCountSubstrings(String s, int count) {
            int ans = 0;
            for (int k = 1; k <= 26; k++) {
                int[] freq = new int[26];
                int uniq = 0;
                for (int i = 0; i < s.length(); i++) {
                    if (++freq[s.charAt(i) - 'a'] == count)
                        ++uniq;
                    if (i >= k * count && freq[s.charAt(i - k * count) - 'a']-- == count)
                        --uniq;
                    if (uniq == k)
                        ++ans;
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/number-of-good-ways-to-split-a-string/description/
    static class Solution7 {
        public int numSplits(String s) {
            var ans = 0;
            var n = s.length();
            var left = new HashMap<Character, Integer>();
            var right = new HashMap<Character, Integer>();
            for (int i = 0; i < n; i++) {
                var ch = s.charAt(i);
                right.put(ch, right.getOrDefault(ch, 0) + 1);
            }
            for (int i = 0; i < n; i++) {
                var ch = s.charAt(i);
                left.put(ch, left.getOrDefault(ch, 0) + 1);
                right.put(ch, right.getOrDefault(ch, 0) - 1);
                if (right.get(ch) == 0) {
                    right.remove(ch);
                }
                if (left.size() == right.size()) {
                    ans++;
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/minimum-number-of-flips-to-make-binary-grid-palindromic-ii/description/
    static class Solution8 {
        public int minFlips(int[][] A) {
            int res = 0, one = 0, diff = 0, m = A.length, n = A[0].length;

            // handle quadrants
            for (int i = 0; i < m / 2; i++) {
                for (int j = 0; j < n / 2; j++) {
                    int v = A[i][j] + A[i][n - 1 - j] + A[m - 1 - i][j] + A[m - 1 - i][n - 1 - j];
                    res += Math.min(v, 4 - v);
                }
            }
            // handle middle column
            if (n % 2 > 0) {
                for (int i = 0; i < m / 2; i++) {
                    diff += A[i][n / 2] ^ A[m - 1 - i][n / 2];
                    one += A[i][n / 2] + A[m - 1 - i][n / 2];
                }
            }

            // handle middle row
            if (m % 2 > 0) {
                for (int j = 0; j < n / 2; ++j) {
                    diff += A[m / 2][j] ^ A[m / 2][n - 1 - j];
                    one += A[m / 2][j] + A[m / 2][n - 1 - j];
                }
            }

            // handle center point
            if (n % 2 > 0 && m % 2 > 0) {
                res += A[m / 2][n / 2];
            }

            // divisible by 4
            if (diff == 0 && one % 4 > 0) {
                res += 2;
            }
            return res + diff;
        }
    }

    // https://leetcode.com/problems/sentence-similarity/description/
    static class Solution9 {
        public boolean areSentencesSimilar(String[] sentence1, String[] sentence2, List<List<String>> similarPairs) {
            if (sentence1.length != sentence2.length) {
                return false;
            }

            var sim = new HashMap<String, Set<String>>();
            for (int i = 0; i < similarPairs.size(); i++) {
                var p = similarPairs.get(i);

                sim.putIfAbsent(p.get(0), new HashSet<>());
                sim.putIfAbsent(p.get(1), new HashSet<>());

                sim.get(p.get(0)).add(p.get(1));
                sim.get(p.get(1)).add(p.get(0));
            }

            for (int i = 0; i < sentence1.length; i++) {
                if (!sentence1[i].equals(sentence2[i])) {
                    if (sim.containsKey(sentence1[i]) && sim.get(sentence1[i]).contains(sentence2[i])) {
                        continue;
                    } else {
                        return false;
                    }
                } else {
                    continue;
                }
            }
            return true;
        }
    }

    // https://leetcode.com/problems/coin-change/description/
    static class Solution10 {
        public int coinChange(int[] coins, int amount) {
            if (amount < 1)
                return 0;
            return coinChange(coins, amount, new int[amount]);
        }

        int coinChange(int[] coins, int rem, int[] count) {
            if (rem < 0)
                return -1;
            if (rem == 0)
                return 0;
            if (count[rem - 1] != 0)
                return count[rem - 1];
            int min = Integer.MAX_VALUE;
            for (int coin : coins) {
                int res = coinChange(coins, rem - coin, count);
                if (res >= 0 && res < min) {
                    min = 1 + res;
                }
            }
            return count[rem - 1] = (min == Integer.MAX_VALUE) ? -1 : min;
        }
    }

    static class Solution11 {
        public int coinChange(int[] coins, int amount) {
            int max = amount + 1;
            int[] dp = new int[amount + 1];
            Arrays.fill(dp, max);
            dp[0] = 0;
            for (int i = 1; i <= amount; i++) {
                for (int j = 0; j < coins.length; j++) {
                    if (coins[j] <= i) {
                        dp[i] = Math.min(dp[i], dp[i - coins[j]] + 1);
                    }
                }
            }
            return dp[amount] > amount ? -1 : dp[amount];
        }
    }
}
