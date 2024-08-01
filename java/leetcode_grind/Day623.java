package leetcode_grind;

public class Day623 {
    // https://leetcode.com/problems/number-of-senior-citizens/description/?envType=daily-question&envId=2024-08-01
    static class Solution1 {
        public int countSeniors(String[] details) {
            int seniorCount = 0;

            for (String passengerInfo : details) {
                int ageTens = passengerInfo.charAt(11) - '0';
                int ageOnes = passengerInfo.charAt(12) - '0';

                int age = ageTens * 10 + ageOnes;

                if (age > 60) {
                    seniorCount++;
                }
            }

            return seniorCount;
        }
    }
}
