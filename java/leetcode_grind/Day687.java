package leetcode_grind;

import java.util.*;

public class Day687 {
    // https://leetcode.com/problems/divide-players-into-teams-of-equal-skill/description/?envType=daily-question&envId=2024-10-04
    static class Solution1 {
        public long dividePlayers(int[] skill) {
            Arrays.sort(skill);
            int i = 0, j = skill.length - 1;
            int max = skill[i] + skill[j];
            long ans = (long) skill[i] * skill[j];
            while (++i < --j) {
                if (skill[i] + skill[j] != max) {
                    return -1;
                }
                ans += ((long) skill[i] * skill[j]);
            }
            return ans;
        }
    }

    static class Solution2 {
        public long dividePlayers(int[] skill) {
            int n = skill.length;
            int totalSkill = 0;
            int[] skillFrequency = new int[1001];
            for (int playerSkill : skill) {
                totalSkill += playerSkill;
                skillFrequency[playerSkill]++;
            }
            if (totalSkill % (n / 2) != 0) {
                return -1;
            }
            int targetTeamSkill = totalSkill / (n / 2);
            long totalChemistry = 0;
            for (int playerSkill : skill) {
                int partnerSkill = targetTeamSkill - playerSkill;
                if (skillFrequency[partnerSkill] == 0) {
                    return -1;
                }
                totalChemistry += (long) playerSkill * (long) partnerSkill;
                skillFrequency[partnerSkill]--;
            }
            return totalChemistry / 2;
        }
    }

    static class Solution3 {
        public long dividePlayers(int[] skill) {
            int n = skill.length;
            int totalSkill = 0;
            Map<Integer, Integer> skillMap = new HashMap<>();
            for (int s : skill) {
                totalSkill += s;
                skillMap.put(s, skillMap.getOrDefault(s, 0) + 1);
            }
            if (totalSkill % (n / 2) != 0) {
                return -1;
            }
            int targetSkill = totalSkill / (n / 2);
            long totalChemistry = 0;
            for (int currSkill : skillMap.keySet()) {
                int currFreq = skillMap.get(currSkill);
                int partnerSkill = targetSkill - currSkill;
                if (!skillMap.containsKey(partnerSkill) || currFreq != skillMap.get(partnerSkill)) {
                    return -1;
                }
                totalChemistry += (long) currSkill * (long) partnerSkill * (long) currFreq;
            }
            return totalChemistry / 2;
        }
    }
}
