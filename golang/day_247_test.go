package main

// https://leetcode.com/problems/add-two-numbers-ii/description/
func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	reverse := func(list *ListNode) *ListNode {
		var prev *ListNode
		for list != nil {
			tmp := list.Next
			list.Next = prev
			prev = list
			list = tmp
		}
		return prev
	}
	l1 = reverse(l1)
	l2 = reverse(l2)
	total := 0
	var ans *ListNode
	for l1 != nil || l2 != nil {
		if l1 != nil {
			total += l1.Val
			l1 = l1.Next
		}
		if l2 != nil {
			total += l2.Val
			l2 = l2.Next
		}
		ans = &ListNode{total % 10, ans}
		total /= 10
	}
	if total > 0 {
		ans = &ListNode{total, ans}
	}
	return ans
}

// https://leetcode.com/problems/smallest-sufficient-team/description/
func smallestSufficientTeam(req_skills []string, people [][]string) []int {
	popCount := func(i int) int {
		ans := 0
		for i > 0 {
			ans += i & 1
			i >>= 1
		}
		return ans
	}

	n, m := len(people), len(req_skills)
	skillId := map[string]int{}
	for i, s := range req_skills {
		skillId[s] = i
	}

	skillsMaskOfPerson := make([]int, n)
	for i := 0; i < n; i++ {
		mask := 0
		for _, s := range people[i] {
			mask |= 1 << skillId[s]
		}
		skillsMaskOfPerson[i] = mask
	}

	dp := make([]int, 1<<m)
	for i := 0; i < len(dp); i++ {
		dp[i] = (1 << n) - 1
	}
	dp[0] = 0

	for skillMask := 0; skillMask < 1<<m; skillMask++ {
		for person := 0; person < n; person++ {
			smallerSkillMask := skillMask & ^skillsMaskOfPerson[person]
			if smallerSkillMask != skillMask {
				peopleMask := dp[smallerSkillMask] | (1 << person)
				if popCount(peopleMask) < popCount(dp[skillMask]) {
					dp[skillMask] = peopleMask
				}
			}
		}
	}

	mask := dp[(1<<m)-1]
	ans := make([]int, popCount(mask))
	ptr := 0
	for i := 0; i < n; i++ {
		if (mask>>i)&1 == 1 {
			ans[ptr] = i
			ptr++
		}
	}
	return ans
}
