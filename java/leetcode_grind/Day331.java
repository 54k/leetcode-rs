package leetcode_grind;

public class Day331 {
    // https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/description
    static class Solution {
        public int[] searchRange(int[] nums, int target) {
            var findBound = new Object() {
                int bound(boolean isFirst) {
                    var lo = 0;
                    var hi = nums.length - 1;
                    while (lo <= hi) {
                        var mid = (lo + hi) / 2;
                        if (nums[mid] == target) {
                            if (isFirst) {
                                if (mid == lo || nums[mid - 1] != target) {
                                    return mid;
                                }

                                hi = mid - 1;
                            } else {
                                if (mid == hi || nums[mid + 1] != target) {
                                    return mid;
                                }

                                lo = mid + 1;
                            }
                        } else if (nums[mid] > target) {
                            hi = mid - 1;
                        } else {
                            lo = mid + 1;
                        }
                    }
                    return -1;
                }
            };

            var firstOccurence = findBound.bound(true);
            if (firstOccurence == -1) {
                return new int[] { -1, -1 };
            }
            var lastOccurence = findBound.bound(false);
            return new int[] { firstOccurence, lastOccurence };
        }
    }
}
