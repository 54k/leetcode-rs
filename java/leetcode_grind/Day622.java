package leetcode_grind;

public class Day622 {
    // https://leetcode.com/problems/filling-bookcase-shelves/description/?envType=daily-question&envId=2024-07-31
    static class Solution1 {
        public int minHeightShelves(int[][] books, int shelfWidth) {
            int[][] memo = new int[books.length][shelfWidth + 1];
            return dpHelper(books, shelfWidth, memo, 0, shelfWidth, 0);
        }

        int dpHelper(int[][] books, int shelfWidth, int[][] memo, int i, int remainingShelfWidth, int maxHeight) {
            int[] currentBook = books[i];
            int maxHeightUpdated = Math.max(maxHeight, currentBook[1]);
            if (i == books.length - 1) {
                if (remainingShelfWidth >= currentBook[0]) {
                    return maxHeightUpdated;
                }
                return maxHeight + currentBook[1];
            }

            if (memo[i][remainingShelfWidth] != 0) {
                return memo[i][remainingShelfWidth];
            }

            int ans = maxHeight + dpHelper(
                    books, shelfWidth, memo, i + 1, shelfWidth - currentBook[0], currentBook[1]);

            if (remainingShelfWidth >= currentBook[0]) {
                ans = Math.min(ans, dpHelper(
                        books, shelfWidth, memo, i + 1, remainingShelfWidth - currentBook[0], maxHeightUpdated));
            }
            return memo[i][remainingShelfWidth] = ans;
        }
    }
}
