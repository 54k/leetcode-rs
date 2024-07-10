N = int(input()) 
D, V = map(int, input().split()) 
Buses = [[] for i in range(N + 1)] 
R = int(input()) 
for i in range(R): 
    start, start_time, finish, finish_time = map(int, input().split()) 
    Buses[start].append((start_time, finish, finish_time)) 

INF = 10 ** 10 

Time = [INF] * (N + 1) 
Time[D] = 0 
Colored = [False] * (N + 1) 

while True: 
    min_time = INF 
    for i in range(1, N + 1): 
        if not Colored[i] and Time[i] < min_time: 
            min_time = Time[i] 
            min_village = i 
    if min_time == INF: 
        break 
    start = min_village 
    Colored[start] = True 
    for start_time, finish, finish_time in Buses[start]: 
        if Time[start] <= start_time and finish_time < Time[finish]: 
            Time[finish] = finish_time 

if Time[V] == INF: 
    print(-1) 
else: 
    print(Time[V])