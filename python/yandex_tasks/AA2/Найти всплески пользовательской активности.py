#  Есть последовательность событий, каждое событие это пара user_id, time, события отсортированы по времени
#  Нужно уметь отвечать на вопрос, сколько за последние 5 минут было пользователей, которые задали >= 1000 запросов.

from collections import defaultdict


class UserStatistics(object):
    def __init__(self, k=5, limit=1000):
        self.window_time = k
        self.limit = limit
        self.user_events_in_window = defaultdict(int)  # dict user: events_count
        self.events_window = []  # list of (time, user_id)
        self.big_users_count = 0

    def clean_old_events(self, time):
        first_in_window = time - self.window_time

        while self.events_window and self.events_window[0][0] < first_in_window:
            if self.user_events_in_window[self.events_window[0][1]] == 1:
                self.user_events_in_window.pop(self.events_window[0][1])
            else:
                self.user_events_in_window[self.events_window[0][1]] -= 1

            if self.user_events_in_window[self.events_window[0][1]] == self.limit - 1:
                self.big_users_count -= 1

            self.events_window.pop(0)

    def event(self, time, user_id):
        self.clean_old_events(time)

        self.events_window.append((time, user_id))
        self.user_events_in_window[user_id] += 1

        if self.user_events_in_window[user_id] == self.limit:
            self.big_users_count += 1

    def get_robots_count(self, time):
        self.clean_old_events(time)
        return self.big_users_count
