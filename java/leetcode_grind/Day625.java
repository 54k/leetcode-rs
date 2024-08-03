package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;

public class Day625 {
    // https://leetcode.com/problems/make-two-arrays-equal-by-reversing-subarrays/description/?envType=daily-question&envId=2024-08-03
    static class Solution1 {
        public boolean canBeEqual(int[] target, int[] arr) {
            Arrays.sort(target);
            Arrays.sort(arr);
            for (int i = 0; i < arr.length; i++) {
                if (target[i] != arr[i]) {
                    return false;
                }
            }
            return true;
        }
    }

    // https://leetcode.com/problems/longest-repeating-substring/description/?envType=weekly-question&envId=2024-08-01
    static class Solution2 {
        public int longestRepeatingSubstring(String s) {
            Set<String> seenSubstrings = new HashSet<>();
            int maxLength = s.length() - 1;
            for (int start = 0; start <= s.length(); start++) {
                int end = start;
                if (end + maxLength > s.length()) {
                    if (--maxLength == 0)
                        break;
                    start = -1;
                    seenSubstrings.clear();
                    continue;
                }

                String currentSubstring = s.substring(end, end + maxLength);
                if (!seenSubstrings.add(currentSubstring)) {
                    return maxLength;
                }
            }
            return maxLength;
        }
    }

    static class Solution3 {
        public int longestRepeatingSubstring(String s) {
            int length = s.length(), maxLength = 0;
            Set<String> seenSubstrings = new HashSet<>();

            for (int start = 0; start < length; start++) {
                int end = start;
                if (end + maxLength >= length) {
                    return maxLength;
                }
                String currentSubstring = s.substring(end, end + maxLength + 1);
                if (!seenSubstrings.add(currentSubstring)) {
                    start = -1;
                    seenSubstrings.clear();
                    maxLength++;
                }
            }
            return maxLength;
        }
    }

    static class Solution4 {
        public int longestRepeatingSubstring(String s) {
            char[] characters = s.toCharArray();
            int start = 1, end = characters.length - 1;

            while (start <= end) {
                int mid = (start + end) / 2;
                if (hasRepeatingSubstring(characters, mid)) {
                    start = mid + 1;
                } else {
                    end = mid - 1;
                }
            }
            return end;
        }

        boolean hasRepeatingSubstring(char[] characters, int length) {
            Set<String> seenSubstrings = new HashSet<>();
            for (int i = 0; i <= characters.length - length; i++) {
                String substring = new String(characters, i, length);
                if (!seenSubstrings.add(substring)) {
                    return true;
                }
            }
            return false;
        }
    }

    static class Solution5 {
        public int longestRepeatingSubstring(String s) {
            int length = s.length();
            String[] suffixes = new String[length];
            for (int i = 0; i < length; i++) {
                suffixes[i] = s.substring(i);
            }
            msdRadixSort(suffixes);
            int maxLength = 0;
            for (int i = 1; i < length; i++) {
                int j = 0;
                while (j < Math.min(suffixes[i].length(), suffixes[i - 1].length()) &&
                        suffixes[i].charAt(j) == suffixes[i - 1].charAt(j)) {
                    j++;
                }
                maxLength = Math.max(maxLength, j);
            }
            return maxLength;
        }

        void msdRadixSort(String[] input) {
            sort(input, 0, input.length - 1, 0, new String[input.length]);
        }

        void sort(String[] input, int lo, int hi, int depth, String[] aux) {
            if (lo >= hi)
                return;
            int[] count = new int[128];
            for (int i = lo; i <= hi; i++) {
                count[charAt(input[i], depth) + 1]++;
            }
            for (int i = 1; i < 28; i++) {
                count[i] += count[i - 1];
            }
            for (int i = lo; i <= hi; i++) {
                aux[count[charAt(input[i], depth)]++] = input[i];
            }
            for (int i = lo; i <= hi; i++) {
                input[i] = aux[i - lo];
            }
            for (int i = 0; i < 27; i++) {
                sort(input, lo + count[i], lo + count[i + 1] - 1, depth + 1, aux);
            }
        }

        int charAt(String s, int index) {
            if (index >= s.length())
                return 0;
            return s.charAt(index) - 'a' + 1;
        }
    }

    // https://leetcode.com/problems/word-search-ii/description/
    static class Solution6 {
        static class TrieNode {
            Map<Character, TrieNode> children = new HashMap<>();
            String word = null;
        }

        char[][] _board = null;
        ArrayList<String> _result = new ArrayList<String>();

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
            for (int row = 0; row < board.length; ++row) {
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
                if (newRow < 0 || newRow >= this._board.length || newCol < 0 || newCol >= this._board[0].length) {
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
