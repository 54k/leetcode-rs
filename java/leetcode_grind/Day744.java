package leetcode_grind;

import java.util.HashSet;
import java.util.Set;

public class Day744 {
    // https://leetcode.com/problems/find-the-celebrity/description/
    /*
     * The knows API is defined in the parent class Relation.
     * boolean knows(int a, int b);
     */

    // static public class Solution extends Relation {
    // int numberOfPeople;

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

    // https://leetcode.com/problems/check-if-n-and-its-double-exist/description/
    static class Solution1 {
        public boolean checkIfExist(int[] arr) {
            Set<Integer> seen = new HashSet<>();
            for (int num : arr) {
                if (seen.contains(2 * num) || (num % 2 == 0 && seen.contains(num / 2))) {
                    return true;
                }
                seen.add(num);
            }
            return false;
        }
    }

}
