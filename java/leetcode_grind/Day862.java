package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;

public class Day862 {
    // https://leetcode.com/problems/partition-labels/submissions/1590963183/
    static class Solution1 {
        public List<Integer> partitionLabels(String s) {
            List<Integer> partitionSizes = new ArrayList<>();
            int[] lastOccurrence = new int[26];
            int[] firstOccurrence = new int[26];
            Arrays.fill(firstOccurrence, -1);

            int partitionStart = 0, partitionEnd = 0;

            for (int i = 0; i < s.length(); i++) {
                lastOccurrence[s.charAt(i) - 'a'] = i;
            }

            for (int i = 0; i < s.length(); i++) {
                if (firstOccurrence[s.charAt(i) - 'a'] == -1) {
                    firstOccurrence[s.charAt(i) - 'a'] = i;
                }

                if (partitionEnd < firstOccurrence[s.charAt(i) - 'a']) {
                    partitionSizes.add(partitionEnd - partitionStart + 1);
                    partitionStart = i;
                    partitionEnd = i;
                }

                partitionEnd = Math.max(partitionEnd, lastOccurrence[s.charAt(i) - 'a']);
            }

            if (partitionEnd - partitionStart + 1 > 0) {
                partitionSizes.add(partitionEnd - partitionStart + 1);
            }

            return partitionSizes;
        }
    }

    static class Solution2 {
        public List<Integer> partitionLabels(String s) {
            int[] lastOccurrence = new int[26];
            for (int i = 0; i < s.length(); i++) {
                lastOccurrence[s.charAt(i) - 'a'] = i;
            }
            int partitionEnd = 0, partitionStart = 0;
            List<Integer> partitionSizes = new ArrayList<>();
            for (int i = 0; i < s.length(); ++i) {
                partitionEnd = Math.max(
                        partitionEnd,
                        lastOccurrence[s.charAt(i) - 'a']);
                if (i == partitionEnd) {
                    partitionSizes.add(i - partitionStart + 1);
                    partitionStart = i + 1;
                }
            }
            return partitionSizes;
        }
    }

    // https://leetcode.com/problems/shortest-way-to-form-string/description/
    static class Solution3 {
        public int shortestWay(String source, String target) {
            boolean[] sourceChars = new boolean[26];
            for (char c : source.toCharArray()) {
                sourceChars[c - 'a'] = true;
            }

            for (char c : target.toCharArray()) {
                if (!sourceChars[c - 'a']) {
                    return -1;
                }
            }

            String concatenatedSource = source;
            int count = 1;
            while (!isSubsequence(target, concatenatedSource)) {
                concatenatedSource += source;
                count++;
            }

            return count;
        }

        boolean isSubsequence(String toCheck, String inString) {
            int i = 0, j = 0;
            while (i < toCheck.length() && j < inString.length()) {
                if (toCheck.charAt(i) == inString.charAt(j)) {
                    i++;
                }
                j++;
            }
            return i == toCheck.length();
        }
    }

    static class Solution4 {
        public int shortestWay(String source, String target) {
            boolean[] sourceChars = new boolean[26];
            for (char c : source.toCharArray()) {
                sourceChars[c - 'a'] = true;
            }

            for (char c : target.toCharArray()) {
                if (!sourceChars[c - 'a']) {
                    return -1;
                }
            }

            int m = source.length();
            int sourceIterator = 0;
            int count = 0;

            for (char c : target.toCharArray()) {
                if (sourceIterator == 0) {
                    count++;
                }

                while (source.charAt(sourceIterator) != c) {
                    sourceIterator = (sourceIterator + 1) % m;
                    if (sourceIterator == 0) {
                        count++;
                    }
                }

                sourceIterator = (sourceIterator + 1) % m;
            }

            return count;
        }
    }

    static class Solution5 {
        public int shortestWay(String source, String target) {
            ArrayList<Integer>[] charToIndices = new ArrayList[26];
            for (int i = 0; i < source.length(); i++) {
                char c = source.charAt(i);
                if (charToIndices[c - 'a'] == null) {
                    charToIndices[c - 'a'] = new ArrayList<>();
                }
                charToIndices[c - 'a'].add(i);
            }

            int sourceIterator = 0;
            int count = 1;

            for (char c : target.toCharArray()) {
                if (charToIndices[c - 'a'] == null) {
                    return -1;
                }

                ArrayList<Integer> indices = charToIndices[c - 'a'];
                int index = Collections.binarySearch(indices, sourceIterator);

                if (index < 0) {
                    index = -index - 1;
                }

                if (index == indices.size()) {
                    count++;
                    sourceIterator = indices.get(0) + 1;
                } else {
                    sourceIterator = indices.get(index) + 1;
                }
            }

            return count;
        }
    }

    static class Solution6 {
        public int shortestWay(String source, String target) {
            int[][] nextOccurence = new int[source.length()][26];
            for (int c = 0; c < 26; c++) {
                nextOccurence[source.length() - 1][c] = -1;
            }
            nextOccurence[source.length() - 1][source.charAt(source.length() - 1) - 'a'] = source.length() - 1;

            for (int idx = source.length() - 2; idx >= 0; idx--) {
                for (int c = 0; c < 26; c++) {
                    nextOccurence[idx][c] = nextOccurence[idx + 1][c];
                }
                nextOccurence[idx][source.charAt(idx) - 'a'] = idx;
            }

            int sourceIterator = 0;
            int count = 1;

            for (char c : target.toCharArray()) {
                if (nextOccurence[0][c - 'a'] == -1) {
                    return -1;
                }

                if (sourceIterator == source.length() || nextOccurence[sourceIterator][c - 'a'] == -1) {
                    count++;
                    sourceIterator = 0;
                }

                sourceIterator = nextOccurence[sourceIterator][c - 'a'] + 1;
            }
            return count;
        }
    }
}
