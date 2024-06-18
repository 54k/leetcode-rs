from sortedcontainers import SortedList

class Solution:
    def maxTaskAssign(self, tasks: List[int], workers: List[int], p: int, strength: int) -> int:
        n = len(tasks)
        m = len(workers)

        tasks.sort()
        workers.sort()
        lo = 0
        hi = min(m, n)
        ans = None

        while lo <= hi:
            mid = lo + (hi - lo) // 2
            count = 0
            flag = True

            st = SortedList(workers)

            for i in range(mid - 1, -1, -1):
                it = st[-1]
                if tasks[i] <= it:
                    st.remove(it)
                else:
                    it = st.bisect_left(tasks[i] - strength)
                    if it != len(st):
                        count += 1
                        st.pop(it)
                    else:
                        flag = False
                        break
                if count > p:
                    flag = False
                    break
            if flag:
                ans = mid
                lo = mid + 1
            else:
                hi = mid - 1
                
        return ans