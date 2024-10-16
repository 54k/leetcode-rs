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

    static class Solution2 {
        static class Pair {
            int count;
            char character;

            Pair(int count, char character) {
                this.count = count;
                this.character = character;
            }
        }

        public String longestDiverseString(int a, int b, int c) {
            var pq = new PriorityQueue<Pair>((x, y) -> (y.count - x.count));
            if (a > 0) {
                pq.add(new Pair(a, 'a'));
            }
            if (b > 0) {
                pq.add(new Pair(b, 'b'));
            }
            if (c > 0) {
                pq.add(new Pair(c, 'c'));
            }
            StringBuilder ans = new StringBuilder();
            while (!pq.isEmpty()) {
                Pair p = pq.poll();
                int count = p.count;
                char character = p.character;

                if (ans.length() >= 2 && ans.charAt(ans.length() - 1) == p.character
                        && ans.charAt(ans.length() - 2) == p.character) {
                    if (pq.isEmpty())
                        break;
                    Pair temp = pq.poll();
                    ans.append(temp.character);
                    if (temp.count - 1 > 0) {
                        pq.add(new Pair(temp.count - 1, temp.character));
                    }
                } else {
                    count--;
                    ans.append(character);
                }
                if (count > 0) {
                    pq.add(new Pair(count, character));
                }
            }
            return ans.toString();
        }
    }

    static class Solution3 {
        public String longestDiverseString(int a, int b, int c) {
            int curra = 0, currb = 0, currc = 0;
            int totalIterations = a + b + c;
            StringBuilder ans = new StringBuilder();
            for (int i = 0; i < totalIterations; i++) {
                if ((a >= b && a >= c && curra != 2) ||
                        (a > 0 && (currb == 2 || currc == 2))) {
                    ans.append('a');
                    a--;
                    curra++;
                    currb = 0;
                    currc = 0;
                } else if ((b >= a && b >= c && currb != 2) ||
                        (b > 0 && (currc == 2 || curra == 2))) {
                    ans.append('b');
                    b--;
                    currb++;
                    curra = 0;
                    currc = 0;
                } else if ((c >= a && c >= b && currc != 2) ||
                        (c > 0 && (curra == 2 || currb == 2))) {
                    ans.append('c');
                    c--;
                    currc++;
                    curra = 0;
                    currb = 0;
                }
            }
            return ans.toString();
        }
    }
}