package leetcode_grind;

import java.util.*;

public class Day746 {
    // https://leetcode.com/problems/adding-spaces-to-a-string/description/
    static class Solution1 {
        public String addSpaces(String s, int[] spaces) {
            var sb = new StringBuilder();
            for (int i = 0, j = 0; i < s.length(); i++) {
                if (j < spaces.length && i == spaces[j]) {
                    sb.append(" ");
                    j++;
                }
                sb.append(s.charAt(i));
            }
            return sb.toString();
        }
    }

    // https://leetcode.com/problems/find-the-celebrity/

    // /*
    // * The knows API is defined in the parent class Relation.
    // * boolean knows(int a, int b);
    // */

    // public class Solution extends Relation {
    // int numberOfPeople;
    // Map<Pair<Integer, Integer>, Boolean> cache = new HashMap<>();

    // @Override
    // public boolean knows(int a, int b) {
    // if (!cache.containsKey(new Pair(a, b))) {
    // cache.put(new Pair(a, b), super.knows(a, b));
    // }
    // return cache.get(new Pair(a, b));
    // }

    // public int findCelebrity(int n) {
    // numberOfPeople = n;
    // int celebrityCandidate = 0;
    // for (int i = 0; i < n; i++) {
    // if (knows(celebrityCandidate, i)) {
    // celebrityCandidate = i;
    // }
    // }
    // if (isCelebrity(celebrityCandidate)) {
    // return celebrityCandidate;
    // }
    // return -1;
    // }

    // boolean isCelebrity(int i) {
    // for (int j = 0; j < numberOfPeople; j++) {
    // if (i == j)
    // continue;
    // if (knows(i, j) || !knows(j, i)) {
    // return false;
    // }
    // }
    // return true;
    // }
    // }

}
