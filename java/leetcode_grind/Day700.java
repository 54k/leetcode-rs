package leetcode_grind;

import java.util.PriorityQueue;

public class Day700 {
    // https://leetcode.com/problems/longest-happy-string/description/?envType=daily-question&envId=2024-10-16
    static class Solution1 {
        public String longestDiverseString(int a, int b, int c) {
            var sb = new StringBuilder();
            var pq = new PriorityQueue<int[]>((x, y) -> y[0] - x[0]);

            pq.add(new int[] { a, 'a' });
            pq.add(new int[] { b, 'b' });
            pq.add(new int[] { c, 'c' });
            int[] f;
            int[] s;

            while (true) {
                f = pq.poll();
                s = pq.poll();
                if (sb.length() == 0 || (sb.length() > 0 && sb.charAt(sb.length() - 1) != f[1])) {
                    for (var i = 0; i < f[0] && i < 2; i++) {
                        sb.append((char) f[1]);
                        f[0]--;
                    }
                }

                if (s[0] == 0) {
                    break;
                }

                sb.append((char) s[1]);
                s[0]--;
                pq.add(f);
                pq.add(s);
            }

            return sb.toString();
        }
    }
}