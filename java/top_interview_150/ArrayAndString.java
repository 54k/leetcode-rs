package top_interview_150;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Random;
import java.util.Stack;
import java.util.function.Consumer;
import java.util.function.Function;

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

    // https://leetcode.com/problems/insert-delete-getrandom-o1/description
    static class Solution12 {
        class RandomizedSet {
            Map<Integer, Integer> valToIdx = new HashMap<Integer, Integer>();
            List<Integer> values = new ArrayList<Integer>();
            Random rand = new Random();

            public RandomizedSet() {
            }

            public boolean insert(int val) {
                if (!valToIdx.containsKey(val)) {
                    return false;
                }
                values.add(val);
                valToIdx.put(val, values.size() - 1);
                return true;
            }

            public boolean remove1(int val) {
                if (!valToIdx.containsKey(val)) {
                    return false;
                }
                var idx = valToIdx.remove(val);
                values.set(idx, values.get(values.size() - 1));
                values.remove(values.size() - 1);
                if (values.size() > idx) {
                    valToIdx.put(values.get(idx), idx);
                }
                return true;
            }

            public boolean remove(int val) {
                if (!valToIdx.containsKey(val)) {
                    return false;
                }
                var idx = valToIdx.get(val);
                var last = values.get(values.size() - 1);
                values.set(idx, last);
                valToIdx.put(last, idx);
                valToIdx.remove(val);
                values.remove(values.size() - 1);
                return true;
            }

            public int getRandom() {
                return values.get(Math.abs(rand.nextInt() - 1) % values.size());
            }
        }
    }

    // https://leetcode.com/problems/product-of-array-except-self
    static class Solution13 {
        public int[] productExceptSelf1(int[] nums) {
            var L = new int[nums.length];
            L[0] = 1;
            var R = new int[nums.length];
            R[nums.length - 1] = 1;

            for (int i = 1; i < nums.length; i++) {
                L[i] = nums[i - 1] * L[i - 1];
            }
            for (int i = nums.length - 2; i >= 0; i--) {
                R[i] = nums[i + 1] * R[i + 1];
            }
            var ans = new int[nums.length];
            for (int i = 0; i < nums.length; i++) {
                ans[i] = L[i] * R[i];
            }
            return ans;
        }

        public int[] productExceptSelf2(int[] nums) {
            var L = new int[nums.length];
            L[0] = 1;
            for (int i = 1; i < nums.length; i++) {
                L[i] = L[i - 1] * nums[i - 1];
            }

            var R = 1;
            for (int i = nums.length - 1; i >= 0; i--) {
                L[i] *= R;
                R *= nums[i];
            }

            return L;
        }
    }

    // https://leetcode.com/problems/gas-station
    static class Solution14 {
        public int canCompleteCircuit1(int[] gas, int[] cost) {
            var ans = 0;
            var cur = 0;

            var totalGas = 0;
            var totalCost = 0;

            for (int i = 0; i < gas.length; i++) {
                totalCost += cost[i];
                totalGas += gas[i];

                cur += gas[i] - cost[i];
                if (cur < 0) {
                    ans = i + 1;
                    cur = 0;
                }
            }

            if (totalGas - totalCost < 0) {
                return -1;
            }
            return ans;
        }

        public int canCompleteCircuit2(int[] gas, int[] cost) {
            var minSum = Integer.MAX_VALUE;
            var curSum = 0;
            var ans = 0;
            for (int i = 0; i < gas.length; i++) {
                if (curSum < minSum) {
                    minSum = curSum;
                    ans = i;
                }
                curSum += gas[i] - cost[i];
            }
            if (curSum < 0) {
                return -1;
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/candy/description
    static class Solution15 {
        public int candy1(int[] ratings) {
            int[] candies = new int[ratings.length];
            Arrays.fill(candies, 1);
            boolean hasChanged = true;

            while (hasChanged) {
                hasChanged = false;
                for (int i = 0; i < ratings.length; i++) {
                    if (i != ratings.length - 1 && ratings[i] > ratings[i + 1] && candies[i] <= candies[i + 1]) {
                        candies[i] = candies[i + 1] + 1;
                        hasChanged = true;
                    }
                    if (i != 0 && ratings[i] > ratings[i - 1] && candies[i] <= candies[i - 1]) {
                        candies[i] = candies[i - 1] + 1;
                        hasChanged = true;
                    }
                }
            }

            var sum = 0;
            for (var c : candies) {
                sum += c;
            }
            return sum;
        }

        public int candy2(int[] ratings) {
            var leftToRightCandies = new int[ratings.length];
            var rightToLeftCandies = new int[ratings.length];

            for (int i = 0; i < rightToLeftCandies.length; i++) {
                leftToRightCandies[i]++;
                rightToLeftCandies[i]++;
            }

            for (int i = 1; i < ratings.length; i++) {
                if (ratings[i] > ratings[i - 1]) {
                    leftToRightCandies[i] = leftToRightCandies[i - 1] + 1;
                }
            }

            for (int i = ratings.length - 2; i >= 0; i--) {
                if (ratings[i] > ratings[i + 1]) {
                    rightToLeftCandies[i] = rightToLeftCandies[i + 1] + 1;
                }
            }

            var sum = 0;
            for (int i = 0; i < ratings.length; i++) {
                sum += Math.max(leftToRightCandies[i], rightToLeftCandies[i]);
            }
            return sum;
        }

        public int candy3(int[] ratings) {
            var candies = new int[ratings.length];
            Arrays.fill(candies, 1);
            for (int i = 1; i < candies.length; i++) {
                if (ratings[i] > ratings[i - 1]) {
                    candies[i] = candies[i - 1] + 1;
                }
            }
            var sum = candies[ratings.length - 1];
            for (int i = ratings.length - 2; i >= 0; i--) {
                if (ratings[i] > ratings[i + 1]) {
                    candies[i] = Math.max(candies[i + 1] + 1, candies[i]);
                }
                sum += candies[i];
            }
            return sum;
        }

        public int candy4(int[] ratings) {
            Function<Integer, Integer> count = (n) -> (n * (n + 1)) / 2;

            if (ratings.length <= 1) {
                return ratings.length;
            }

            int candies = 0;
            int up = 0;
            int down = 0;
            int oldSlope = 0;

            for (int i = 1; i < ratings.length; i++) {
                int newSlope = ratings[i] > ratings[i - 1] ? 1 : ratings[i] < ratings[i - 1] ? -1 : 0;

                if ((oldSlope > 0 && newSlope == 0) || (oldSlope < 0 && newSlope >= 0)) {
                    candies += count.apply(up) + count.apply(down) + Math.max(up, down);
                    up = 0;
                    down = 0;
                }

                if (newSlope > 0) {
                    up++;
                } else if (newSlope < 0) {
                    down++;
                } else {
                    candies++;
                }

                oldSlope = newSlope;
            }

            candies += count.apply(up) + count.apply(down) + Math.max(up, down) + 1;
            return candies;
        }

        // https://leetcode.com/problems/candy/solutions/135698/Simple-solution-with-one-pass-using-O(1)-space/
        public int candy5(int[] ratings) {
            if (ratings.length == 0)
                return 0;
            int ret = 1;
            int up = 0, down = 0, peak = 0;
            for (int i = 1; i < ratings.length; i++) {
                if (ratings[i - 1] < ratings[i]) {
                    peak = ++up;
                    down = 0;
                    ret += 1 + up;
                } else if (ratings[i - 1] == ratings[i]) {
                    peak = up = down = 0;
                    ret += 1;
                } else {
                    up = 0;
                    down++;
                    ret += 1 + down + (peak >= down ? -1 : 0);
                }
            }
            return ret;
        }
    }

    // https://leetcode.com/problems/trapping-rain-water/description/
    static class Solution16 {
        public int trap1(int[] height) {
            var ans = 0;
            for (int i = 0; i < height.length; i++) {
                int leftMax = 0;
                int rightMax = 0;

                for (int j = i; j >= 0; j--) {
                    leftMax = Math.max(leftMax, height[j]);
                }

                for (int k = i; k < height.length; k++) {
                    rightMax = Math.max(rightMax, height[k]);
                }

                ans += Math.min(leftMax, rightMax) - height[i];
            }
            return ans;
        }

        public int trap2(int[] height) {
            if (height.length == 0) {
                return 0;
            }

            var len = height.length;

            var leftMax = new int[height.length];
            leftMax[0] = height[0];

            var rightMax = new int[height.length];
            rightMax[height.length - 1] = height[height.length - 1];

            for (int i = 1; i < len; i++) {
                leftMax[i] = Math.max(height[i], leftMax[i - 1]);
            }
            for (int i = len - 2; i >= 0; i--) {
                rightMax[i] = Math.max(height[i], rightMax[i + 1]);
            }

            var ans = 0;
            for (int i = 0; i < len; i++) {
                ans += Math.min(leftMax[i], rightMax[i]) - height[i];
            }
            return ans;
        }

        public int trap3(int[] height) {
            var ans = 0;

            var stack = new Stack<Integer>();
            var current = 0;
            while (current < height.length) {
                while (!stack.isEmpty() && height[current] > height[stack.peek()]) {
                    var top = stack.pop();
                    if (stack.isEmpty()) {
                        break;
                    }

                    var distance = current - stack.peek() - 1;
                    var boundedHeight = Math.min(height[current], height[stack.peek()]) - height[top];
                    ans += distance * boundedHeight;
                }
                stack.push(current++);
            }

            return ans;
        }

        public int trap4(int[] height) {
            var leftMax = 0;
            var rightMax = 0;
            var ans = 0;

            var left = 0;
            var right = height.length - 1;
            while (left < right) {
                if (height[left] < height[right]) {
                    if (height[left] >= leftMax) {
                        leftMax = height[left];
                    } else {
                        ans += leftMax - height[left];
                    }
                    left++;
                } else {
                    if (height[right] >= rightMax) {
                        rightMax = height[right];
                    } else {
                        ans += rightMax - height[right];
                    }
                    right--;
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/roman-to-integer
    static class Solution17 {
        public int romanToIntForward(String s) {
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

        public int romanToIntBackwards(String s) {
            Map<String, Integer> values = new HashMap<>();
            values.put("M", 1000);
            values.put("D", 500);
            values.put("C", 100);
            values.put("L", 50);
            values.put("X", 10);
            values.put("V", 5);
            values.put("I", 1);

            String lastSymbol = s.substring(s.length() - 1);
            int lastValue = values.get(lastSymbol);
            int total = lastValue;

            for (int i = s.length() - 2; i >= 0; i--) {
                String currentSymbol = s.substring(i, i + 1);
                int currentValue = values.get(currentSymbol);

                if (currentValue < lastValue) {
                    total -= currentValue;
                } else {
                    total += currentValue;
                }
                lastValue = currentValue;
            }

            return total;
        }
    }

    // https://leetcode.com/problems/integer-to-roman/description
    static class Solution18 {
        public String intToRomanGreedy(int num) {
            var symbols = new String[] { "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I" };
            var values = new int[] { 1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1 };

            var ans = new StringBuilder();
            for (int i = 0; i < values.length && num > 0; i++) {
                while (num >= values[i]) {
                    num -= values[i];
                    ans.append(symbols[i]);
                }
            }
            return ans.toString();
        }

        public String intToRomanHardcodedDigits(int num) {
            var thousands = new String[] { "", "M", "MM", "MMM" };
            var hundreds = new String[] { "", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM" };
            var tens = new String[] { "", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC" };
            var ones = new String[] { "", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX" };

            return thousands[num / 1000] + hundreds[num % 1000 / 100] + tens[num % 100 / 10] + ones[num % 10];
        }
    }

    // https://leetcode.com/problems/length-of-last-word/description
    static class Solution19 {
        public int lengthOfLastWord1(String s) {
            var end = s.length();
            for (int i = s.length() - 1; i >= 0; i--) {
                if (s.charAt(i) != ' ' && end == s.length()) {
                    end = i;
                } else if (s.charAt(i) == ' ' && end != s.length()) {
                    return end - i;
                }
            }
            return end + 1;
        }

        public int lengthOfLastWord2(String s) {
            var i = s.length() - 1;
            var length = 0;
            while (i >= 0) {
                if (s.charAt(i) != ' ') {
                    length++;
                } else if (length > 0) {
                    return length;
                }
                i--;
            }
            return length;
        }
    }

    // https://leetcode.com/problems/longest-common-prefix/description
    static class Solution20 {
        public String longestCommonPrefix1(String[] strs) {
            if (strs.length == 0)
                return "";
            String prefix = strs[0];
            for (int i = 1; i < strs.length; i++) {
                while (strs[i].indexOf(prefix) != 0) {
                    prefix = prefix.substring(0, prefix.length() - 1);
                }
            }
            return prefix;
        }

        public String longestCommonPrefix2(String[] strs) {
            if (strs.length == 0)
                return "";
            String prefix = strs[0];
            for (int i = 1; i < strs.length; i++) {
                while (strs[i].indexOf(prefix) != 0) {
                    prefix = prefix.substring(0, prefix.length() - 1);
                }
            }
            return prefix;
        }

        public String longestCommonPrefix3(String[] strs) {
            if (strs == null || strs.length == 0)
                return "";
            var lcp = new Object() {
                String apply(int l, int r) {
                    if (l == r) {
                        return strs[l];
                    }
                    int mid = (l + r) / 2;
                    String lcpLeft = apply(l, mid);
                    String lcpRight = apply(mid + 1, r);
                    return commonPrefix(lcpLeft, lcpRight);
                }

                String commonPrefix(String left, String right) {
                    int min = Math.min(left.length(), right.length());
                    for (int i = 0; i < min; i++) {
                        if (left.charAt(i) != right.charAt(i)) {
                            return left.substring(0, i);
                        }
                    }
                    return left.substring(0, min);
                }
            };
            return lcp.apply(0, strs.length - 1);
        }

        public String longestCommonPrefix4(String[] strs) {
            if (strs == null || strs.length == 0) {
                return "";
            }

            var isCommonPrefix = new Object() {
                boolean apply(int len) {
                    var str1 = strs[0].substring(0, len);
                    for (int i = 1; i < strs.length; i++) {
                        if (!strs[i].startsWith(str1)) {
                            return false;
                        }
                    }
                    return true;
                }
            };

            var minLen = Integer.MAX_VALUE;
            for (var str : strs) {
                minLen = Math.min(minLen, str.length());
            }

            var low = 1;
            var high = minLen;

            while (low <= high) {
                var mid = (low + high) / 2;
                if (isCommonPrefix.apply(mid)) {
                    low = mid + 1;
                } else {
                    high = mid - 1;
                }
            }

            return strs[0].substring(0, (low + high) / 2);
        }
    }

    // https://leetcode.com/problems/reverse-words-in-a-string/description
    static class Solution21 {
        public String reverseWords1(String s) {
            s = s.trim();
            var list = Arrays.asList(s.split("\\s+"));
            Collections.reverse(list);
            return String.join(" ", list);
        }

        public String reverseWords2(String s) {
            var trimSpaces = new Object() {
                StringBuilder apply(String s) {
                    int left = 0, right = s.length() - 1;
                    while (left <= right && s.charAt(left) == ' ')
                        ++left;
                    while (left <= right && s.charAt(right) == ' ')
                        --right;

                    var sb = new StringBuilder();
                    while (left <= right) {
                        char c = s.charAt(left);
                        if (c != ' ')
                            sb.append(c);
                        else if (sb.charAt(sb.length() - 1) != ' ')
                            sb.append(c);
                        ++left;
                    }
                    return sb;
                }
            };

            var reverse = new Object() {
                void apply(StringBuilder sb, int left, int right) {
                    while (left < right) {
                        var t = sb.charAt(left);
                        sb.setCharAt(left, sb.charAt(right));
                        sb.setCharAt(right, t);
                        left++;
                        right--;
                    }
                }
            };

            var reverseEachWord = new Object() {
                void apply(StringBuilder sb) {
                    int n = sb.length();
                    int start = 0, end = 0;

                    while (start < n) {
                        while (end < n && sb.charAt(end) != ' ')
                            ++end;
                        reverse.apply(sb, start, end - 1);
                        start = end + 1;
                        end++;
                    }
                }
            };

            var sb = trimSpaces.apply(s);
            reverse.apply(sb, 0, sb.length() - 1);
            reverseEachWord.apply(sb);
            return sb.toString();
        }

        public String reverseWords3(String s) {
            int left = 0, right = s.length() - 1;
            while (left <= right && s.charAt(left) == ' ')
                ++left;
            while (left <= right && s.charAt(right) == ' ')
                --right;

            var d = new ArrayDeque<String>();
            StringBuilder word = new StringBuilder();

            while (left <= right) {
                char c = s.charAt(left);

                if ((word.length() != 0) && (c == ' ')) {
                    d.offerFirst(word.toString());
                    word.setLength(0);
                } else if (c != ' ') {
                    word.append(c);
                }
                ++left;
            }

            d.offerFirst(word.toString());

            return String.join(" ", d);
        }
    }
}
