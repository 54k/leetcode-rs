package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeMap;

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

    // https://leetcode.com/problems/odd-even-jump/description/
    static class Solution4 {
        public int oddEvenJumps(int[] A) {
            int n = A.length, res = 1;
            boolean[] higher = new boolean[n], lower = new boolean[n];
            higher[n - 1] = lower[n - 1] = true;
            TreeMap<Integer, Integer> map = new TreeMap<>();
            map.put(A[n - 1], n - 1);
            for (int i = n - 2; i >= 0; --i) {
                Map.Entry<Integer, Integer> hi = map.ceilingEntry(A[i]), lo = map.floorEntry(A[i]);
                if (hi != null)
                    higher[i] = lower[(int) hi.getValue()];
                if (lo != null)
                    lower[i] = higher[(int) lo.getValue()];

                if (higher[i])
                    res++;
                map.put(A[i], i);
            }
            return res;
        }
    }

    // https://leetcode.com/problems/license-key-formatting/description/
    static class Solution5 {
        public String licenseKeyFormatting(String s, int k) {
            var n = s.length();
            var res = new StringBuilder();
            for (int i = n - 1, cur = 0; i >= 0; i--) {
                if (s.charAt(i) == '-')
                    continue;
                res.append(Character.toUpperCase(s.charAt(i)));
                cur++;
                if (cur == k && i != 0) {
                    cur = 0;
                    res.append('-');
                }
            }
            if (res.length() > 0 && res.charAt(res.length() - 1) == '-')
                res.setLength(res.length() - 1);
            return res.reverse().toString();
        }
    }

    static class Solution6 {
        public String licenseKeyFormatting(String s, int k) {
            int totalChars = 0;
            for (int i = 0; i < s.length(); i++) {
                if (s.charAt(i) != '-') {
                    totalChars++;
                }
            }
            int sizeOfFirstGroup = (totalChars % k);
            if (sizeOfFirstGroup == 0) {
                sizeOfFirstGroup = k;
            }
            StringBuilder ans = new StringBuilder();
            int i = 0;
            int count = 0;

            while (i < s.length()) {
                if (count == sizeOfFirstGroup) {
                    count = 0;
                    break;
                }
                if (s.charAt(i) != '-') {
                    count++;
                    ans.append(Character.toUpperCase(s.charAt(i)));
                }
                i++;
            }

            if (i >= s.length()) {
                return ans.toString();
            }

            ans.append('-');
            while (i < s.length()) {
                if (s.charAt(i) != '-') {
                    if (count == k) {
                        ans.append('-');
                        count = 0;
                    }
                    ans.append(Character.toUpperCase(s.charAt(i)));
                    count++;
                }
                i++;
            }
            return ans.toString();
        }
    }

    // https://leetcode.com/problems/fruit-into-baskets/description/
    static class Solution7 {
        public int totalFruit(int[] fruits) {
            int maxPicked = 0;
            for (int left = 0; left < fruits.length; left++) {
                for (int right = 0; right < fruits.length; ++right) {
                    Set<Integer> basket = new HashSet<>();
                    for (int currentIndex = left; currentIndex <= right; ++currentIndex) {
                        basket.add(fruits[currentIndex]);
                    }
                    if (basket.size() <= 2) {
                        maxPicked = Math.max(maxPicked, right - left + 1);
                    }
                }
            }
            return maxPicked;
        }
    }

    static class Solution8 {
        public int totalFruit(int[] fruits) {
            int maxPicked = 0;
            for (int left = 0; left < fruits.length; ++left) {
                Set<Integer> basket = new HashSet<>();
                int right = left;
                while (right < fruits.length) {
                    if (!basket.contains(fruits[right]) && basket.size() == 2) {
                        break;
                    }
                    basket.add(fruits[right]);
                    right++;
                }
                maxPicked = Math.max(maxPicked, right - left);
            }
            return maxPicked;
        }
    }

    static class Solution9 {
        public int totalFruit(int[] fruits) {
            Map<Integer, Integer> basket = new HashMap<>();
            int left = 0, right;
            for (right = 0; right < fruits.length; ++right) {
                basket.put(fruits[right], basket.getOrDefault(fruits[right], 0) + 1);
                if (basket.size() > 2) {
                    basket.put(fruits[left], basket.get(fruits[left]) - 1);
                    if (basket.get(fruits[left]) == 0) {
                        basket.remove(fruits[left]);
                    }
                    left++;
                }
            }
            return right - left;
        }
    }

    static class Solution10 {
        public int totalFruit(int[] fruits) {
            Map<Integer, Integer> basket = new HashMap<>();
            int left = 0, maxPicked = 0;

            for (int right = 0; right < fruits.length; ++right) {
                basket.put(fruits[right], basket.getOrDefault(fruits[right], 0) + 1);

                while (basket.size() > 2) {
                    basket.put(fruits[left], basket.get(fruits[left]) - 1);
                    if (basket.get(fruits[left]) == 0) {
                        basket.remove(fruits[left]);
                    }
                    left++;
                }

                maxPicked = Math.max(maxPicked, right - left + 1);
            }

            return maxPicked;
        }
    }
}
