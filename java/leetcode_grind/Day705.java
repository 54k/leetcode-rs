package leetcode_grind;

import java.util.*;

public class Day705 {
    // https://leetcode.com/problems/split-a-string-into-the-max-number-of-unique-substrings/description/
    static class Solution1 {
        public int maxUniqueSplit(String s) {
            Set<String> seen = new HashSet<>();
            return backtrack(s, 0, seen);
        }

        int backtrack(String s, int start, Set<String> seen) {
            if (start == s.length()) {
                return 0;
            }
            int maxCount = 0;
            for (int end = start + 1; end <= s.length(); ++end) {
                String substring = s.substring(start, end);
                if (!seen.contains(substring)) {
                    seen.add(substring);
                    maxCount = Math.max(maxCount, 1 + backtrack(s, end, seen));
                    seen.remove(substring);
                }
            }
            return maxCount;
        }
    }

    static class Solution2 {
        int ans = 0;

        public int maxUniqueSplit(String s) {
            var set = new HashSet<String>();
            var bc = new Object() {
                void apply(int start) {
                    if (start == s.length()) {
                        ans = Math.max(ans, set.size());
                        return;
                    }
                    for (int i = start + 1; i <= s.length(); i++) {
                        var ss = s.substring(start, i);
                        if (set.add(ss)) {
                            apply(i);
                            set.remove(ss);
                        }
                    }
                }
            };
            bc.apply(0);
            return ans;
        }
    }

    // https://leetcode.com/problems/meeting-rooms-iii/description/
    static class Solution3 {
        public int mostBooked(int n, int[][] meetings) {
            long[] roomAvailabilityTime = new long[n];
            int[] meetingCount = new int[n];
            Arrays.sort(meetings, (a, b) -> Integer.compare(a[0], b[0]));

            for (int[] meeting : meetings) {
                int start = meeting[0], end = meeting[1];
                long minRoomAvailabilityTime = Long.MAX_VALUE;
                int minAvailabilityTimeRoom = 0;
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
                        minAvailabilityTimeRoom = i;
                    }
                }

                if (!foundUnusedRoom) {
                    roomAvailabilityTime[minAvailabilityTimeRoom] += end - start;
                    meetingCount[minAvailabilityTimeRoom]++;
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

    static class Solution4 {
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
