package leetcode_grind;

public class Day620 {

    // https://leetcode.com/problems/count-number-of-teams/description/?envType=daily-question&envId=2024-07-29
    static class Solution1 {
        public int numTeams(int[] rating) {
            int n = rating.length;
            int teams = 0;

            Integer[][] increasingCache = new Integer[n][4];
            Integer[][] decreasingCache = new Integer[n][4];

            for (int startIndex = 0; startIndex < n; startIndex++) {
                teams += countIncreasingTeams(rating, startIndex, 1, increasingCache) +
                        countDecreasingTeams(rating, startIndex, 1, decreasingCache);
            }

            return teams;
        }

        int countIncreasingTeams(int[] rating, int currentIndex, int teamSize, Integer[][] increasingCache) {
            int n = rating.length;

            if (currentIndex == n)
                return 0;

            if (teamSize == 3)
                return 1;

            if (increasingCache[currentIndex][teamSize] != null) {
                return increasingCache[currentIndex][teamSize];
            }

            int validTeams = 0;

            for (int nextIndex = currentIndex + 1; nextIndex < n; nextIndex++) {
                if (rating[nextIndex] > rating[currentIndex]) {
                    validTeams += countIncreasingTeams(rating, nextIndex, teamSize + 1, increasingCache);
                }
            }

            return increasingCache[currentIndex][teamSize] = validTeams;
        }

        int countDecreasingTeams(int[] rating, int currentIndex, int teamSize, Integer[][] decreasingCache) {
            int n = rating.length;

            if (currentIndex == n)
                return 0;

            if (teamSize == 3)
                return 1;

            if (decreasingCache[currentIndex][teamSize] != null) {
                return decreasingCache[currentIndex][teamSize];
            }

            int validTeams = 0;

            for (int nextIndex = currentIndex + 1; nextIndex < n; nextIndex++) {
                if (rating[nextIndex] < rating[currentIndex]) {
                    validTeams += countDecreasingTeams(rating, nextIndex, teamSize + 1, decreasingCache);
                }
            }

            return decreasingCache[currentIndex][teamSize] = validTeams;
        }
    }

    static class Solution2 {
        public int numTeams(int[] rating) {
            int n = rating.length;
            int teams = 0;

            for (int mid = 0; mid < n; mid++) {
                int leftSmaller = 0;
                int rightLarger = 0;

                for (int left = mid - 1; left >= 0; left--) {
                    if (rating[left] < rating[mid]) {
                        leftSmaller++;
                    }
                }

                for (int right = mid + 1; right < n; right++) {
                    if (rating[right] > rating[mid]) {
                        rightLarger++;
                    }
                }

                teams += leftSmaller * rightLarger;

                int leftLarger = mid - leftSmaller;
                int rightSmaller = n - mid - 1 - rightLarger;

                teams += leftLarger * rightSmaller;
            }

            return teams;
        }
    }
}
