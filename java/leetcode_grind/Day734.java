package leetcode_grind;

public class Day734 {
    // https://leetcode.com/problems/take-k-of-each-character-from-left-and-right/description/?envType=daily-question&envId=2024-11-20
    static class Solution1 {
        int minMinutes = Integer.MAX_VALUE;

        public int takeCharacters(String s, int k) {
            if (k == 0)
                return 0;
            int[] count = new int[3];
            solve(s, k, 0, s.length() - 1, count, 0);
            return minMinutes == Integer.MAX_VALUE ? -1 : minMinutes;
        }

        void solve(
                String s,
                int k,
                int left,
                int right,
                int[] count,
                int minutes) {
            if (count[0] >= k && count[1] >= k && count[2] >= k) {
                minMinutes = Math.min(minMinutes, minutes);
                return;
            }

            if (left > right)
                return;

            int[] leftCount = count.clone();
            leftCount[s.charAt(left) - 'a']++;
            solve(s, k, left + 1, right, leftCount, minutes + 1);

            int[] rightCount = count.clone();
            rightCount[s.charAt(right) - 'a']++;
            solve(s, k, left, right - 1, rightCount, minutes + 1);

        }
    }

    static class Solution2 {
        public int takeCharacters(String s, int k) {
            int[] count = new int[3];
            int n = s.length();

            for (char c : s.toCharArray()) {
                count[c - 'a']++;
            }

            for (int i = 0; i < 3; i++) {
                if (count[i] < k)
                    return -1;
            }

            int[] window = new int[3];
            int left = 0, maxWindow = 0;

            for (int right = 0; right < n; right++) {
                window[s.charAt(right) - 'a']++;
                while (left <= right
                        && (count[0] - window[0] < k || count[1] - window[1] < k || count[2] - window[2] < k)) {
                    window[s.charAt(left) - 'a']--;
                    left++;
                }
                maxWindow = Math.max(maxWindow, right - left + 1);
            }

            return n - maxWindow;
        }
    }
}
