package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day1013 {
    // https://leetcode.com/problems/sort-matrix-by-diagonals/description/?envType=daily-question&envId=2025-08-28
    static class Solution1 {
        public int[][] sortMatrix(int[][] grid) {
            int n = grid.length;
            for (int i = 0; i < n; i++) {
                List<Integer> tmp = new ArrayList<>();
                for (int j = 0; i + j < n; j++) {
                    tmp.add(grid[i + j][j]);
                }
                tmp.sort(Collections.reverseOrder());
                for (int j = 0; i + j < n; j++) {
                    grid[i + j][j] = tmp.get(j);
                }
            }

            for (int j = 1; j < n; j++) {
                List<Integer> tmp = new ArrayList<>();
                for (int i = 0; j + i < n; i++) {
                    tmp.add(grid[i][j + i]);
                }
                Collections.sort(tmp);
                for (int i = 0; j + i < n; i++) {
                    grid[i][j + i] = tmp.get(i);
                }
            }

            return grid;
        }
    }

    // https://leetcode.com/problems/word-search-ii/description/
    static class Solution2 {
        static class TrieNode {
            Map<Character, TrieNode> children = new HashMap<>();
            String word = null;
        }

        char[][] _board = null;
        List<String> _result = new ArrayList<>();

        public List<String> findWords(char[][] board, String[] words) {
            TrieNode root = new TrieNode();
            for (String word : words) {
                TrieNode node = root;

                for (Character letter : word.toCharArray()) {
                    if (node.children.containsKey(letter)) {
                        node = node.children.get(letter);
                    } else {
                        TrieNode newNode = new TrieNode();
                        node.children.put(letter, newNode);
                        node = newNode;
                    }
                }

                node.word = word;
            }

            this._board = board;
            for (int row = 0; row < board.length; row++) {
                for (int col = 0; col < board[row].length; ++col) {
                    if (root.children.containsKey(board[row][col])) {
                        backtracking(row, col, root);
                    }
                }
            }

            return this._result;
        }

        void backtracking(int row, int col, TrieNode parent) {
            Character letter = this._board[row][col];
            TrieNode currNode = parent.children.get(letter);

            if (currNode.word != null) {
                this._result.add(currNode.word);
                currNode.word = null;
            }

            this._board[row][col] = '#';

            int[] rowOffset = { -1, 0, 1, 0 };
            int[] colOffset = { 0, 1, 0, -1 };

            for (int i = 0; i < 4; i++) {
                int newRow = row + rowOffset[i];
                int newCol = col + colOffset[i];

                if (newRow < 0 || newRow >= this._board.length ||
                        newCol < 0 || newCol >= this._board[0].length) {
                    continue;
                }

                if (currNode.children.containsKey(this._board[newRow][newCol])) {
                    backtracking(newRow, newCol, currNode);
                }
            }

            this._board[row][col] = letter;

            if (currNode.children.isEmpty()) {
                parent.children.remove(letter);
            }
        }
    }
}
