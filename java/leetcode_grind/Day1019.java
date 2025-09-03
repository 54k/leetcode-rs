package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;

public class Day1019 {
    // https://leetcode.com/problems/find-the-number-of-ways-to-place-people-ii/description/?envType=daily-question&envId=2025-09-03
    static public class Solution1 {

        public int numberOfPairs(int[][] points) {
            int ans = 0;
            Arrays.sort(points, (a, b) -> a[0] != b[0] ? a[0] - b[0] : b[1] - a[1]);
            for (int i = 0; i < points.length - 1; i++) {
                int[] pointA = points[i];
                int xMin = pointA[0] - 1;
                int xMax = Integer.MAX_VALUE;
                int yMin = Integer.MIN_VALUE;
                int yMax = pointA[1] + 1;

                for (int j = i + 1; j < points.length; j++) {
                    int[] pointB = points[j];
                    if (pointB[0] > xMin &&
                            pointB[0] < xMax &&
                            pointB[1] > yMin &&
                            pointB[1] < yMax) {
                        ans++;
                        xMin = pointB[0];
                        yMin = pointB[1];
                    }
                }
            }
            return ans;
        }
    }

    static class Solution2 {
        static class Point {
            int x, y;

            Point(int x, int y) {
                this.x = x;
                this.y = y;
            }

            @Override
            public boolean equals(Object o) {
                if (this == o) {
                    return true;
                }
                if (o == null || getClass() != o.getClass()) {
                    return false;
                }
                Point point = (Point) o;
                return x == point.x && y == point.y;
            }

            @Override
            public int hashCode() {
                return Objects.hash(x, y);
            }
        }

        public int numberOfPairs(int[][] points) {
            Map<Integer, Integer> col = new HashMap<>();
            Map<Integer, Integer> row = new HashMap<>();
            Map<Point, int[]> coordinatesMap = new HashMap<>();
            for (int[] point : points) {
                int x = point[0];
                int y = point[1];
                col.put(x, 0);
                row.put(y, 0);
            }

            List<Integer> rowKeys = new ArrayList<>(row.keySet());
            Collections.sort(rowKeys);
            for (int i = 0; i < rowKeys.size(); i++) {
                row.put(rowKeys.get(i), i + 1);
            }

            List<Integer> colKeys = new ArrayList<>(col.keySet());
            Collections.sort(colKeys);
            for (int i = 0; i < colKeys.size(); i++) {
                col.put(colKeys.get(i), i + 1);
            }

            int nc = col.size() + 1;
            int nr = row.size() + 1;
            int[][] m = new int[nc][nr];
            int[][] prefixSum = new int[nc][nr];

            for (int[] point : points) {
                int x = point[0];
                int y = point[1];
                int c = col.get(x);
                int r = row.get(y);

                Point key = new Point(x, y);
                coordinatesMap.put(key, new int[] { c, r });
                m[c][r] = 1;
            }
            for (int i = 1; i < nc; i++) {
                for (int j = 1; j < nr; j++) {
                    prefixSum[i][j] = prefixSum[i - 1][j] +
                            prefixSum[i][j - 1] -
                            prefixSum[i - 1][j - 1] +
                            m[i][j];
                }
            }

            int ans = 0;
            Arrays.sort(points, (a, b) -> {
                if (a[0] == b[0]) {
                    return Integer.compare(b[1], a[1]);
                }
                return Integer.compare(a[0], b[0]);
            });

            int n = points.length;
            for (int i = 0; i < n - 1; i++) {
                for (int j = i + 1; j < n; j++) {
                    if (points[i][1] >= points[j][1]) {
                        Point key1 = new Point(points[i][0], points[i][1]);
                        Point key2 = new Point(points[j][0], points[j][1]);
                        int[] coord1 = coordinatesMap.get(key1);
                        int[] coord2 = coordinatesMap.get(key2);
                        int c1 = coord1[0];
                        int c2 = coord2[0];
                        int r1 = coord1[1];
                        int r2 = coord2[1];
                        int cnt = prefixSum[c2][r1] -
                                prefixSum[c1 - 1][r1] -
                                prefixSum[c2][r2 - 1] +
                                prefixSum[c1 - 1][r2 - 1];
                        if (cnt == 2) {
                            ans++;
                        }
                    }
                }
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/letter-combinations-of-a-phone-number/description/
    static class Solution3 {
        private List<String> combinations = new ArrayList<>();
        private Map<Character, String> letters = Map.of(
                '2', "abc",
                '3', "def",
                '4', "ghi",
                '5', "jkl",
                '6', "mno",
                '7', "pqrs",
                '8', "tuv",
                '9', "wxyz");
        private String phoneDigits;

        public List<String> letterCombinations(String digits) {
            if (digits.length() == 0) {
                return combinations;
            }

            phoneDigits = digits;
            backtrack(0, new StringBuilder());
            return combinations;
        }

        void backtrack(int index, StringBuilder path) {
            if (path.length() == phoneDigits.length()) {
                combinations.add(path.toString());
                return;
            }

            String possibleLetters = letters.get(phoneDigits.charAt(index));
            for (char letter : possibleLetters.toCharArray()) {
                path.append(letter);
                backtrack(index + 1, path);
                path.deleteCharAt(path.length() - 1);
            }
        }
    }

    // https://leetcode.com/problems/generate-parentheses/description/
    static class Solution4 {
        boolean isValid(String pString) {
            int leftCount = 0;
            for (char p : pString.toCharArray()) {
                if (p == '(') {
                    leftCount++;
                } else {
                    leftCount--;
                }

                if (leftCount < 0) {
                    return false;
                }
            }
            return leftCount == 0;
        }

        public List<String> generateParenthesis(int n) {
            List<String> answer = new ArrayList<>();
            Queue<String> queue = new LinkedList<>(Arrays.asList(""));

            while (!queue.isEmpty()) {
                String curString = queue.poll();
                if (curString.length() == 2 * n) {
                    if (isValid(curString)) {
                        answer.add(curString);
                    }
                    continue;
                }
                queue.offer(curString + ")");
                queue.offer(curString + "(");
            }

            return answer;
        }
    }
}
