public class Day312 {
    static class Solution {
        public int findDuplicate(int[] nums) {
            var left = 1;
            var right = nums.length;
            var duplicate = -1;
            while (left <= right) {
                var mid = (left + right) / 2;

                var count = 0;
                for (var num : nums) {
                    if (num <= mid) {
                        count++;
                    }
                }

                if (count > mid) {
                    duplicate = mid;
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            return duplicate;
        }
    }
}
