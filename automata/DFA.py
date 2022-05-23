"""
Check if DFA accepts word
© Vladimir Latypov™
"""

file = open("problem1.in", "r")

word = file.readline()[:-1]

n, m, k = map(int, file.readline().split())
terminal = list(map(int, file.readline().split()))
# assert len(terminal) == k

transitions = [dict() for i in range(n + 1)] # For all states keys are chars, values are state indexes


for i in range(m):
	fr, to, c = file.readline().split()
	fr = int(fr)
	to = int(to)
	transitions[fr][c] = to

state = 1
for c in word:
	if c in transitions[state]:
		state = transitions[state][c]
	else:
		state = None
		break

open("problem1.out", "w").write("Accepts" if state in terminal else "Rejects")
