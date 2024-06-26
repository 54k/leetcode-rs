package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;

public class Day409 {
    // https://leetcode.com/problems/number-of-dice-rolls-with-target-sum/description/
    static class Solution1 {
        final int MOD = 1_000_000_007;

        int waysToTarget(Integer[][] memo, int diceIndex, int n, int currSum, int target, int k) {
            if (diceIndex == n) {
                return currSum == target ? 1 : 0;
            }

            if (memo[diceIndex][currSum] != null) {
                return memo[diceIndex][currSum];
            }

            int ways = 0;
            for (int i = 1; i <= Math.min(k, target - currSum); i++) {
                ways = (ways + waysToTarget(memo, diceIndex + 1, n, currSum + i, target, k)) % MOD;
            }
            return memo[diceIndex][currSum] = ways;
        }

        public int numRollsToTarget1(int n, int k, int target) {
            Integer[][] memo = new Integer[n + 1][target + 1];
            return waysToTarget(memo, 0, n, 0, target, k);
        }

        public int numRollsToTarget2(int n, int k, int target) {
            int MOD = 1_000_000_007;
            int[][] dp = new int[n + 1][target + 1];
            dp[n][target] = 1;

            for (int i = n - 1; i >= 0; i--) {
                for (int j = 0; j <= target; j++) {
                    int ways = 0;

                    for (int x = 1; x <= Math.min(k, target - j); x++) {
                        ways = (ways + dp[i + 1][x + j]) % MOD;
                    }

                    dp[i][j] = ways;
                }
            }

            return dp[0][0];
        }
    }

    // https://leetcode.com/problems/maximum-length-of-pair-chain/description
    static class Solution2 {
        public int findLongestChain1(int[][] pairs) {
            var rec = new Object() {
                int longestChain(int i, int[][] pairs, int n, int[] memo) {
                    if (memo[i] != 0) {
                        return memo[i];
                    }
                    memo[i] = 1;
                    for (int j = i + 1; j < n; j++) {
                        if (pairs[i][1] < pairs[j][0]) {
                            memo[i] = Math.max(memo[i], 1 + longestChain(j, pairs, n, memo));
                        }
                    }
                    return memo[i];
                }
            };

            int n = pairs.length;
            Arrays.sort(pairs, (a, b) -> a[0] - b[0]);
            int[] memo = new int[n];

            int ans = 0;
            for (int i = 0; i < n; i++) {
                ans = Math.max(ans, rec.longestChain(i, pairs, n, memo));
            }
            return ans;
        }

        public int findLongestChain2(int[][] pairs) {
            int n = pairs.length;
            Arrays.sort(pairs, (a, b) -> a[0] - b[0]);
            int[] dp = new int[n];
            Arrays.fill(dp, 1);

            int ans = 1;
            for (int i = n - 1; i >= 0; i--) {
                for (int j = i + 1; j < n; j++) {
                    if (pairs[i][1] < pairs[j][0]) {
                        dp[i] = Math.max(dp[i], 1 + dp[j]);
                    }
                }
                ans = Math.max(ans, dp[i]);
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/longest-arithmetic-subsequence-of-given-difference/description
    static class Solution3 {
        public int longestSubsequence(int[] arr, int difference) {
            Map<Integer, Integer> dp = new HashMap<>();
            int answer = 1;

            for (int a : arr) {
                int beforeA = dp.getOrDefault(a - difference, 0);
                dp.put(a, beforeA + 1);
                answer = Math.max(answer, dp.get(a));
            }

            return answer;
        }
    }

    // https://leetcode.com/problems/longest-arithmetic-subsequence/description
    static class Solution4 {
        public int longestArithSeqLength(int[] nums) {
            int maxLength = 0;
            HashMap<Integer, Integer>[] dp = new HashMap[nums.length];

            for (int right = 0; right < nums.length; ++right) {
                dp[right] = new HashMap<>();

                for (int left = 0; left < right; ++left) {
                    int diff = nums[left] - nums[right];
                    dp[right].put(diff, dp[left].getOrDefault(diff, 1) + 1);
                    maxLength = Math.max(maxLength, dp[right].get(diff));
                }
            }

            return maxLength;
        }
    }

    // https://leetcode.com/problems/longest-increasing-subsequence/description/
    static class Solution5 {
        public int lengthOfLIS(int[] nums) {
            var sub = new ArrayList<Integer>();
            var search = new Object() {
                int apply(int t) {
                    var lo = -1;
                    var hi = sub.size();

                    while (lo + 1 < hi) {
                        int mid = (lo + hi) / 2;
                        if (sub.get(mid) < t) {
                            lo = mid;
                        } else {
                            hi = mid;
                        }
                    }

                    return hi;
                }
            };

            for (var n : nums) {
                if (sub.size() == 0 || sub.get(sub.size() - 1) < n) {
                    sub.add(n);
                } else {
                    var i = search.apply(n);
                    sub.set(i, n);
                }
            }

            return sub.size();
        }
    }

    // https://leetcode.com/problems/longest-non-decreasing-subarray-from-two-arrays/description/
    static class Solution6 {
        public int maxNonDecreasingLength(int[] nums1, int[] nums2) {
            int[][] dp = new int[nums1.length][2];
            dp[0][0] = 1;
            dp[0][1] = 1;

            int ans = 1;

            for (int i = 1; i < nums1.length; i++) {
                dp[i][0] = 1;
                dp[i][1] = 1;

                if (nums1[i] >= nums1[i - 1]) {
                    dp[i][0] = Math.max(dp[i][0], dp[i - 1][0] + 1);
                }

                if (nums1[i] >= nums2[i - 1]) {
                    dp[i][0] = Math.max(dp[i][0], dp[i - 1][1] + 1);
                }

                if (nums2[i] >= nums2[i - 1]) {
                    dp[i][1] = Math.max(dp[i][1], dp[i - 1][1] + 1);
                }

                if (nums2[i] >= nums1[i - 1]) {
                    dp[i][1] = Math.max(dp[i][1], dp[i - 1][0] + 1);
                }

                ans = Math.max(ans, Math.max(dp[i][0], dp[i][1]));
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/russian-doll-envelopes/description
    static class Solution7 {
        int lengthOfLIS(int[] nums) {
            int[] dp = new int[nums.length];
            int len = 0;
            for (int num : nums) {
                int i = Arrays.binarySearch(dp, 0, len, num);
                if (i < 0) {
                    i = -(i + 1);
                }
                dp[i] = num;
                if (i == len) {
                    len++;
                }
            }
            return len;
        }

        public int maxEnvelopes(int[][] envelopes) {
            Arrays.sort(envelopes, new Comparator<int[]>() {
                public int compare(int[] arr1, int[] arr2) {
                    if (arr1[0] == arr2[0]) {
                        return arr2[1] - arr1[1];
                    } else {
                        return arr1[0] - arr2[0];
                    }
                }
            });

            int[] secondDim = new int[envelopes.length];
            for (int i = 0; i < envelopes.length; i++) {
                secondDim[i] = envelopes[i][1];
            }
            return lengthOfLIS(secondDim);
        }
    }

    // https://leetcode.com/problems/find-the-longest-valid-obstacle-course-at-each-position/description
    static class Solution8 {
        int bisectRight(int[] A, int target, int right) {
            if (right == 0) {
                return 0;
            }

            int left = 0;
            while (left < right) {
                int mid = left + (right - left) / 2;
                if (A[mid] <= target) {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            return left;
        }

        public int[] longestObstacleCourseAtEachPosition(int[] obstacles) {
            int n = obstacles.length;
            int lisLength = 0;

            int[] answer = new int[n];
            int[] lis = new int[n];

            for (int i = 0; i < n; i++) {
                int height = obstacles[i];

                int idx = bisectRight(lis, height, lisLength);
                if (idx == lisLength) {
                    lisLength++;
                }
                lis[idx] = height;
                answer[i] = idx + 1;
            }

            return answer;
        }
    }

    // https://leetcode.com/problems/intersection-of-two-arrays-ii/description/
    static class Solution9 {
        public int[] intersect(int[] nums1, int[] nums2) {
            var freq = new int[1001];
            for (var n : nums1) {
                freq[n]++;
            }

            List<Integer> ans = new ArrayList<>();
            for (var n : nums2) {
                if (freq[n]-- > 0) {
                    ans.add(n);
                }
            }
            var arr = new int[ans.size()];
            for (int i = 0; i < ans.size(); i++) {
                arr[i] = ans.get(i);
            }
            return arr;
        }
    }

    // https://leetcode.com/problems/find-the-difference-of-two-arrays/
    static class Solution10 {
        List<Integer> getElemsOnlyInFirstList(int[] nums1, int[] nums2) {
            Set<Integer> onlyInNums1 = new HashSet<>();
            Set<Integer> existsInNums2 = new HashSet<>();
            for (int num : nums2) {
                existsInNums2.add(num);
            }

            for (int num : nums1) {
                if (!existsInNums2.contains(num)) {
                    onlyInNums1.add(num);
                }
            }

            return new ArrayList<>(onlyInNums1);
        }

        public List<List<Integer>> findDifference(int[] nums1, int[] nums2) {
            return Arrays.asList(
                    getElemsOnlyInFirstList(nums1, nums2),
                    getElemsOnlyInFirstList(nums2, nums1));
        }
    }
}
