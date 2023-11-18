package data_structures_examples;

public class BinSearch {
    // https://leetcode.com/problems/search-insert-position/description/
    static class Solution1 {
        public int searchInsert1(int[] nums, int target) {
            var lo = -1;
            var hi = nums.length;

            while (hi - lo > 1) {
                var mid = (lo + hi) / 2;

                if (nums[mid] < target) {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            return hi;
        }

        public int searchInsert2(int[] nums, int target) {
            var lo = 0;
            var hi = nums.length;
            while (lo < hi) {
                var mid = (lo + hi) / 2;
                if (nums[mid] < target) {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            return lo;
        }

        public int searchInsert3(int[] nums, int target) {
            var lo = 0;
            var hi = nums.length - 1;
            while (lo <= hi) {
                var mid = (lo + hi) / 2;
                if (nums[mid] == target) {
                    return mid;
                }
                if (nums[mid] < target) {
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }
            return lo;
        }
    }

    static class FirstBadVersion {
        /*
         * The isBadVersion API is defined in the parent class VersionControl.
         * boolean isBadVersion(int version);
         */
        public class VersionControl {
            boolean isBadVersion(int n) {
                return false;
            }
        }

        public class Solution extends VersionControl {
            public int firstBadVersion1(int n) {
                var lo = 1;
                var hi = n;
                while (lo < hi) {
                    var mid = lo + (hi - lo) / 2;
                    if (isBadVersion(mid)) {
                        hi = mid;
                    } else {
                        lo = mid + 1;
                    }
                }
                return lo;
            }

            public int firstBadVersion2(int n) {
                var lo = 0;
                var hi = n;
                while (lo + 1 < hi) {
                    var mid = lo + (hi - lo) / 2;
                    if (isBadVersion(mid)) {
                        hi = mid;
                    } else {
                        lo = mid;
                    }
                }
                return hi;
            }

            public int firstBadVersion3(int n) {
                var lo = 1;
                var hi = n;
                while (lo <= hi) {
                    var mid = lo + (hi - lo) / 2;
                    if (isBadVersion(mid)) {
                        hi = mid - 1;
                    } else {
                        lo = mid + 1;
                    }
                }
                return lo;
            }
        }
    }
}
