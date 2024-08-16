package leetcode_grind;

import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.TreeSet;

public class Day638 {
    // https://leetcode.com/problems/maximum-distance-in-arrays/description/?envType=daily-question&envId=2024-08-16
    static class Solution1 {
        public int maxDistance(List<List<Integer>> arrays) {
            int res = 0;
            int n = arrays.get(0).size();
            int min_val = arrays.get(0).get(0);
            int max_val = arrays.get(0).get(arrays.get(0).size() - 1);
            for (int i = 1; i < arrays.size(); i++) {
                n = arrays.get(i).size();
                res = Math.max(res, Math.max(
                        Math.abs(arrays.get(i).get(n - 1) - min_val),
                        Math.abs(max_val - arrays.get(i).get(0))));
                min_val = Math.min(min_val, arrays.get(i).get(0));
                max_val = Math.max(max_val, arrays.get(i).get(n - 1));
            }
            return res;
        }
    }

    // https://leetcode.com/problems/contains-duplicate-iii/description/
    static class Solution2 {
        public boolean containsNearbyAlmostDuplicate(int[] nums, int k, int t) {
            TreeSet<Integer> set = new TreeSet<>();
            for (int i = 0; i < nums.length; i++) {
                Integer s = set.ceiling(nums[i]);
                if (s != null && s <= nums[i] + t)
                    return true;
                Integer g = set.floor(nums[i]);
                if (g != null && nums[i] <= g + t)
                    return true;
                set.add(nums[i]);
                if (set.size() > k) {
                    set.remove(nums[i - k]);
                }
            }
            return false;
        }
    }

    // https://leetcode.com/problems/contains-duplicate-iii/description/
    static class Solution3 {
        long getID(int x, long w) {
            return Math.floorDiv(x, w);
        }

        public boolean containsNearbyAlmostDuplicate(int[] nums, int k, int t) {
            if (t < 0)
                return false;
            Map<Long, Long> buckets = new HashMap<>();
            long w = (long) t + 1;

            for (int i = 0; i < nums.length; i++) {
                long bucket = getID(nums[i], w);

                if (buckets.containsKey(bucket))
                    return true;
                if (buckets.containsKey(bucket - 1) && Math.abs(nums[i] - buckets.get(bucket - 1)) < w)
                    return true;
                if (buckets.containsKey(bucket + 1) && Math.abs(nums[i] - buckets.get(bucket + 1)) < w)
                    return true;

                buckets.put(bucket, (long) nums[i]);
                if (i >= k)
                    buckets.remove(getID(nums[i - k], w));
            }
            return false;
        }
    }
}
