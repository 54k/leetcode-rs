package day292

import "sort"

// https://leetcode.com/problems/course-schedule-iii/description/
func scheduleCourse(courses [][]int) int {
	sort.Slice(courses, func(i, j int) bool {
		return courses[i][1] < courses[j][1]
	})

	time, count := 0, 0
	for i := 0; i < len(courses); i++ {
		if time+courses[i][0] <= courses[i][1] {
			courses[count] = courses[i]
			count++
			time += courses[i][0]
		} else {
			max_i := i
			for j := 0; j < count; j++ {
				if courses[j][0] > courses[max_i][0] {
					max_i = j
				}
			}
			if courses[max_i][0] > courses[i][0] {
				time += courses[i][0] - courses[max_i][0]
				courses[max_i] = courses[i]
			}
		}
	}

	return count
}
