package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class Day581 {
    // https://leetcode.com/problems/most-profit-assigning-work/description/
    static class Solution1 {
        public int maxProfitAssignment(int[] difficulty, int[] profit, int[] worker) {
            List<int[]> jobProfile = new ArrayList<>();
            jobProfile.add(new int[] { 0, 0 });
            for (int i = 0; i < difficulty.length; i++) {
                jobProfile.add(new int[] { difficulty[i], profit[i] });
            }

            Collections.sort(jobProfile, (a, b) -> Integer.compare(a[0], b[0]));
            for (int i = 0; i < jobProfile.size() - 1; i++) {
                jobProfile.get(i + 1)[1] = Math.max(jobProfile.get(i)[1], jobProfile.get(i + 1)[1]);
            }

            int netProfit = 0;
            for (int i = 0; i < worker.length; i++) {
                int ability = worker[i];
                int l = 0, r = jobProfile.size() - 1, jobProfit = 0;
                while (l <= r) {
                    int mid = (l + r) / 2;
                    if (jobProfile.get(mid)[0] <= ability) {
                        jobProfit = Math.max(jobProfit, jobProfile.get(mid)[1]);
                        l = mid + 1;
                    } else {
                        r = mid - 1;
                    }
                }
                netProfit += jobProfit;
            }
            return netProfit;
        }
    }

    // https://leetcode.com/problems/most-profit-assigning-work/description
    static class Solution2 {
        public int maxProfitAssignment(int[] difficulty, int[] profit, int[] worker) {
            List<int[]> jobProfile = new ArrayList<>();
            jobProfile.add(new int[] { 0, 0 });
            for (int i = 0; i < difficulty.length; i++) {
                jobProfile.add(new int[] { profit[i], difficulty[i] });
            }

            jobProfile.sort((a, b) -> Integer.compare(b[0], a[0]));
            for (int i = 0; i < jobProfile.size() - 1; i++) {
                jobProfile.get(i + 1)[1] = Math.min(jobProfile.get(i)[1], jobProfile.get(i + 1)[1]);
            }

            int netProfit = 0;
            for (int i = 0; i < worker.length; i++) {
                int ability = worker[i];
                int l = 0, r = jobProfile.size() - 1, jobProfit = 0;
                while (l <= r) {
                    int mid = (l + r) / 2;
                    if (jobProfile.get(mid)[1] <= ability) {
                        jobProfit = Math.max(jobProfit, jobProfile.get(mid)[0]);
                        r = mid - 1;
                    } else {
                        l = mid + 1;
                    }
                }
                netProfit += jobProfit;
            }
            return netProfit;
        }
    }
}
