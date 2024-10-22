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

    // https://leetcode.com/problems/the-knights-tour/description/?envType=weekly-question&envId=2024-10-22
    static class Solution5 {
        public int[][] tourOfKnight(int m, int n, int r, int c) {
            int[][] chessboard = new int[m][n];
            chessboard[r][c] = -1;
            solveKnightsTour(m, n, r, c, chessboard, 1);
            chessboard[r][c] = 0;
            return chessboard;
        }

        boolean solveKnightsTour(
                int rows,
                int cols,
                int currentRow,
                int currentCol,
                int[][] chessboard,
                int moveCount) {
            if (moveCount == rows * cols) {
                return true;
            }

            for (int newRow = 0; newRow < rows; newRow++) {
                for (int newCol = 0; newCol < cols; newCol++) {
                    if (!isValidMove(
                            chessboard,
                            currentRow,
                            currentCol,
                            newRow,
                            newCol)) {
                        continue;
                    }

                    chessboard[newRow][newCol] = moveCount;

                    if (solveKnightsTour(rows, cols, newRow, newCol, chessboard, moveCount + 1)) {
                        return true;
                    }

                    chessboard[newRow][newCol] = 0;
                }
            }
            return false;
        }

        boolean isValidMove(
                int[][] chessboard,
                int fromRow,
                int fromCol,
                int toRow,
                int toCol) {
            return (toRow >= 0 &&
                    toCol >= 0 &&
                    toRow < chessboard.length &&
                    toCol < chessboard[0].length &&
                    Math.min(Math.abs(fromRow - toRow), Math.abs(fromCol - toCol)) == 1 &&
                    Math.max(Math.abs(fromRow - toRow), Math.abs(fromCol - toCol)) == 2 &&
                    chessboard[toRow][toCol] == 0);
        }
    }

    static class Solution6 {

        // Possible knight moves: (row, column) pairs
        int[][] knightMoves = {
                { -1, -2 },
                { -2, -1 },
                { -1, 2 },
                { -2, 1 },
                { 1, -2 },
                { 2, -1 },
                { 1, 2 },
                { 2, 1 },
        };

        public int[][] tourOfKnight(int m, int n, int r, int c) {
            int[][] chessboard = new int[m][n];
            chessboard[r][c] = -1;

            solveKnightsTour(m, n, r, c, chessboard, 1);

            // Reset starting position to 0
            chessboard[r][c] = 0;
            return chessboard;
        }

        private boolean solveKnightsTour(
                int rows,
                int cols,
                int currentRow,
                int currentCol,
                int[][] chessboard,
                int moveCount) {
            // Base case: if all cells are visited, we've found a solution
            if (moveCount == rows * cols)
                return true;

            // Get and sort possible next moves based on Warnsdorff's rule
            List<int[]> nextMoves = getNextMovesWarnsdorff(
                    chessboard,
                    currentRow,
                    currentCol);

            // Try each possible move
            for (int[] move : nextMoves) {
                int nextRow = currentRow + knightMoves[move[1]][0];
                int nextCol = currentCol + knightMoves[move[1]][1];

                if (!isValidMove(chessboard, nextRow, nextCol))
                    continue;

                chessboard[nextRow][nextCol] = moveCount;

                // Recursively try to solve from this new position
                if (solveKnightsTour(
                        rows,
                        cols,
                        nextRow,
                        nextCol,
                        chessboard,
                        moveCount + 1)) {
                    return true;
                }

                // If the move doesn't lead to a solution, backtrack
                chessboard[nextRow][nextCol] = 0;
            }

            return false; // No solution found from this position
        }

        // Implement Warnsdorff's rule: prefer moves with fewer onward moves
        private List<int[]> getNextMovesWarnsdorff(
                int[][] chessboard,
                int row,
                int col) {
            List<int[]> nextMoves = new ArrayList<>();
            for (int moveIndex = 0; moveIndex < 8; moveIndex++) {
                int nextRow = row + knightMoves[moveIndex][0];
                int nextCol = col + knightMoves[moveIndex][1];
                int accessibilityScore = countAccessibleMoves(
                        chessboard,
                        nextRow,
                        nextCol);
                nextMoves.add(new int[] { accessibilityScore, moveIndex });
            }
            // Sort moves based on accessibility (fewer accessible squares first)
            Collections.sort(nextMoves, (a, b) -> a[0] - b[0]);
            return nextMoves;
        }

        private int countAccessibleMoves(int[][] chessboard, int row, int col) {
            int count = 0;
            for (int[] move : knightMoves) {
                int nextRow = row + move[0];
                int nextCol = col + move[1];
                if (isValidMove(chessboard, nextRow, nextCol))
                    count++;
            }
            return count;
        }

        private boolean isValidMove(int[][] chessboard, int row, int col) {
            return (row >= 0 &&
                    col >= 0 &&
                    row < chessboard.length &&
                    col < chessboard[0].length &&
                    chessboard[row][col] == 0);
        }
    }

    // https://leetcode.com/problems/find-peak-element/description/
    static class Solution7 {
        public int findPeakElement(int[] nums) {
            return search(nums, 0, nums.length - 1);
        }

        int search(int[] nums, int l, int r) {
            if (l == r)
                return l;
            int mid = (l + r) / 2;
            if (nums[mid] > nums[mid + 1])
                return search(nums, l, mid);
            return search(nums, mid + 1, r);
        }
    }

    static class Solution8 {
        public int findPeakElement(int[] nums) {
            int l = 0, r = nums.length - 1;
            while (l < r) {
                int mid = (l + r) / 2;
                if (nums[mid] > nums[mid + 1])
                    r = mid;
                else
                    l = mid + 1;
            }
            return l;
        }
    }

    // https://leetcode.com/problems/online-election/description/
    static class TopVotedCandidate {
        Map<Integer, Integer> m = new HashMap<>();
        int[] time;

        public TopVotedCandidate(int[] persons, int[] times) {
            int n = persons.length, lead = -1;
            Map<Integer, Integer> count = new HashMap<>();
            time = times;
            for (int i = 0; i < n; i++) {
                count.put(persons[i], count.getOrDefault(persons[i], 0) + 1);
                if (i == 0 || count.get(persons[i]) >= count.get(lead))
                    lead = persons[i];
                m.put(times[i], lead);
            }
        }

        public int q(int t) {
            int i = Arrays.binarySearch(time, t);
            return i < 0 ? m.get(time[-i - 2]) : m.get(time[i]);
        }
    }

    /**
     * Your TopVotedCandidate object will be instantiated and called as such:
     * TopVotedCandidate obj = new TopVotedCandidate(persons, times);
     * int param_1 = obj.q(t);
     */
}
