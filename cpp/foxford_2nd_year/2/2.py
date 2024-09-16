t, d = input().split()
h, m = t.split(":")
if d[0] == "p":
    if int(h) != 12:
        h = str(int(h) + 12)
elif int(h) < 10:
    h = f"0{h}"
print(f"{h}:{m}", sep="")