package leetcode_grind;

import java.util.Stack;

public class Day621 {
    // https://leetcode.com/problems/minimum-deletions-to-make-string-balanced/description/?envType=daily-question&envId=2024-07-30
    static class Solution1 {
        public int minimumDeletions(String s) {
            var st = new Stack<Character>();
            for (int i = 0; i < s.length(); i++) {
                while (i < s.length() && !st.isEmpty() && st.peek() > s.charAt(i)) {
                    st.pop();
                    i++;
                }
                if (i < s.length())
                    st.push(s.charAt(i));
            }
            return (s.length() - st.size()) / 2;
        }
    }

    static class Solution2 {
        public int minimumDeletions(String s) {
            int n = s.length();
            int[] countA = new int[n];
            int aCount = 0;

            for (int i = n - 1; i >= 0; i--) {
                countA[i] = aCount;
                if (s.charAt(i) == 'a')
                    aCount++;
            }

            int minimumDeletions = n;
            int bCount = 0;

            for (int i = 0; i < n; i++) {
                minimumDeletions = Math.min(countA[i] + bCount, minimumDeletions);
                if (s.charAt(i) == 'b')
                    bCount++;
            }

            return minimumDeletions;
        }
    }
}
