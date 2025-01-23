package leetcode_grind;

import java.util.HashSet;
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
}
