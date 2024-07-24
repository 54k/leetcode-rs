package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class Day615 {
    // https://leetcode.com/problems/sort-the-jumbled-numbers/description/?envType=daily-question&envId=2024-07-24
    static class Solution1 {
        public int[] sortJumbled(int[] mapping, int[] nums) {
            ArrayList<Integer[]> storePairs = new ArrayList<>();

            for (int i = 0; i < nums.length; i++) {
                String number = Integer.toString(nums[i]);
                String formed = "";
                for (int j = 0; j < number.length(); ++j) {
                    formed = formed + Integer.toString(mapping[number.charAt(j) - '0']);
                }
                int mappedValue = Integer.parseInt(formed);
                storePairs.add(new Integer[] { mappedValue, i });
            }

            Collections.sort(storePairs, new Comparator<Integer[]>() {
                @Override
                public int compare(Integer[] o1, Integer[] o2) {
                    return o1[0].compareTo(o2[0]);
                }
            });

            int[] answer = new int[nums.length];
            for (int i = 0; i < storePairs.size(); i++) {
                answer[i] = nums[storePairs.get(i)[1]];
            }
            return answer;
        }
    }

    // https://leetcode.com/problems/sort-the-jumbled-numbers/description/?envType=daily-question&envId=2024-07-24
    static class Solution2 {
        public int[] sortJumbled(int[] mapping, int[] nums) {
            List<int[]> storePairs = new ArrayList<int[]>();

            for (int i = 0; i < nums.length; i++) {
                int mappedValue = 0;
                int temp = nums[i];
                int place = 1;

                if (temp == 0) {
                    storePairs.add(new int[] { mapping[0], i });
                    continue;
                }

                while (temp != 0) {
                    mappedValue = place * mapping[temp % 10] + mappedValue;
                    place *= 10;
                    temp /= 10;
                }
                storePairs.add(new int[] { mappedValue, i });
            }

            Collections.sort(storePairs, (a, b) -> a[0] - b[0]);

            int[] answer = new int[nums.length];
            for (int i = 0; i < storePairs.size(); i++) {
                answer[i] = nums[storePairs.get(i)[1]];
            }
            return answer;
        }
    }

    // https://leetcode.com/problems/unique-email-addresses/description/
    class Solution3 {
        public int numUniqueEmails(String[] emails) {
            Set<String> uniqueEmails = new HashSet<>();
            for (String email : emails) {
                StringBuilder cleanMail = new StringBuilder();

                for (int i = 0; i < email.length(); i++) {
                    char currChar = email.charAt(i);

                    if (currChar == '+' || currChar == '@')
                        break;
                    if (currChar != '.')
                        cleanMail.append(currChar);
                }

                StringBuilder domainName = new StringBuilder();

                for (int i = email.length() - 1; i >= 0; --i) {
                    char currChar = email.charAt(i);
                    domainName.append(currChar);
                    if (currChar == '@')
                        break;
                }

                domainName = domainName.reverse();
                cleanMail.append(domainName);
                uniqueEmails.add(cleanMail.toString());
            }

            return uniqueEmails.size();
        }
    }
}
