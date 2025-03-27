package data_structures_examples;

public class SuffixArrayMSD {
    // https://leetcode.com/problems/longest-repeating-substring/description/
    static class Solution {
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
                while (j < Math.min(suffixes[i].length(), suffixes[i - 1].length())
                        && suffixes[i].charAt(j) == suffixes[i - 1].charAt(j)) {
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

            int[] count = new int[28];

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

}
