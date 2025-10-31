package leetcode_grind;

import java.util.HashMap;
import java.util.Map;
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

    /**
     * Your VideoSharingPlatform object will be instantiated and called as such:
     * VideoSharingPlatform obj = new VideoSharingPlatform();
     * int param_1 = obj.upload(video);
     * obj.remove(videoId);
     * String param_3 = obj.watch(videoId,startMinute,endMinute);
     * obj.like(videoId);
     * obj.dislike(videoId);
     * int[] param_6 = obj.getLikesAndDislikes(videoId);
     * int param_7 = obj.getViews(videoId);
     */
}
