package leetcode_grind;

import java.util.Arrays;

public class Day620 {

    // https://leetcode.com/problems/count-number-of-teams/description/?envType=daily-question&envId=2024-07-29
    static class Solution1 {
        public int numTeams(int[] rating) {
            int n = rating.length;
            int teams = 0;

            Integer[][] increasingCache = new Integer[n][4];
            Integer[][] decreasingCache = new Integer[n][4];

            for (int startIndex = 0; startIndex < n; startIndex++) {
                teams += countIncreasingTeams(rating, startIndex, 1, increasingCache) +
                        countDecreasingTeams(rating, startIndex, 1, decreasingCache);
            }

            return teams;
        }

        int countIncreasingTeams(int[] rating, int currentIndex, int teamSize, Integer[][] increasingCache) {
            int n = rating.length;

            if (currentIndex == n)
                return 0;

            if (teamSize == 3)
                return 1;

            if (increasingCache[currentIndex][teamSize] != null) {
                return increasingCache[currentIndex][teamSize];
            }

            int validTeams = 0;

            for (int nextIndex = currentIndex + 1; nextIndex < n; nextIndex++) {
                if (rating[nextIndex] > rating[currentIndex]) {
                    validTeams += countIncreasingTeams(rating, nextIndex, teamSize + 1, increasingCache);
                }
            }

            return increasingCache[currentIndex][teamSize] = validTeams;
        }

        int countDecreasingTeams(int[] rating, int currentIndex, int teamSize, Integer[][] decreasingCache) {
            int n = rating.length;

            if (currentIndex == n)
                return 0;

            if (teamSize == 3)
                return 1;

            if (decreasingCache[currentIndex][teamSize] != null) {
                return decreasingCache[currentIndex][teamSize];
            }

            int validTeams = 0;

            for (int nextIndex = currentIndex + 1; nextIndex < n; nextIndex++) {
                if (rating[nextIndex] < rating[currentIndex]) {
                    validTeams += countDecreasingTeams(rating, nextIndex, teamSize + 1, decreasingCache);
                }
            }

            return decreasingCache[currentIndex][teamSize] = validTeams;
        }
    }

    static class Solution2 {
        public int numTeams(int[] rating) {
            int n = rating.length;
            int teams = 0;

            for (int mid = 0; mid < n; mid++) {
                int leftSmaller = 0;
                int rightLarger = 0;

                for (int left = mid - 1; left >= 0; left--) {
                    if (rating[left] < rating[mid]) {
                        leftSmaller++;
                    }
                }

                for (int right = mid + 1; right < n; right++) {
                    if (rating[right] > rating[mid]) {
                        rightLarger++;
                    }
                }

                teams += leftSmaller * rightLarger;

                int leftLarger = mid - leftSmaller;
                int rightSmaller = n - mid - 1 - rightLarger;

                teams += leftLarger * rightSmaller;
            }

            return teams;
        }
    }

    // https://leetcode.com/problems/toss-strange-coins/description/?envType=weekly-question&envId=2024-07-29
    static class Solution3 {
        double findProbability(int index, int n, double[][] memo, double[] prob, int target) {
            if (target < 0)
                return 0;
            if (index == n) {
                return target == 0 ? 1 : 0;
            }
            if (memo[index][target] != -1) {
                return memo[index][target];
            }
            memo[index][target] = findProbability(index + 1, n, memo, prob, target - 1) * prob[index]
                    + findProbability(index + 1, n, memo, prob, target) * (1.0 - prob[index]);
            return memo[index][target];
        }

        public double probabilityOfHeads(double[] prob, int target) {
            int n = prob.length;
            double[][] memo = new double[n][target + 1];
            for (double[] row : memo) {
                Arrays.fill(row, -1);
            }
            return findProbability(0, n, memo, prob, target);
        }
    }

    static class Solution4 {
        public double probabilityOfHeads(double[] prob, int target) {
            int n = prob.length;
            double[] dp = new double[target + 1];
            dp[0] = 1;

            for (int i = 1; i <= n; i++) {
                for (int j = target; j >= 1; j--) {
                    dp[j] = dp[j - 1] * prob[i - 1] + dp[j] * (1 - prob[i - 1]);
                }
                dp[0] = dp[0] * (1 - prob[i - 1]);
            }

            return dp[target];
        }
    }

    // https://leetcode.com/problems/expressive-words/description/
    static class Solution5 {
        public int expressiveWords(String s, String[] words) {
            if (s == null || words == null) {
                return 0;
            }
            int count = 0;
            for (String word : words) {
                if (stretchy(s, word)) {
                    count++;
                }
            }
            return count;
        }

        boolean stretchy(String s, String word) {
            if (word == null) {
                return false;
            }

            int i = 0;
            int j = 0;

            while (i < s.length() && j < word.length()) {
                if (s.charAt(i) != word.charAt(j)) {
                    return false;
                }

                int len1 = getRepeatedLength(s, i);
                int len2 = getRepeatedLength(word, j);

                if (len1 < 3 && len1 != len2 || len1 >= 3 && len1 < len2) {
                    return false;
                }

                i += len1;
                j += len2;
            }

            return i == s.length() && j == word.length();
        }

        int getRepeatedLength(String str, int i) {
            int j = i;
            while (j < str.length() && str.charAt(j) == str.charAt(i)) {
                j++;
            }
            return j - i;
        }
    }

    static class Solution6 {
        public String findReplaceString(String s, int[] indices, String[] sources, String[] targets) {
            var ind = new Integer[indices.length];
            for (int i = 0; i < indices.length; i++) {
                ind[i] = i;
            }

            Arrays.sort(ind, (a, b) -> {
                return indices[(int) a] - indices[(int) b];
            });

            var ans = new StringBuilder();
            var i = 0;
            var c = 0;
            while (i < s.length() && c < indices.length) {
                var j = ind[c];
                if (i == indices[j]) {
                    if (s.substring(indices[j]).startsWith(sources[j])) {
                        ans.append(targets[j]);
                        i += sources[j].length();
                    }
                    c++;
                } else {
                    ans.append(s.charAt(i));
                    i += 1;
                }
            }

            while (i < s.length()) {
                ans.append(s.charAt(i++));
            }

            return ans.toString();
        }
    }

    // https://leetcode.com/problems/maximize-distance-to-closest-person/description/
    static class Solution7 {
        public int maxDistToClosest(int[] seats) {
            int N = seats.length;
            int[] left = new int[N];
            int[] right = new int[N];
            Arrays.fill(left, N);
            Arrays.fill(right, N);

            for (int i = 0; i < N; i++) {
                if (seats[i] == 1) {
                    left[i] = 0;
                } else if (i > 0) {
                    left[i] = left[i - 1] + 1;
                }
            }

            for (int i = N - 1; i >= 0; i--) {
                if (seats[i] == 1) {
                    right[i] = 0;
                } else if (i < N - 1) {
                    right[i] = right[i + 1] + 1;
                }
            }

            int ans = 0;
            for (int i = 0; i < N; i++) {
                if (seats[i] == 0) {
                    ans = Math.max(ans, Math.min(left[i], right[i]));
                }
            }
            return ans;
        }
    }

    static class Solution8 {
        public int maxDistToClosest(int[] seats) {
            int N = seats.length;
            int prev = -1, future = 0;
            int ans = 0;

            for (int i = 0; i < N; i++) {
                if (seats[i] == 1) {
                    prev = i;
                } else {
                    while (future < N && seats[future] == 0 || future < i) {
                        future++;
                    }

                    int left = prev == -1 ? N : i - prev;
                    int right = future == N ? N : future - i;
                    ans = Math.max(ans, Math.min(left, right));
                }
            }

            return ans;
        }
    }

    static class Solution9 {
        public int maxDistToClosest(int[] seats) {
            int N = seats.length;
            int K = 0;
            int ans = 0;

            for (int i = 0; i < N; i++) {
                if (seats[i] == 1) {
                    K = 0;
                } else {
                    K++;
                    ans = Math.max(ans, (K + 1) / 2);
                }
            }

            for (int i = 0; i < N; i++) {
                if (seats[i] == 1) {
                    ans = Math.max(ans, i);
                    break;
                }
            }

            for (int i = N - 1; i >= 0; --i) {
                if (seats[i] == 1) {
                    ans = Math.max(ans, N - 1 - i);
                    break;
                }
            }

            return ans;
        }
    }
}