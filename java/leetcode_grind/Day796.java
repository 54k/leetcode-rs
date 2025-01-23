package leetcode_grind;

public class Day796 {
    // https://leetcode.com/problems/count-servers-that-communicate/description/?envType=daily-question&envId=2025-01-23
    static class Solution1 {
        public int countServers(int[][] grid) {
            int numRows = grid.length;
            int numCols = numRows > 0 ? grid[0].length : 0;
            int communicableServersCount = 0;

            for (int row = 0; row < numRows; ++row) {
                for (int col = 0; col < numCols; ++col) {
                    if (grid[row][col] == 1) {
                        boolean canCommunicate = false;

                        for (int otherCol = 0; otherCol < numCols; ++otherCol) {
                            if (otherCol != col && grid[row][otherCol] == 1) {
                                canCommunicate = true;
                                break;
                            }
                        }

                        if (canCommunicate) {
                            communicableServersCount++;
                        } else {
                            for (int otherRow = 0; otherRow < numRows; ++otherRow) {
                                if (otherRow != row && grid[otherRow][col] == 1) {
                                    canCommunicate = true;
                                    break;
                                }
                            }

                            if (canCommunicate) {
                                communicableServersCount++;
                            }
                        }
                    }
                }
            }
            return communicableServersCount;
        }
    }

    static class Solution2 {
        public int countServers(int[][] grid) {
            int communicableServersCount = 0;
            int[] rowCounts = new int[grid[0].length];
            int[] lastServerInCol = new int[grid.length];
            for (int i = 0; i < lastServerInCol.length; i++) {
                lastServerInCol[i] = -1;
            }

            for (int row = 0; row < grid.length; row++) {
                int serverCountInRow = 0;
                for (int col = 0; col < grid[0].length; col++) {
                    if (grid[row][col] == 1) {
                        serverCountInRow++;
                        rowCounts[col]++;
                        lastServerInCol[row] = col;
                    }
                }

                if (serverCountInRow != 1) {
                    communicableServersCount += serverCountInRow;
                    lastServerInCol[row] = -1;
                }
            }

            for (int row = 0; row < grid.length; row++) {
                if (lastServerInCol[row] != -1 && rowCounts[lastServerInCol[row]] > 1) {
                    communicableServersCount++;
                }
            }

            return communicableServersCount;
        }
    }
}
