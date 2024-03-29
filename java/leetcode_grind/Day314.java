package leetcode_grind;
import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.function.Function;
import java.util.stream.Collectors;

public class Day314 {
    // https://leetcode.com/problems/median-of-a-row-wise-sorted-matrix/description/
    static class Solution1 {
        public int matrixMedian(int[][] grid) {
            var binSearch = new Object() {
                int apply(int[] cols, int target) {
                    var left = 0;
                    var right = cols.length - 1;

                    while (left <= right) {
                        var mid = (left + right) / 2;
                        if (cols[mid] >= target) {
                            right = mid - 1;
                        } else {
                            left = mid + 1;
                        }
                    }
                    return left;
                }
            };

            var m = grid.length;
            var n = grid[0].length;

            var half = (m * n) / 2;
            var left = 1;
            var right = (int) 10e6;

            while (left + 1 < right) {
                var mid = (left + right) / 2;

                var sum = 0;
                for (var col : grid) {
                    sum += binSearch.apply(col, mid);
                }

                if (sum > half) {
                    right = mid;
                } else {
                    left = mid;
                }
            }

            return left;
        }
    }

    // https://leetcode.com/problems/is-subsequence/description
    static class Solution2 {
        public boolean isSubsequence(String s, String t) {
            var indices = new HashMap<Character, List<Integer>>();
            var i = 0;
            for (char c : t.toCharArray()) {
                indices.putIfAbsent(c, new ArrayList<>());
                indices.get(c).add(i++);
            }

            var currMatchIndex = -1;
            for (char c : s.toCharArray()) {
                if (!indices.containsKey(c)) {
                    return false;
                }

                var isMatched = false;
                for (var matchIndex : indices.get(c)) {
                    if (currMatchIndex < matchIndex) {
                        currMatchIndex = matchIndex;
                        isMatched = true;
                        break;
                    }
                }

                if (!isMatched) {
                    return false;
                }
            }

            return true;
        }
    }

    // https://leetcode.com/problems/number-of-matching-subsequences/description/
    static class Solution3 {
        public int numMatchingSubseqHashMapTLE(String s, String[] words) {
            var indices = new HashMap<Character, List<Integer>>();
            var i = 0;
            for (var ch : s.toCharArray()) {
                indices.putIfAbsent(ch, new ArrayList<>());
                indices.get(ch).add(i);
                i++;
            }

            Function<String, Boolean> check = (str) -> {
                var last = -1;
                for (var ch : str.toCharArray()) {
                    if (!indices.containsKey(ch)) {
                        return false;
                    }
                    var found = false;

                    for (var idx : indices.get(ch)) {
                        if (idx > last) {
                            last = idx;
                            found = true;
                            break;
                        }
                    }

                    if (!found) {
                        return false;
                    }
                }

                return true;
            };

            var ans = 0;
            for (var w : words) {
                if (check.apply(w)) {
                    ans++;
                }
            }
            return ans;
        }

        public int numMatchingSubseqBucketsApproach(String s, String[] words) {
            class Node {
                String word;
                int index;

                Node(String w, int i) {
                    word = w;
                    index = i;
                }
            }

            var ans = 0;
            @SuppressWarnings("unchecked")
            ArrayList<Node>[] heads = new ArrayList[26];
            for (var i = 0; i < 26; ++i) {
                heads[i] = new ArrayList<Node>();
            }

            for (var word : words) {
                heads[word.charAt(0) - 'a'].add(new Node(word, 0));
            }

            for (var c : s.toCharArray()) {
                var oldBucket = heads[c - 'a'];
                heads[c - 'a'] = new ArrayList<>();

                for (var node : oldBucket) {
                    node.index++;
                    if (node.index == node.word.length()) {
                        ans++;
                    } else {
                        heads[node.word.charAt(node.index) - 'a'].add(node);
                    }
                }
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/shortest-way-to-form-string/description/
    static class Solution4 {
        public int shortestWay(String source, String target) {
            var invertedIndex = new HashMap<Character, List<Integer>>();
            var i = 0;
            for (var ch : source.toCharArray()) {
                invertedIndex.putIfAbsent(ch, new ArrayList<>());
                invertedIndex.get(ch).add(i);
                i++;
            }

            for (var ch : target.toCharArray()) {
                if (!invertedIndex.containsKey(ch)) {
                    return -1;
                }
            }

            var sourceIterator = 0;
            var count = 1;

            for (var ch : target.toCharArray()) {
                var indices = invertedIndex.get(ch);
                var index = Collections.binarySearch(indices, sourceIterator);

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
}
