# https://leetcode.com/problems/the-number-of-beautiful-subsets/description/
class Solution1:
    def beautifulSubsets(self, nums: List[int], k: int) -> int:
        total_count = 1
        freq_map = defaultdict(dict)

        for num in nums:
            remainder = num % k
            freq_map[remainder][num] = freq_map[remainder].get(num, 0) + 1
        
        for fr in freq_map.values():
            n = len(fr)

            subsets = sorted(fr.items())
            counts = [0] * (n + 1)
            counts[n] = 1

            for i in range(n - 1, -1, -1):
                skip = counts[i + 1]
                take = 2 ** subsets[i][1] - 1

                if i + 1 < n and subsets[i + 1][0] - subsets[i][0] == k:
                    take *= counts[i + 2]
                else:
                    take *= counts[i + 1]
                
                counts[i] = skip + take
            total_count *= counts[0]

        return total_count - 1

# https://leetcode.com/problems/maximum-score-words-formed-by-letters/description
class Solution2:
    def maxScoreWords(self, words: List[str], letters: List[str], score: List[int]) -> int:
        W = len(words)
        # Count how many times each letter occurs
        self.max_score = 0
        freq = [0 for i in range(26)]
        subset_letters = [0 for i in range(26)]
        for c in letters:
            freq[ord(c) - 97] += 1

        # Check if adding this word exceeds the frequency of any letter
        def is_valid_word(subset_letters):
            for c in range(26):
                if freq[c] < subset_letters[c]:
                    return False
            else:
                return True
        
        def check(w, words, score, subset_letters, total_score):
            if w == -1:
                # If all words have been iterated, check the score of this subset
                self.max_score = max(self.max_score, total_score)
                return
            # Not adding words[w] to the current subset
            check(w - 1, words, score, subset_letters, total_score)
            # Adding words[w] to the current subset
            L = len(words[w])
            for i in range(L):
                c = ord(words[w][i]) - 97
                subset_letters[c] += 1
                total_score += score[c]

            if is_valid_word(subset_letters):
                # Consider the next word if this subset is still valid
                check(w - 1, words, score, subset_letters, total_score)
                
            # Rollback effects of this word
            for i in range(L):
                c = ord(words[w][i]) - 97
                subset_letters[c] -= 1
                total_score -= score[c]

        check(W - 1, words, score, subset_letters, 0)
        # Return max_score as the result
        return self.max_score