package leetcode_grind;

public class Day1151 {
    // https://leetcode.com/problems/separate-squares-i/description/?envType=daily-question&envId=2026-01-13
    static class Solution1 {
        public double separateSquares(int[][] squares) {
            double max_y = 0;
            double total_area = 0;

            for (int[] sq : squares) {
                int y = sq[1];
                int l = sq[2];
                total_area += (double) l * l;
                max_y = Math.max(max_y, (double) (y + l));
            }

            double lo = 0;
            double hi = max_y;
            double eps = 1e-5;

            while (Math.abs(hi - lo) > eps) {
                double mid = (hi + lo) / 2;
                if (check(mid, squares, total_area)) {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }

            return hi;
        }

        boolean check(double limit_y, int[][] squares, double total_area) {
            double area = 0;
            for (int[] sq : squares) {
                int y = sq[1];
                int l = sq[2];
                if (y < limit_y) {
                    area += (double) l * Math.min(limit_y - y, (double) l);
                }
            }
            return area >= total_area / 2;
        }
    }

}
