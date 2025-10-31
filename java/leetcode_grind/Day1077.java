package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeSet;

public class Day1077 {
    // https://leetcode.com/problems/the-two-sneaky-numbers-of-digitville/description/?envType=daily-question&envId=2025-10-31
    static class Solution1 {
        public int[] getSneakyNumbers(int[] nums) {
            int n = nums.length - 2;
            int y = 0;
            for (int x : nums) {
                y ^= x;
            }
            for (int i = 0; i < n; i++) {
                y ^= i;
            }

            int lowBit = y & -y;
            int x1 = 0;
            int x2 = 0;

            for (int x : nums) {
                if ((x & lowBit) != 0) {
                    x1 ^= x;
                } else {
                    x2 ^= x;
                }
            }

            for (int i = 0; i < n; i++) {
                if ((i & lowBit) != 0) {
                    x1 ^= i;
                } else {
                    x2 ^= i;
                }
            }
            return new int[] { x1, x2 };
        }
    }

    // https://leetcode.com/problems/design-video-sharing-platform/description/
    static class VideoSharingPlatform {
        int idHigh = -1;
        TreeSet<Integer> availableIds = new TreeSet<>();
        Map<Integer, String> videos = new HashMap<>();

        Map<Integer, Integer> views = new HashMap<>();
        Map<Integer, Integer> likes = new HashMap<>();
        Map<Integer, Integer> dislikes = new HashMap<>();

        public VideoSharingPlatform() {

        }

        public int upload(String video) {
            int id;
            if (availableIds.size() > 0) {
                id = availableIds.removeFirst();
            } else {
                id = ++idHigh;
            }
            videos.put(id, video);
            views.put(id, 0);
            likes.put(id, 0);
            dislikes.put(id, 0);
            return id;
        }

        public void remove(int videoId) {
            if (!videos.containsKey(videoId)) {
                return;
            }
            videos.remove(videoId);
            availableIds.add(videoId);

            views.remove(videoId);
            likes.remove(videoId);
            dislikes.remove(videoId);
        }

        public String watch(int videoId, int startMinute, int endMinute) {
            if (!videos.containsKey(videoId)) {
                return "-1";
            }
            String video = videos.get(videoId);
            String ret = video.substring(startMinute, Math.min(endMinute + 1, video.length()));
            views.put(videoId, views.get(videoId) + 1);
            return ret;
        }

        public void like(int videoId) {
            if (!videos.containsKey(videoId)) {
                return;
            }
            likes.put(videoId, likes.get(videoId) + 1);
        }

        public void dislike(int videoId) {
            if (!videos.containsKey(videoId)) {
                return;
            }
            dislikes.put(videoId, dislikes.get(videoId) + 1);
        }

        public int[] getLikesAndDislikes(int videoId) {
            if (!videos.containsKey(videoId)) {
                return new int[] { -1 };
            }

            return new int[] {
                    likes.get(videoId),
                    dislikes.get(videoId),
            };
        }

        public int getViews(int videoId) {
            if (!videos.containsKey(videoId)) {
                return -1;
            }
            return views.get(videoId);
        }
    }

    // https://leetcode.com/problems/3sum/description/
    static class Solution2 {
        public List<List<Integer>> threeSum(int[] nums) {
            Set<List<Integer>> res = new HashSet<>();
            Set<Integer> dups = new HashSet<>();
            Map<Integer, Integer> seen = new HashMap<>();
            for (int i = 0; i < nums.length; ++i) {
                if (dups.add(nums[i])) {
                    for (int j = i + 1; j < nums.length; ++j) {
                        int complement = -nums[i] - nums[j];
                        if (seen.containsKey(complement) && seen.get(complement) == i) {
                            List<Integer> triplet = Arrays.asList(
                                    nums[i],
                                    nums[j],
                                    complement);
                            Collections.sort(triplet);
                            res.add(triplet);
                        }
                        seen.put(nums[j], i);
                    }
                }
            }
            return new ArrayList<>(res);
        }
    }
}
