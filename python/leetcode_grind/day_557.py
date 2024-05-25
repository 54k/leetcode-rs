# https://leetcode.com/problems/word-break-ii/description
class Solution1:
    def wordBreak(self, s: str, wordDict: List[str]) -> List[str]:
        n = len(s)
        wd = set(wordDict)
        ans = []
        def dfs(i, cur):
            # print(f"{i} of {n}, {cur}")
            if i == n:
                ans.append(" ".join(cur))
                return
            for k in range(i, n+1):
                ss = s[i:k]
                if ss in wd:
                    dfs(k, cur + [ss])

        dfs(0, [])
        return ans

class TrieNode:
    def __init__(self):
        self.isEnd = False
        self.children = [None] * 26

class Trie:
    def __init__(self):
        self.root = TrieNode()
    
    def insert(self, word):
        node = self.root
        for char in word:
            index = ord(char) - ord('a')
            if not node.children[index]:
                node.children[index] = TrieNode()
            node = node.children[index]
        node.isEnd = True

# https://leetcode.com/problems/word-break-ii/description
class Solution2:
    def wordBreak(self, s: str, wordDict: List[str]) -> List[str]:
        trie = Trie()
        for word in wordDict:
            trie.insert(word)
        
        dp = {}

        for start_idx in range(len(s), -1, -1):
            valid_sentences = []
            current_node = trie.root

            for end_idx in range(start_idx, len(s)):
                char = s[end_idx]
                index = ord(char) - ord('a')
                if not current_node.children[index]:
                    break
                
                current_node = current_node.children[index]

                if current_node.isEnd:
                    current_word = s[start_idx : end_idx + 1]
                    if end_idx == len(s) - 1:
                        valid_sentences.append(current_word)
                    else:
                        sentences_from_next_index = dp.get(end_idx + 1, [])
                        for sentence in sentences_from_next_index:
                            valid_sentences.append(current_word + " " + sentence)
            dp[start_idx] = valid_sentences

        return dp.get(0, [])