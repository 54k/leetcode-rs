package top_interview_150;

import java.util.Arrays;
import java.util.Map;
import java.util.function.Consumer;

// https://leetcode.com/studyplan/top-interview-150/
public class ArrayAndString {

    // https://leetcode.com/problems/merge-sorted-array/description
    static class Solution1 {
        public void merge3PointersBackward(int[] nums1, int m, int[] nums2, int n) {
            // 3 pointers forward
            int p1 = m - 1;
            int p2 = n - 1;
            for (int p = m + n - 1; p >= 0; p--) {
                if (p2 < 0) {
                    break;
                }
                if (p1 >= 0 && nums1[p1] > nums2[p2]) {
                    nums1[p] = nums1[p1--];
                } else {
                    nums1[p] = nums2[p2--];
                }
            }
        }
    }

    // https://leetcode.com/problems/remove-element/description
    static class Solution2 {
        public int removeElement1(int[] nums, int val) {
            int left = 0;
            for (int right = 0; right < nums.length; right++) {
                if (nums[right] != val) {
                    nums[left++] = nums[right];
                }
            }
            return left++;
        }

        public int removeElement2(int[] nums, int val) {
            int i = 0;
            int n = nums.length;
            while (i < n) {
                if (nums[i] == val) {
                    nums[i] = nums[n - 1];
                    n--;
                } else {
                    i++;
                }
            }
            return i;
        }
    }

    // https://leetcode.com/problems/remove-duplicates-from-sorted-array
    static class Solution3 {
        public int removeDuplicates1(int[] nums) {
            int left = 0;
            for (int right = 0; right < nums.length; right++) {
                if (nums[left] != nums[right]) {
                    nums[++left] = nums[right];
                }
            }
            return ++left;
        }

        public int removeDuplicates2(int[] nums) {
            int insertIndex = 1;
            for (int i = 1; i < nums.length; i++) {
                if (nums[i - 1] != nums[i]) {
                    nums[insertIndex++] = nums[i];
                }
            }
            return insertIndex;
        }
    }

    // https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/description
    static class Solution4 {
        public int removeDuplicates1(int[] nums) {
            Consumer<Integer> removeFromArray = (index) -> {
                for (int i = index; i < nums.length - 1; i++) {
                    nums[i] = nums[i + 1];
                }
            };

            int length = nums.length;
            int i = 1;
            int count = 1;

            while (i < length) {
                if (nums[i] == nums[i - 1]) {
                    count++;
                    if (count > 2) {
                        removeFromArray.accept(i);
                        i--;
                        length--;
                    }
                }
                i++;
            }
            return length;
        }

        public int removeDuplicates2(int[] nums) {
            int j = 1, count = 1;
            for (int i = 1; i < nums.length; i++) {
                if (nums[i - 1] == nums[i]) {
                    count++;
                } else {
                    count = 1;
                }
                if (count <= 2) {
                    nums[j] = nums[i];
                    j++;
                }
            }
            return j;
        }

        public int removeDuplicates3(int[] nums) {
            var k = 2;
            var left = 0;
            var dups = 1;

            for (var right = 1; right < nums.length; right++) {
                if (nums[left] != nums[right]) {
                    nums[++left] = nums[right];
                    dups = 1;
                } else if (dups < k) {
                    nums[++left] = nums[right];
                    dups++;
                }
            }

            return ++left;
        }
    }

    // https://leetcode.com/problems/majority-element
    static class Solution5 {
        public int majorityElement3(int[] nums) {
            Arrays.sort(nums);
            return nums[nums.length / 2];
        }

        public int majorityElement4(int[] nums) {
            int n = nums.length;
            int majority_element = 0;

            for (int i = 0; i < 32; i++) {
                int bit = 1 << i;

                int bit_count = 0;
                for (int num : nums) {
                    if ((num & bit) != 0) {
                        bit_count++;
                    }
                }

                if (bit_count > n / 2) {
                    majority_element |= bit;
                }
            }

            return majority_element;
        }

        public int majorityElement6(int[] nums) {
            var countFreq = new Object() {
                int apply(int target) {
                    var freq = 0;
                    for (var num : nums) {
                        if (num == target) {
                            freq++;
                        }
                    }
                    return freq;
                }
            };

            var majorityElementRec = new Object() {
                int apply(int lo, int hi) {
                    if (lo == hi) {
                        return nums[lo];
                    }

                    int mid = (hi - lo) / 2 + lo;
                    int left = apply(lo, mid);
                    int right = apply(mid + 1, hi);

                    if (left == right) {
                        return left;
                    }

                    if (countFreq.apply(left) > countFreq.apply(right)) {
                        return left;
                    } else {
                        return right;
                    }
                }
            };

            return majorityElementRec.apply(0, nums.length - 1);
        }

        public int majorityElement7(int[] nums) {
            int count = 0;
            Integer candidate = null;
            for (var num : nums) {
                if (count == 0) {
                    candidate = num;
                }
                count += candidate == num ? 1 : -1;
            }
            return candidate;
        }
    }

    // https://leetcode.com/problems/rotate-array/description
    static class Solution6 {
        public void rotate2(int[] nums, int k) {
            var a = new int[nums.length];
            for (int i = 0; i < nums.length; i++) {
                a[(i + k) % nums.length] = nums[i];
            }
            for (int i = 0; i < a.length; i++) {
                nums[i] = a[i];
            }
        }

        public void rotate3(int[] nums, int k) {
            k = k % nums.length;
            int count = 0;
            for (int start = 0; count < nums.length; start++) {
                int current = start;
                int prev = nums[start];

                do {
                    int next = (current + k) % nums.length;
                    int temp = nums[next];
                    nums[next] = prev;
                    prev = temp;
                    current = next;
                    count++;
                } while (start != current);
            }
        }

        public void rotate4(int[] nums, int k) {
            var reverse = new Object() {
                void apply(int from, int to) {
                    for (int i = from; i < (from + to) / 2; i++) {
                        var temp = nums[i];
                        nums[i] = nums[(from + to - 1) - i];
                        nums[(from + to - 1) - i] = temp;
                    }
                }
            };

            var n = nums.length;
            reverse.apply(0, n);
            reverse.apply(0, k % n);
            reverse.apply((k) % n, n);
        }
    }

    // https://leetcode.com/problems/best-time-to-buy-and-sell-stock
    static class Solution7 {
        public int maxProfit1(int[] prices) {
            var minPrice = prices[0];
            var maxProfit = 0;
            for (int i = 0; i < prices.length; i++) {
                if (prices[i] < minPrice) {
                    minPrice = prices[i];
                } else if (prices[i] - minPrice > maxProfit) {
                    maxProfit = prices[i] - minPrice;
                }
            }
            return maxProfit;
        }

        public int maxProfit2(int[] prices) {
            var ans = 0;
            var prev = prices[0];
            for (var curr : prices) {
                ans = Math.max(Math.max(ans, curr - prev), 0);
                if (curr <= prev) {
                    prev = curr;
                }
            }
            return ans;
        }

        public int maxProfit3(int[] prices) {
            int largestDifference = 0;
            int minSoFar = Integer.MAX_VALUE;
            for (int i = 0; i < prices.length; i++) {
                if (prices[i] < minSoFar) {
                    minSoFar = prices[i];
                } else {
                    largestDifference = Math.max(largestDifference, prices[i] - minSoFar);
                }
            }
            return largestDifference;
        }
    }

    // https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/description
    static class Solution8 {
        public int maxProfitPeaksAndValleys(int[] prices) {
            var total = 0;
            var peak = Integer.MAX_VALUE;
            var valley = Integer.MAX_VALUE;

            for (int i = 0; i < prices.length; i++) {
                if (prices[i] < peak) {
                    total += peak - valley;
                    valley = prices[i];
                    peak = valley;
                } else {
                    peak = prices[i];
                }
            }

            total += peak - valley;
            return total;
        }

        public int maxProfitImprovedPeaksAndValleys(int[] prices) {
            var total = 0;
            for (int i = 1; i < prices.length; i++) {
                if (prices[i] > prices[i - 1]) {
                    total += prices[i] - prices[i - 1];
                }
            }
            return total;
        }

        public int maxProfitBruteForce(int[] prices) {
            var dp = new int[prices.length];
            var maxProfit = 0;
            for (int i = 0; i < dp.length; i++) {
                var maxSoFar = 0;
                for (int j = 0; j < i; j++) {
                    maxSoFar = Math.max(maxSoFar, dp[j]);
                    dp[i] = Math.max(dp[i], maxSoFar + (prices[i] - prices[j]));
                    maxProfit = Math.max(maxProfit, dp[i]);
                }
            }
            return maxProfit;
        }
    }

    // https://leetcode.com/problems/jump-game/description
    static class Solution9 {
        public boolean canJumpDFS(int[] nums) {
            var visited = new boolean[nums.length];

            var dfs = new Object() {
                boolean appy(int i) {
                    if (i == nums.length - 1) {
                        return true;
                    }

                    if (visited[i]) {
                        return false;
                    }
                    visited[i] = true;

                    for (var next = 1; next <= nums[i]; next++) {
                        if (appy((i + next) % nums.length)) {
                            return true;
                        }
                    }

                    return false;
                }
            };

            return dfs.appy(0);
        }

        public boolean canJumpBackTrack(int[] nums) {
            var backtrack = new Object() {
                boolean apply(int position) {
                    if (position == nums.length - 1) {
                        return true;
                    }

                    var furthestJump = Math.min(position + nums[position], nums.length - 1);
                    for (int nextPosition = position + 1; nextPosition <= furthestJump; nextPosition++) {
                        if (apply(nextPosition)) {
                            return true;
                        }
                    }

                    return false;
                }
            };
            return backtrack.apply(0);
        }

        public boolean canJumpBackTrackMemo(int[] nums) {
            var memo = new Boolean[nums.length];
            var backtrack = new Object() {
                boolean apply(int position) {
                    if (position == nums.length - 1) {
                        return true;
                    }

                    if (memo[position] != null) {
                        return memo[position];
                    }

                    var furthestJump = Math.min(position + nums[position], nums.length - 1);
                    for (int nextPosition = position + 1; nextPosition <= furthestJump; nextPosition++) {
                        if (apply(nextPosition)) {
                            memo[position] = true;
                            return true;
                        }
                    }
                    memo[position] = false;
                    return false;
                }
            };
            return backtrack.apply(0);
        }

        public boolean canJumpDPBottomUpBackward(int[] nums) {
            var dp = new boolean[nums.length];
            dp[nums.length - 1] = true;
            for (int i = dp.length - 2; i >= 0; i--) {
                var furthestJump = Math.min(i + nums[i], dp.length - 1);
                for (int j = i + 1; j <= furthestJump; j++) {
                    dp[i] |= dp[j];
                    if (dp[j]) {
                        break;
                    }
                }
            }
            return dp[0];
        }

        public boolean canJumpDPBottomUpForward(int[] nums) {
            var dp = new boolean[nums.length];
            dp[0] = true;
            for (int k = 0; k < dp.length; k++) {
                for (int k2 = k + 1; k2 <= Math.min(dp.length - 1, k + nums[k]); k2++) {
                    dp[k2] |= dp[k];
                }
            }
            return dp[nums.length - 1];
        }

        public boolean canJumpGreedy(int[] nums) {
            var lastGood = nums.length - 1;
            for (int i = nums.length - 2; i >= 0; i--) {
                if (i + nums[i] >= lastGood) {
                    lastGood = i;
                }
            }
            return lastGood == 0;
        }
    }

    // https://leetcode.com/problems/jump-game-ii/description
    static class Solution10 {
        public int jump(int[] nums) {
            var end = 0;
            var far = 0;
            var answer = 0;

            for (int i = 0; i < nums.length - 1; i++) {
                far = Math.max(far, i + nums[i]);
                if (i == end) {
                    answer++;
                    end = far;
                }
            }

            return answer;
        }
    }

    // https://leetcode.com/problems/h-index/description
    static class Solution11 {
        public int hIndexSorting(int[] citations) {
            Arrays.sort(citations);
            int i = 0;
            while (i < citations.length && citations[citations.length - 1 - i] > i) {
                i++;
            }
            return i;
        }

        public int hIndexCountingSort(int[] citations) {
            int n = citations.length;
            int[] papers = new int[n + 1];
            for (int c : citations) {
                papers[Math.min(n, c)]++;
            }

            int k = n;
            for (int s = papers[n]; k > s; s += papers[k]) {
                k--;
            }
            return k;
        }
    }

    // https://leetcode.com/problems/roman-to-integer
    static class Solution {
        public int romanToInt(String s) {
            var map = Map.of(
                    'I', 1,
                    'V', 5,
                    'X', 10,
                    'L', 50,
                    'C', 100,
                    'D', 500,
                    'M', 1000);

            var res = 0;
            for (int i = 0; i < s.length(); i++) {
                var ch = s.charAt(i);
                if (i < s.length() - 1 && map.get(ch) < map.get(s.charAt(i + 1))) {
                    res += map.get(s.charAt(i + 1)) - map.get(ch);
                    i++;
                } else {
                    res += map.get(ch);
                }
            }
            return res;
        }
    }
}
