package leetcode_grind;

import java.util.List;

public class Day430 {
    // https://leetcode.com/problems/valid-word-square/
    static class Solution1 {
        public boolean validWordSquare(List<String> words) {
            for (int wordNum = 0; wordNum < words.size(); ++wordNum) {
                for (int charPos = 0; charPos < words.get(wordNum).length(); ++charPos) {
                    if (charPos >= words.size() ||
                            wordNum >= words.get(charPos).length() ||
                            words.get(wordNum).charAt(charPos) != words.get(charPos).charAt(wordNum)) {
                        return false;
                    }
                }
            }
            return true;
        }
    }

    // https://leetcode.com/problems/minimum-time-difference/description/
    static class Solution2 {
        // public int findMinDifference(List<String> timePoints) {
        // List<Integer> minutes = timePoints.stream().flatMap(p -> {
        // var s = p.split(":");
        // var h = Integer.parseInt(s[0]);
        // var m = Integer.parseInt(s[1]);

        // if (h < 12) {
        // return Stream.of(24 * 60 + h * 60 + m, h * 60 + m);
        // }

        // return Stream.of(h * 60 + m);
        // }).collect(Collectors.toList());

        // Collections.sort(minutes);
        // var diff = Integer.MAX_VALUE;
        // for (int i = 0; i < minutes.size(); i++) {
        // var a = minutes.get(i);
        // var b = minutes.get((i - 1 + minutes.size()) % minutes.size());
        // var res = Math.min(Math.abs(a - b), Math.abs(b - a));
        // diff = Math.min(diff, res);
        // }
        // return diff;
        // }
    }
}
