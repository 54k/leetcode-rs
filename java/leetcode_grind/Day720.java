package leetcode_grind;

import java.util.*;

public class Day720 {
    // https://leetcode.com/problems/find-if-array-can-be-sorted/description/?envType=daily-question&envId=2024-11-06
    static class Solution1 {
        public boolean canSortArray(int[] nums) {
            int[] values = Arrays.copyOf(nums, nums.length);
            int n = values.length;

            for (int i = 0; i < n; i++) {
                for (int j = 0; j < n - i - 1; j++) {
                    if (values[j] <= values[j + 1]) {
                        continue;
                    } else {
                        if (Integer.bitCount(values[j]) == Integer.bitCount(values[j + 1])) {
                            int temp = values[j];
                            values[j] = values[j + 1];
                            values[j + 1] = temp;
                        } else {
                            return false;
                        }
                    }
                }
            }

            return true;
        }
    }

    static class Solution2 {
        public boolean canSortArray(int[] nums) {
            int numOfSetBits = Integer.bitCount(nums[0]);
            int maxOfSegment = nums[0];
            int minOfSegment = nums[0];

            int maxOfPrevSegment = Integer.MIN_VALUE;

            for (int i = 1; i < nums.length; i++) {
                if (Integer.bitCount(nums[i]) == numOfSetBits) {
                    maxOfSegment = Math.max(maxOfSegment, nums[i]);
                    minOfSegment = Math.min(minOfSegment, nums[i]);
                } else {
                    if (minOfSegment < maxOfPrevSegment) {
                        return false;
                    }

                    maxOfPrevSegment = maxOfSegment;
                    maxOfSegment = nums[i];
                    minOfSegment = nums[i];
                    numOfSetBits = Integer.bitCount(nums[i]);
                }
            }
            return (minOfSegment < maxOfPrevSegment) ? false : true;
        }
    }

    static class Solution3 {
        public boolean canSortArray(int[] nums) {
            int n = nums.length;
            int[] values = Arrays.copyOf(nums, n);
            for (int i = 0; i < n - 1; i++) {
                if (values[i] <= values[i + 1]) {
                    continue;
                } else {
                    if (Integer.bitCount(values[i]) == Integer.bitCount(values[i + 1])) {
                        int temp = values[i];
                        values[i] = values[i + 1];
                        values[i + 1] = temp;
                    } else {
                        return false;
                    }
                }
            }
            for (int i = n - 1; i >= 1; i--) {
                if (values[i] >= values[i - 1]) {
                    continue;
                } else {
                    if (Integer.bitCount(values[i]) == Integer.bitCount(values[i - 1])) {
                        int temp = values[i];
                        values[i] = values[i - 1];
                        values[i - 1] = temp;
                    } else {
                        return false;
                    }
                }
            }
            return true;
        }
    }

    // https://leetcode.com/problems/prison-cells-after-n-days/description/
    static class Solution4 {
        int cellsToBitmap(int[] cells) {
            int stateBitmap = 0;
            for (int cell : cells) {
                stateBitmap <<= 1;
                stateBitmap = (stateBitmap | cell);
            }
            return stateBitmap;
        }

        int[] nextDay(int[] cells) {
            int[] newCells = new int[cells.length];
            newCells[0] = 0;
            for (int i = 1; i < cells.length - 1; i++) {
                newCells[i] = (cells[i - 1] == cells[i + 1]) ? 1 : 0;
            }
            newCells[cells.length - 1] = 0;
            return newCells;
        }

        public int[] prisonAfterNDays(int[] cells, int N) {
            Map<Integer, Integer> seen = new HashMap<>();
            boolean isFastForwarded = false;

            while (N > 0) {
                if (!isFastForwarded) {
                    int stateBitmap = this.cellsToBitmap(cells);
                    if (seen.containsKey(stateBitmap)) {
                        N %= seen.get(stateBitmap) - N;
                        isFastForwarded = true;
                    } else {
                        seen.put(stateBitmap, N);
                    }
                }

                if (N > 0) {
                    N -= 1;
                    cells = this.nextDay(cells);
                }
            }
            return cells;
        }
    }

    static class Solution5 {
        int nextDay(int stateBitmap) {
            stateBitmap = ~(stateBitmap << 1) ^ (stateBitmap >> 1);
            stateBitmap = stateBitmap & 0x7e;
            return stateBitmap;
        }

        public int[] prisonAfterNDays(int[] cells, int N) {
            Map<Integer, Integer> seen = new HashMap<>();
            boolean isFastForwarded = false;

            int stateBitmap = 0x0;
            for (int cell : cells) {
                stateBitmap <<= 1;
                stateBitmap = (stateBitmap | cell);
            }

            while (N > 0) {
                if (!isFastForwarded) {
                    if (seen.containsKey(stateBitmap)) {
                        N %= seen.get(stateBitmap) - N;
                        isFastForwarded = true;
                    } else {
                        seen.put(stateBitmap, N);
                    }
                }

                if (N > 0) {
                    N -= 1;
                    stateBitmap = this.nextDay(stateBitmap);
                }
            }
            int ret[] = new int[cells.length];
            for (int i = cells.length - 1; i >= 0; i--) {
                ret[i] = (stateBitmap & 0x1);
                stateBitmap = stateBitmap >> 1;
            }
            return ret;
        }
    }
}
