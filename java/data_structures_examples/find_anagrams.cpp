// https://leetcode.com/problems/find-all-anagrams-in-a-string/description/
class Solution {
public:
    vector<int> findAnagrams(string s, string p) {
        vector<int> res;       
        if (s.size() < p.size()) {
            return res;
        }

        int m[26];
        for (auto &ch : p) {
            m[ch-'a']++;
        }

        int left = 0, right = 0, count = p.size();
        while (right < s.size()) {
            if (m[s[right++]-'a']-- >= 1) count--;

            if (count == 0) res.push_back(left);

            if (right - left + 1 > p.size() && m[s[left++]-'a']++ >= 0) count++;
        }

        return res;
    }
};