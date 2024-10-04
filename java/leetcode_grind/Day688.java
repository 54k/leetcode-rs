package leetcode_grind;

import java.util.*;

public class Day688 {
    // https://leetcode.com/problems/largest-time-for-given-digits/description/
    static class Solution1 {
        int max_time = -1;

        public String largestTimeFromDigits(int[] A) {
            this.max_time = -1;
            for (int i1 = 0; i1 < A.length; ++i1) {
                for (int i2 = 0; i2 < A.length; ++i2) {
                    for (int i3 = 0; i3 < A.length; ++i3) {
                        if (i1 == i2 || i2 == i3 || i1 == i3) {
                            continue;
                        }
                        int i4 = 6 - i1 - i2 - i3;
                        int[] perm = { A[i1], A[i2], A[i3], A[i4] };
                        validateTime(perm);
                    }
                }
            }
            if (this.max_time == -1) {
                return "";
            } else {
                return String.format("%02d:%02d", max_time / 60, max_time % 60);
            }
        }

        void validateTime(int[] perm) {
            int hour = perm[0] * 10 + perm[1];
            int minute = perm[2] * 10 + perm[3];
            if (hour < 24 && minute < 60) {
                this.max_time = Math.max(this.max_time, hour * 60 + minute);
            }
        }
    }

    static class Solution2 {
        int max_time = -1;

        public String largestTimeFromDigits(int[] A) {
            this.max_time = -1;
            permutate(A, 0);
            if (this.max_time == -1) {
                return "";
            } else {
                return String.format("%02d:%02d", max_time / 60, max_time % 60);
            }
        }

        void permutate(int[] array, int start) {
            if (start == array.length) {
                this.build_time(array);
                return;
            }

            for (int i = start; i < array.length; i++) {
                this.swap(array, i, start);
                this.permutate(array, start + 1);
                this.swap(array, i, start);
            }
        }

        void build_time(int[] perm) {
            int hour = perm[0] * 10 + perm[1];
            int minute = perm[2] * 10 + perm[3];
            if (hour < 24 && minute < 60) {
                this.max_time = Math.max(this.max_time, hour * 60 + minute);
            }
        }

        void swap(int[] array, int i, int j) {
            if (i != j) {
                int temp = array[i];
                array[i] = array[j];
                array[j] = temp;
            }
        }
    }
}
