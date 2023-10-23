package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Objects;
import java.util.PriorityQueue;

public class Day345 {
    // https://leetcode.com/problems/power-of-two/description/
    static class Solution1 {
        public boolean isPowerOfTwo1(int n) {
            if (n == 0)
                return false;
            long x = (long) n;
            return (x & (-x)) == x;
        }

        public boolean isPowerOfTwo2(int n) {
            if (n == 0)
                return false;
            long x = (long) n;
            return (x & (x - 1)) == 0;
        }
    }

    // https://leetcode.com/problems/power-of-three/description/
    static class Solution2 {
        public boolean isPowerOfThreeConvertToBase3(int n) {
            if (n == 0)
                return false;
            var base3 = "";
            while (n > 0) {
                base3 = Objects.toString(n % 3) + base3;
                n /= 3;
            }
            return Objects.toString(base3).matches("^10*$");
        }

        public boolean isPowerOfThree(int n) {
            return n > 0 && 1162261467 % n == 0;
        }
    }

    // https://leetcode.com/problems/power-of-four/description/
    static class Solution3 {
        public boolean isPowerOfFour(int num) {
            return (num > 0) && (num & (num - 1)) == 0 && (num & 0xaaaaaaaa) == 0;
        }
    }

    // https://leetcode.com/problems/insert-interval/description/
    static class Solution4 {
        public int[][] insertLinear(int[][] intervals, int[] newInterval) {
            var doesIntervalsOverlap = new Object() {
                boolean apply(int[] a, int[] b) {
                    return Math.min(a[1], b[1]) - Math.max(a[0], b[0]) >= 0;
                }
            };
            var mergeIntervals = new Object() {
                int[] apply(int[] a, int[] b) {
                    var newInterval = new int[2];
                    newInterval[0] = Math.min(a[0], b[0]);
                    newInterval[1] = Math.max(a[1], b[1]);
                    return newInterval;
                }
            };
            var insertInterval = new Object() {
                int[][] apply(int[][] intervals, int[] newInterval) {
                    var isIntervalInserted = false;
                    List<int[]> list = new ArrayList<>(Arrays.asList(intervals));

                    for (int i = 0; i < intervals.length; i++) {
                        if (newInterval[0] < intervals[i][0]) {
                            list.add(i, newInterval);
                            isIntervalInserted = true;
                            break;
                        }
                    }

                    if (!isIntervalInserted) {
                        list.add(newInterval);
                    }
                    return list.toArray(new int[list.size()][2]);
                }
            };

            intervals = insertInterval.apply(intervals, newInterval);
            List<int[]> answer = new ArrayList<>();

            for (int i = 0; i < intervals.length; i++) {
                int[] currInterval = { intervals[i][0], intervals[i][1] };
                while (i < intervals.length && doesIntervalsOverlap.apply(currInterval, intervals[i])) {
                    currInterval = mergeIntervals.apply(intervals[i], currInterval);
                    i++;
                }
                i--;
                answer.add(currInterval);
            }
            return answer.toArray(new int[][] {});
        }

        public int[][] insertBinSearch(int[][] intervals, int[] newInterval) {
            @FunctionalInterface
            interface Function2<P1, P2, R> {
                R apply(P1 p1, P2 p2);
            }

            Function2<int[], int[], Boolean> isOverlap = (a, b) -> Math.min(b[1], b[1]) - Math.max(a[0], b[0]) >= 0;

            Function2<int[], int[], int[]> mergeInterval = (a,
                    b) -> new int[] { Math.min(a[0], b[0]), Math.max(a[1], b[1]) };

            Function2<int[][], int[], int[][]> insert = (a, b) -> {
                var result = new ArrayList<int[]>(Arrays.asList(a));
                var lo = 0;
                var hi = a.length - 1;
                var insertIdx = a.length;
                while (lo <= hi) {
                    var mid = (lo + hi) / 2;
                    if (a[mid][0] >= b[0]) {
                        insertIdx = mid;
                        hi = mid - 1;
                    } else {
                        lo = mid + 1;
                    }
                }
                if (insertIdx < result.size()) {
                    result.add(insertIdx, newInterval);
                } else {
                    result.add(newInterval);
                }
                return result.toArray(new int[][] {});
            };

            intervals = insert.apply(intervals, newInterval);
            var ans = new ArrayList<int[]>();
            for (int i = 0; i < intervals.length; i++) {
                var curr = intervals[i];
                while (i < intervals.length && isOverlap.apply(intervals[i], curr)) {
                    curr = mergeInterval.apply(intervals[i], curr);
                    i++;
                }
                i--;
                ans.add(curr);
            }
            return ans.toArray(new int[0][0]);
        }

        public int[][] insertBinSearch2(int[][] intervals, int[] newInterval) {
            var list = new ArrayList<int[]>(Arrays.asList(intervals));
            var lo = 0;
            var hi = intervals.length;
            while (lo < hi) {
                var mid = (lo + hi) / 2;
                if (intervals[mid][0] <= newInterval[0]) {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            if (lo < list.size()) {
                list.add(lo, newInterval);
            } else {
                list.add(newInterval);
            }

            var res = new ArrayList<int[]>();
            for (int i = 0; i < list.size(); i++) {
                if (res.isEmpty() || (res.get(res.size() - 1)[1] < list.get(i)[0])) {
                    res.add(list.get(i));
                } else {
                    res.get(res.size() - 1)[1] = Math.max(res.get(res.size() - 1)[1], list.get(i)[1]);
                }
            }
            return res.toArray(new int[][] {});
        }
    }

    // https://leetcode.com/problems/merge-intervals/description/
    static class Solution5 {
        public int[][] merge(int[][] intervals) {
            Arrays.sort(intervals, (a, b) -> a[0] - b[0]);
            var ans = new ArrayList<int[]>();
            for (int i = 0; i < intervals.length; i++) {
                if (ans.isEmpty() || ans.get(ans.size() - 1)[1] < intervals[i][0]) {
                    ans.add(intervals[i]);
                } else {
                    ans.get(ans.size() - 1)[1] = Math.max(ans.get(ans.size() - 1)[1], intervals[i][1]);
                }
            }
            return ans.toArray(new int[][] {});
        }
    }

    // https://leetcode.com/problems/non-overlapping-intervals/description/
    static class Solution6 {
        public int eraseOverlapIntervalsFindMaxNonOverlaping(int[][] intervals) {
            Arrays.sort(intervals, (a, b) -> a[1] - b[1]);
            var ans = 0;
            var k = Integer.MIN_VALUE; // most recent end time
            for (var interval : intervals) {
                if (interval[0] >= k) {
                    k = interval[1];
                } else {
                    ans++;
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/meeting-rooms-ii/description/
    static class Solution7 {
        public int minMeetingRoomsPriorityQueue(int[][] intervals) {
            Arrays.sort(intervals, (a, b) -> a[0] - b[0]);
            var pq = new PriorityQueue<Integer>();
            var ans = 0;
            for (var i : intervals) {
                if (!pq.isEmpty() && pq.peek() <= i[0]) {
                    pq.poll();
                    ans--;
                }
                pq.add(i[1]);
                ans++;
            }
            return ans;
        }

        public int minMeetingRoomsChronoOrdering(int[][] intervals) {
            var n = intervals.length;
            var starts = new int[n];
            var ends = new int[n];
            for (int i = 0; i < n; i++) {
                starts[i] = intervals[i][0];
                ends[i] = intervals[i][1];
            }

            Arrays.sort(starts);
            Arrays.sort(ends);

            var sPtr = 0;
            var ePtr = 0;

            var ans = 0;
            while (sPtr < n) {
                if (starts[sPtr] >= ends[ePtr]) {
                    ans--;
                    ePtr++;
                }
                sPtr++;
                ans++;
            }
            return ans;
        }
    }
}
