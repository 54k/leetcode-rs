import base64

s = "HATQKX YIGZZKX SAYZ IULLKK"

for offset in range(0, 26):
    A = ord("A")
    res = ""
    for i in range(len(s)):
        if s[i] == " ":
            res += " "
        else:
            res += str(chr(((ord(s[i]) - A) + offset) % 26 + A))
    print(f"{res}")

ords = [99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52, 98, 108, 51, 125]

s = "".join([chr(x) for x in ords])
print(s)

print("Here is your flag:")
print("".join(chr(o ^ 0x32) for o in ords))

print(bytes.fromhex("72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf"))

print(base64.b64encode(bytes.fromhex("72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf")))
