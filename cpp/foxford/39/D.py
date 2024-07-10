import heapq

def dijkstra(n, start, end, buses):
    INF = 10001  # время больше, чем любое возможное в задаче
    min_time = [INF] * (n + 1)
    min_time[start] = 0
    pq = [(0, start)]  # приоритетная очередь, содержащая (время, деревня)

    while pq:
        current_time, village = heapq.heappop(pq)
        if current_time > min_time[village]:
            continue
        
        for dep_time, arr_village, arr_time in buses[village]:
            if dep_time >= current_time and arr_time < min_time[arr_village]:
                min_time[arr_village] = arr_time
                heapq.heappush(pq, (arr_time, arr_village))

    return min_time[end] if min_time[end] != INF else -1


n = int(input().strip())
start, end = map(int, input().strip().split())
m = int(input().strip())
buses = [[] for _ in range(n + 1)]

for _ in range(m):
    u, dep_time, v, arr_time = map(int, input().strip().split())
    buses[u].append((dep_time, v, arr_time))

result = dijkstra(n, start, end, buses)
print(result)


