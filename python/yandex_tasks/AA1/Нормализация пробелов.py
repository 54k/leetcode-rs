from pprint import pprint


def norm(st):
    i = len(st) - 1
    while i > 0:
        if st[i] == ' ' and st[i - 1] == ' ':
            st.pop(i)
        i -= 1
    pprint(st)


ex = [' ', ' ', 's', 'o', ' ', 'm', 'e', ' ', ' ', ' ', ' ', ' ', 's', 't', 'r', 'i', 'n', 'g', ' ', ' ', ' ', ' ']

norm(ex)
