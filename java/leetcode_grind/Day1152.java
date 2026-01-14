package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeSet;

public class Day1152 {
    static class SegmentTree {

        private int[] count;
        private int[] covered;
        private int[] xs;
        private int n;

        public SegmentTree(int[] xs_) {
            xs = xs_;
            n = xs.length - 1;
            count = new int[4 * n];
            covered = new int[4 * n];
        }

        private void modify(
                int qleft,
                int qright,
                int qval,
                int left,
                int right,
                int pos) {
            if (xs[right + 1] <= qleft || xs[left] >= qright) {
                return;
            }
            if (qleft <= xs[left] && xs[right + 1] <= qright) {
                count[pos] += qval;
            } else {
                int mid = (left + right) / 2;
                modify(qleft, qright, qval, left, mid, pos * 2 + 1);
                modify(qleft, qright, qval, mid + 1, right, pos * 2 + 2);
            }

            if (count[pos] > 0) {
                covered[pos] = xs[right + 1] - xs[left];
            } else {
                if (left == right) {
                    covered[pos] = 0;
                } else {
                    covered[pos] = covered[pos * 2 + 1] + covered[pos * 2 + 2];
                }
            }
        }

        public void update(int qleft, int qright, int qval) {
            modify(qleft, qright, qval, 0, n - 1, 0);
        }

        public int query() {
            return covered[0];
        }
    }

    static class Solution1 {

        public double separateSquares(int[][] squares) {
            // save events: (y-coordinate, type, left boundary, right boundary)
            List<int[]> events = new ArrayList<>();
            Set<Integer> xsSet = new TreeSet<>();

            for (int[] sq : squares) {
                int x = sq[0];
                int y = sq[1];
                int l = sq[2];
                int xr = x + l;
                events.add(new int[] { y, 1, x, xr });
                events.add(new int[] { y + l, -1, x, xr });
                xsSet.add(x);
                xsSet.add(xr);
            }

            // sort events by y-coordinate
            events.sort((a, b) -> Integer.compare(a[0], b[0]));
            // discrete coordinates
            int[] xs = xsSet.stream().mapToInt(i -> i).toArray();
            // initialize the segment tree
            SegmentTree segTree = new SegmentTree(xs);

            List<Long> psum = new ArrayList<>();
            List<Integer> widths = new ArrayList<>();
            Long totalArea = 0L;
            int prev = events.get(0)[0];

            // scan: calculate total area and record intermediate states
            for (int[] event : events) {
                int y = event[0];
                int delta = event[1];
                int xl = event[2];
                int xr = event[3];
                int len = segTree.query();
                totalArea += (long) len * (y - prev);
                segTree.update(xl, xr, delta);
                // record prefix sums and widths
                psum.add(totalArea);
                widths.add(segTree.query());
                prev = y;
            }

            // calculate the target area (half rounded up)
            long target = (long) (totalArea + 1) / 2;
            // binary search
            int i = binarySearch(psum, target);
            double area = psum.get(i);
            // get the corresponding area, width, and height
            int width = widths.get(i);
            int height = events.get(i)[0];

            return height + (totalArea - area * 2) / (width * 2.0);
        }

        private int binarySearch(List<Long> list, long target) {
            int left = 0;
            int right = list.size() - 1;
            int result = 0;

            while (left <= right) {
                int mid = left + (right - left) / 2;
                if (list.get(mid) < target) {
                    result = mid;
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            return result;
        }
    }

    // https://leetcode.com/problems/rectangle-area-ii/description/
    static class Solution2 {
        public int rectangleArea(int[][] rectangles) {
            int N = rectangles.length;
            Set<Integer> Xvals = new HashSet<>();
            Set<Integer> Yvals = new HashSet<>();

            for (int[] rec : rectangles) {
                Xvals.add(rec[0]);
                Xvals.add(rec[2]);
                Yvals.add(rec[1]);
                Yvals.add(rec[3]);
            }

            Integer[] imapx = Xvals.toArray(new Integer[0]);
            Arrays.sort(imapx);
            Integer[] imapy = Yvals.toArray(new Integer[0]);
            Arrays.sort(imapy);

            Map<Integer, Integer> mapx = new HashMap<>();
            Map<Integer, Integer> mapy = new HashMap<>();

            for (int i = 0; i < imapx.length; i++) {
                mapx.put(imapx[i], i);
            }
            for (int i = 0; i < imapy.length; i++) {
                mapy.put(imapy[i], i);
            }

            boolean[][] grid = new boolean[imapx.length][imapy.length];
            for (int[] rec : rectangles) {
                for (int x = mapx.get(rec[0]); x < mapx.get(rec[2]); ++x) {
                    for (int y = mapy.get(rec[1]); y < mapy.get(rec[3]); ++y) {
                        grid[x][y] = true;
                    }
                }
            }

            long ans = 0;
            for (int x = 0; x < grid.length; x++) {
                for (int y = 0; y < grid[0].length; y++) {
                    if (grid[x][y]) {
                        ans += (long) (imapx[x + 1] - imapx[x]) * (imapy[y + 1] - imapy[y]);
                    }
                }
            }
            ans %= 1_000_000_007;
            return (int) ans;
        }
    }

}
