package leetcode_grind;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;

public class Day527 {
    // https://leetcode.com/problems/open-the-lock/description
    static class Solution1 {
        List<String> next(String v) {
            var ans = new ArrayList<String>();
            for (int i = 0; i < v.length(); i++) {
                var cur = (int) (v.charAt(i) - '0');
                for (int j = -1; j <= 1; j++) {
                    var nxt = (char) (((cur + j + 10) % 10) + '0');
                    ans.add(
                            v.substring(0, i) + nxt + v.substring(i + 1));
                }
            }
            return ans;
        }

        public int openLock(String[] deadends, String target) {
            var seen = new HashSet<String>();
            for (var d : deadends)
                seen.add(d);
            var queue = new LinkedList<String>();
            queue.push("0000");
            if (!seen.add("0000")) {
                return -1;
            }
            int steps = 0;
            while (!queue.isEmpty()) {
                var sz = queue.size();
                while (sz-- > 0) {
                    var v = queue.poll();

                    if (v.equals(target)) {
                        return steps;
                    }

                    for (var u : next(v)) {
                        if (!seen.contains(u)) {
                            seen.add(u);
                            queue.add(u);
                        }
                    }
                }

                steps++;
            }
            return -1;
        }
    }
}
