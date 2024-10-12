package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.TreeMap;

public class Day696 {
    // https://leetcode.com/problems/meeting-rooms-ii/description/
    static class Solution1 {
        public int minMeetingRooms(int[][] intervals) {
            if (intervals.length == 0) {
                return 0;
            }

            PriorityQueue<Integer> allocator = new PriorityQueue<>(
                    intervals.length,
                    (a, b) -> a - b);

            Arrays.sort(intervals, (a, b) -> a[0] - b[0]);

            allocator.add(intervals[0][1]);

            for (int i = 1; i < intervals.length; i++) {
                if (intervals[i][0] >= allocator.peek()) {
                    allocator.poll();
                }

                allocator.add(intervals[i][1]);
            }

            return allocator.size();
        }
    }

    static class Solution2 {
        public int minMeetingRooms(int[][] intervals) {
            if (intervals.length == 0) {
                return 0;
            }

            Integer[] start = new Integer[intervals.length];
            Integer[] end = new Integer[intervals.length];

            for (int i = 0; i < intervals.length; i++) {
                start[i] = intervals[i][0];
                end[i] = intervals[i][1];
            }

            Arrays.sort(
                    start, (a, b) -> a - b);
            Arrays.sort(
                    end, (a, b) -> a - b);

            int startPointer = 0, endPointer = 0;
            int usedRooms = 0;

            while (startPointer < intervals.length) {
                if (start[startPointer] >= end[endPointer]) {
                    usedRooms -= 1;
                    endPointer += 1;
                }
                usedRooms += 1;
                startPointer += 1;
            }
            return usedRooms;
        }
    }

    // https://leetcode.com/problems/divide-intervals-into-minimum-number-of-groups/description/?envType=daily-question&envId=2024-10-12
    static class Solution3 {
        public int minGroups(int[][] intervals) {
            var n = intervals.length;
            var start = new int[n];
            var end = new int[n];
            var ans = 0;
            for (int i = 0; i < n; i++) {
                start[i] = intervals[i][0];
                end[i] = intervals[i][1];
            }
            Arrays.sort(start);
            Arrays.sort(end);
            for (int i = 0, j = 0; i < n; i++) {
                if (start[i] > end[j]) {
                    ans--;
                    j++;
                }
                ans++;
            }
            return ans;
        }
    }

    static class Solution4 {
        public int minGroups(int[][] intervals) {
            List<int[]> events = new ArrayList<>();
            for (int[] interval : intervals) {
                events.add(new int[] { interval[0], 1 });
                events.add(new int[] { interval[1] + 1, -1 });
            }
            Collections.sort(events, (a, b) -> {
                if (a[0] == b[0]) {
                    return Integer.compare(a[1], b[1]);
                }
                return Integer.compare(a[0], b[0]);
            });

            int concurrentIntervals = 0;
            int maxConcurrentIntervals = 0;

            for (int[] event : events) {
                concurrentIntervals += event[1];
                maxConcurrentIntervals = Math.max(
                        maxConcurrentIntervals,
                        concurrentIntervals);
            }
            return maxConcurrentIntervals;
        }
    }

    static class Solution5 {
        public int minGroups(int[][] intervals) {
            TreeMap<Integer, Integer> pointToCount = new TreeMap<>();
            for (int[] interval : intervals) {
                pointToCount.put(
                        interval[0],
                        pointToCount.getOrDefault(interval[0], 0) + 1);
                pointToCount.put(
                        interval[1] + 1,
                        pointToCount.getOrDefault(interval[1] + 1, 0) - 1);
            }

            int concurrentIntervals = 0;
            int maxConcurrentIntervals = 0;

            for (Map.Entry<Integer, Integer> entry : pointToCount.entrySet()) {
                concurrentIntervals += entry.getValue();
                maxConcurrentIntervals = Math.max(
                        maxConcurrentIntervals,
                        concurrentIntervals);
            }
            return maxConcurrentIntervals;
        }
    }

    static class Solution6 {
        public int minGroups(int[][] intervals) {
            int rangeStart = Integer.MAX_VALUE;
            int rangeEnd = Integer.MIN_VALUE;
            for (int[] interval : intervals) {
                rangeStart = Math.min(rangeStart, interval[0]);
                rangeEnd = Math.max(rangeEnd, interval[1]);
            }
            int[] pointToCount = new int[rangeEnd + 2];
            for (int[] interval : intervals) {
                pointToCount[interval[0]]++;
                pointToCount[interval[1] + 1]--;
            }
            int concurrentIntervals = 0;
            int maxConcurrentIntervals = 0;
            for (int i = rangeStart; i <= rangeEnd; i++) {
                concurrentIntervals += pointToCount[i];
                maxConcurrentIntervals = Math.max(
                        maxConcurrentIntervals,
                        concurrentIntervals);
            }
            return maxConcurrentIntervals;
        }
    }

    // https://leetcode.com/problems/meeting-rooms-iii/description/
    static class Solution7 {
        public int mostBooked(int n, int[][] meetings) {
            long[] roomAvailabilityTime = new long[n];
            int[] meetingCount = new int[n];
            Arrays.sort(meetings, (a, b) -> Integer.compare(a[0], b[0]));

            for (int[] meeting : meetings) {
                int start = meeting[0], end = meeting[1];
                long minRoomAvailabilityTime = Long.MAX_VALUE;
                int minAvailableTimeRoom = 0;
                boolean foundUnusedRoom = false;

                for (int i = 0; i < n; i++) {
                    if (roomAvailabilityTime[i] <= start) {
                        foundUnusedRoom = true;
                        meetingCount[i]++;
                        roomAvailabilityTime[i] = end;
                        break;
                    }

                    if (minRoomAvailabilityTime > roomAvailabilityTime[i]) {
                        minRoomAvailabilityTime = roomAvailabilityTime[i];
                        minAvailableTimeRoom = i;
                    }
                }

                if (!foundUnusedRoom) {
                    roomAvailabilityTime[minAvailableTimeRoom] += end - start;
                    meetingCount[minAvailableTimeRoom]++;
                }
            }

            int maxMeetingCount = 0, maxMeetingCountRoom = 0;
            for (int i = 0; i < n; i++) {
                if (meetingCount[i] > maxMeetingCount) {
                    maxMeetingCount = meetingCount[i];
                    maxMeetingCountRoom = i;
                }
            }
            return maxMeetingCountRoom;
        }
    }

    static class Solution8 {
        public int mostBooked(int n, int[][] meetings) {
            var meetingCount = new int[n];
            var usedRooms = new PriorityQueue<long[]>(
                    (a, b) -> a[0] != b[0] ? Long.compare(a[0], b[0]) : Long.compare(a[1], b[1]));
            var unusedRooms = new PriorityQueue<Integer>();

            for (int i = 0; i < n; i++) {
                unusedRooms.offer(i);
            }

            Arrays.sort(meetings, (a, b) -> a[0] != b[0] ? Integer.compare(a[0], b[0]) : Integer.compare(a[1], b[1]));

            for (int[] meeting : meetings) {
                int start = meeting[0], end = meeting[1];

                while (!usedRooms.isEmpty() && usedRooms.peek()[0] <= start) {
                    int room = (int) usedRooms.poll()[1];
                    unusedRooms.offer(room);
                }

                if (!unusedRooms.isEmpty()) {
                    int room = unusedRooms.poll();
                    usedRooms.offer(new long[] { end, room });
                    meetingCount[room]++;
                } else {
                    long roomAvailabilityTime = usedRooms.peek()[0];
                    int room = (int) usedRooms.poll()[1];
                    usedRooms.offer(new long[] { roomAvailabilityTime + end - start, room });
                    meetingCount[room]++;
                }
            }

            int maxMeetingCount = 0, maxMeetingCountRoom = 0;
            for (int i = 0; i < n; i++) {
                if (meetingCount[i] > maxMeetingCount) {
                    maxMeetingCount = meetingCount[i];
                    maxMeetingCountRoom = i;
                }
            }
            return maxMeetingCountRoom;
        }
    }
}
