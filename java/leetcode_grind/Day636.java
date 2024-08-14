package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.List;

public class Day636 {
    // https://leetcode.com/problems/find-k-th-smallest-pair-distance/description/?envType=daily-question&envId=2024-08-14
    static class Solution1 {
        public int smallestDistancePair(int[] nums, int k) {
            int arraylength = nums.length;
            int maxElement = Integer.MIN_VALUE;
            for (int num : nums) {
                maxElement = Math.max(maxElement, num);
            }
            int[] distanceBucket = new int[maxElement + 1];
            for (int i = 0; i < arraylength; i++) {
                for (int j = i + 1; j < arraylength; j++) {
                    int distance = Math.abs(nums[i] - nums[j]);
                    ++distanceBucket[distance];
                }
            }
            for (int dist = 0; dist <= maxElement; dist++) {
                k -= distanceBucket[dist];
                if (k <= 0) {
                    return dist;
                }
            }
            return -1;
        }
    }

    static class Solution2 {
        public int smallestDistancePair(int[] nums, int k) {
            Arrays.sort(nums);
            int arraySize = nums.length;

            int maxElement = nums[arraySize - 1];
            int prefixCountSize = maxElement * 2;

            int[] prefixCount = new int[prefixCountSize];
            int[] valueCount = new int[maxElement + 1];

            int prefixIndex = 0;
            for (int value = 0; value < prefixCountSize; value++) {
                while (prefixIndex < arraySize && nums[prefixIndex] <= value) {
                    ++prefixIndex;
                }
                prefixCount[value] = prefixIndex;
            }
            for (int i = 0; i < arraySize; i++) {
                ++valueCount[nums[i]];
            }

            int left = 0;
            int right = maxElement;
            while (left < right) {
                int middle = (left + right) / 2;
                int count = countPairs(nums, prefixCount, valueCount, middle);
                if (count < k) {
                    left = middle + 1;
                } else {
                    right = middle;
                }
            }
            return left;
        }

        int countPairs(int[] nums, int[] prefixCount, int[] valueCount, int maxDistance) {
            int count = 0;
            int arraySize = nums.length;
            int index = 0;
            while (index < arraySize) {
                int currentValue = nums[index];
                int valueCountForCurrent = valueCount[currentValue];
                int pairsWithLargerValues = prefixCount[Math.min(currentValue + maxDistance, prefixCount.length - 1)] -
                        prefixCount[currentValue];
                int pairsWithSameValues = (valueCountForCurrent * (valueCountForCurrent - 1)) / 2;
                count += pairsWithLargerValues * valueCountForCurrent + pairsWithSameValues;

                while (index + 1 < arraySize && nums[index] == nums[index + 1]) {
                    ++index;
                }
                ++index;
            }
            return count;
        }
    }

    static class Solution3 {
        public int smallestDistancePair(int[] nums, int k) {
            Arrays.sort(nums);
            int arraySize = nums.length;
            int low = 0;
            int high = nums[arraySize - 1] - nums[0];
            while (low < high) {
                int mid = (low + high) / 2;
                int count = countPairsWithMaxDistance(nums, mid);
                if (count < k) {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }
            return low;
        }

        int countPairsWithMaxDistance(int[] nums, int maxDistance) {
            int count = 0;
            int arraySize = nums.length;
            int left = 0;
            for (int right = 0; right < arraySize; ++right) {
                while (nums[right] - nums[left] > maxDistance) {
                    ++left;
                }
                count += right - left;
            }
            return count;
        }
    }

    // https://leetcode.com/problems/kth-smallest-number-in-multiplication-table/description/
    static class Solution4 {
        boolean enough(int x, int m, int n, int k) {
            int count = 0;
            for (int i = 1; i <= m; i++) {
                count += Math.min(x / i, n);
            }
            return count >= k;
        }

        public int findKthNumber(int m, int n, int k) {
            int lo = 1, hi = m * n;
            while (lo < hi) {
                int mi = lo + (hi - lo) / 2;
                if (!enough(mi, m, n, k)) {
                    lo = mi + 1;
                } else {
                    hi = mi;
                }
            }
            return lo;
        }
    }

    // https://leetcode.com/problems/reorder-data-in-log-files/description/
    static class Solution5 {
        public String[] reorderLogFiles(String[] logs) {
            Comparator<String> myComp = new Comparator<String>() {
                @Override
                public int compare(String log1, String log2) {
                    String[] split1 = log1.split(" ", 2);
                    String[] split2 = log2.split(" ", 2);
                    boolean isDigit1 = Character.isDigit(split1[1].charAt(0));
                    boolean isDigit2 = Character.isDigit(split2[1].charAt(0));

                    if (!isDigit1 && !isDigit2) {
                        int cmp = split1[1].compareTo(split2[1]);
                        if (cmp != 0) {
                            return cmp;
                        }
                        return split1[0].compareTo(split2[0]);
                    }

                    if (!isDigit1 && isDigit2) {
                        return -1;
                    } else if (isDigit1 && !isDigit2) {
                        return 1;
                    } else {
                        return 0;
                    }
                }
            };

            Arrays.sort(logs, myComp);
            return logs;
        }
    }

    // https://leetcode.com/problems/coin-path/description/
    static class Solution6 {
        public List<Integer> cheapestJump(int[] A, int B) {
            int[] next = new int[A.length];
            Arrays.fill(next, -1);
            jump(A, B, 0, next);
            List<Integer> res = new ArrayList<>();
            int i;
            for (i = 0; i < A.length && next[i] > 0; i = next[i]) {
                res.add(i + 1);
            }
            if (i == A.length - 1 && A[i] >= 0) {
                res.add(A.length);
            } else {
                return new ArrayList<Integer>();
            }
            return res;
        }

        long jump(int[] A, int B, int i, int[] next) {
            if (i == A.length - 1 && A[i] >= 0) {
                return A[i];
            }
            long min_cost = Integer.MAX_VALUE;
            for (int j = i + 1; j <= i + B && j < A.length; j++) {
                if (A[j] >= 0) {
                    long cost = A[i] + jump(A, B, j, next);
                    if (cost < min_cost) {
                        min_cost = cost;
                        next[i] = j;
                    }
                }
            }
            return min_cost;
        }
    }

    static class Solution7 {
        public List<Integer> cheapestJump(int[] A, int B) {
            int[] next = new int[A.length];
            Arrays.fill(next, -1);
            long[] memo = new long[A.length];
            jump(A, B, 0, next, memo);
            List<Integer> res = new ArrayList<>();
            int i;
            for (i = 0; i < A.length && next[i] > 0; i = next[i]) {
                res.add(i + 1);
            }
            if (i == A.length - 1 && A[i] >= 0) {
                res.add(A.length);
            } else {
                return new ArrayList<Integer>();
            }
            return res;
        }

        long jump(int[] A, int B, int i, int[] next, long[] memo) {
            if (memo[i] > 0) {
                return memo[i];
            }
            if (i == A.length - 1 && A[i] >= 0) {
                return A[i];
            }
            long min_cost = Integer.MAX_VALUE;
            for (int j = i + 1; j <= i + B && j < A.length; j++) {
                if (A[j] >= 0) {
                    long cost = A[i] + jump(A, B, j, next, memo);
                    if (cost < min_cost) {
                        min_cost = cost;
                        next[i] = j;
                    }
                }
            }
            memo[i] = min_cost;
            return min_cost;
        }
    }
}
