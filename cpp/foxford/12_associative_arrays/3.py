out = ""
s = input() 
cur=0

for ch in s:
    if ch.isdigit():
        cur = cur * 10 + ord(ch) - ord('0')
    else:
        if cur > 0:
            out += bin(cur)[2:]
            cur = 0
        out += ch

if cur > 0:
    out += bin(cur)[2:]

print(out)