package data_structures_examples;

public class BinSearch {
    // https://leetcode.com/problems/search-insert-position/description/
    static class SearchInsertPosition {
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

    // https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/description/
    static class SearchRange {
        public int[] searchRange(int[] nums, int target) {
            var lowerBound1 = new Object() {
                int find(int target) {
                    int lo = -1;
                    int hi = nums.length;
                    while (lo + 1 < hi) {
                        int mid = (lo + hi) / 2;
                        if (nums[mid] < target) {
                            lo = mid;
                        } else {
                            hi = mid;
                        }
                    }
                    return nums.length == hi || nums[hi] != target ? -1 : hi;
                }
            };

            var lowerBound2 = new Object() {
                int find(int target) {
                    int lo = 0;
                    int hi = nums.length - 1;
                    while (lo <= hi) {
                        int mid = (lo + hi) / 2;
                        if (nums[mid] == target) {
                            if (mid == lo || nums[mid - 1] != target) {
                                return mid;
                            }
                            hi = mid - 1;
                        } else if (nums[mid] < target) {
                            lo = mid + 1;
                        } else {
                            hi = mid - 1;
                        }
                    }
                    return -1;
                }
            };

            var lowerBound3 = new Object() {
                int find(int target) {
                    int lo = 0;
                    int hi = nums.length - 1;
                    while (lo < hi) {
                        int mid = (lo + hi) / 2;
                        if (nums[mid] < target) {
                            lo = mid + 1;
                        } else {
                            hi = mid;
                        }
                    }
                    return lo < nums.length && nums[lo] == target ? lo : -1;
                }
            };

            var upperBound1 = new Object() {
                int find(int target) {
                    int lo = -1;
                    int hi = nums.length;
                    while (lo + 1 < hi) {
                        int mid = (lo + hi) / 2;
                        if (nums[mid] <= target) {
                            lo = mid;
                        } else {
                            hi = mid;
                        }
                    }
                    return nums[lo] == target ? lo : -1;
                }
            };

            var upperBound2 = new Object() {
                int find(int target) {
                    int lo = 0;
                    int hi = nums.length - 1;

                    while (lo <= hi) {
                        int mid = (lo + hi) / 2;
                        if (nums[mid] == target) {
                            if (mid == hi || nums[mid + 1] != target) {
                                return mid;
                            }
                            lo = mid + 1;
                        } else if (nums[mid] < target) {
                            lo = mid + 1;
                        } else {
                            hi = mid - 1;
                        }
                    }
                    return -1;
                }
            };

            var upperBound3 = new Object() {
                int find(int target) {
                    int lo = 0;
                    int hi = nums.length;

                    while (lo < hi) {
                        int mid = (lo + hi) / 2;
                        if (nums[mid] <= target) {
                            lo = mid + 1;
                        } else {
                            hi = mid;
                        }
                    }
                    return nums[lo - 1] == target ? lo - 1 : -1;
                }
            };

            int lower = lowerBound3.find(target);
            if (lower == -1) {
                return new int[] { -1, -1 };
            }

            int upper = upperBound3.find(target);
            if (upper == -1) {
                return new int[] { -1, -1 };
            }

            return new int[] { lower, upper };
        }
    }
}
