package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day371 {
    // https://leetcode.com/problems/frequency-of-the-most-frequent-element/description
    static class Solution1 {
        public int maxFrequency1(int[] nums, int k) {
            Arrays.sort(nums);
            int ans = 0;
            int left = 0;
            int running = 0;
            for (int right = 0; right < nums.length; right++) {
                running += nums[right];
                while (nums[right] * (right - left + 1) - running > k) {
                    running -= nums[left++];
                }
                ans = Math.max(ans, right - left + 1);
            }
            return ans;
        }

        public int maxFrequency2(int[] nums, int k) {
            Arrays.sort(nums);
            int left = 0;
            int running = 0;

            for (int right = 0; right < nums.length; right++) {
                running += nums[right];

                if (nums[right] * (right - left + 1) - running > k) {
                    running -= nums[left++];
                }
            }
            return nums.length - left;
        }

        public int maxFrequency3(int[] nums, int k) {
            Arrays.sort(nums);
            int ans = 0;

            long[] prefix = new long[nums.length];
            prefix[0] = nums[0];
            for (int i = 1; i < prefix.length; i++) {
                prefix[i] = prefix[i - 1] + nums[i];
            }

            for (int i = 0; i < nums.length; i++) {
                int target = nums[i];

                int lo = 0;
                int hi = i;
                int best = i;

                while (lo <= hi) {
                    int mid = (lo + hi) / 2;

                    long count = i - mid + 1;
                    long finalSum = count * target;
                    long originalSum = prefix[i] - prefix[mid] + nums[mid]; // fancy variant for prefix[0 - 1] case
                    long operationsRequired = finalSum - originalSum;

                    if (operationsRequired > k) {
                        lo = mid + 1;
                    } else {
                        best = mid;
                        hi = mid - 1;
                    }
                }

                ans = Math.max(ans, i - best + 1);
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/range-frequency-queries/description/
    class RangeFreqQuery {
        Map<Integer, List<Integer>> idx = new HashMap<>();

        public RangeFreqQuery(int[] arr) {
            var i = 0;
            for (var num : arr) {
                idx.putIfAbsent(num, new ArrayList<>());
                idx.get(num).add(i);
                i++;
            }
        }

        int lower(List<Integer> list, int target) {
            int lo = -1;
            int hi = list.size();

            while (hi - lo > 1) {
                int mid = (lo + hi) / 2;

                if (list.get(mid) < target) {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }

            return hi == list.size() || list.get(hi) < target ? -1 : hi;
        }

        int upper(List<Integer> list, int target) {
            int lo = -1;
            int hi = list.size();

            while (hi - lo > 1) {
                int mid = (lo + hi) / 2;

                if (list.get(mid) <= target) {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            return lo > -1 && list.get(lo) <= target ? hi : -1;
        }

        public int query(int left, int right, int value) {
            var list = idx.get(value);
            if (list == null) {
                return 0;
            }
            var l = lower(list, left);
            if (l == -1) {
                return 0;
            }

            var r = upper(list, right);
            if (r == -1) {
                return 0;
            }

            return r - l;
        }
    }
}
