"""
Check if NFA accepts word
© Vladimir Latypov™
"""
from collections import defaultdict

file = open("problem2.in", "r")

word = file.readline()[:-1]

n, m, k = map(int, file.readline().split())
terminal = list(map(int, file.readline().split()))
# assert len(terminal) == k

transitions = [defaultdict(list) for i in range(n + 1)] # For all states keys are chars, values are lists of state indexes

for i in range(m):
	fr, to, c = file.readline().split()
	fr = int(fr)
	to = int(to)
	transitions[fr][c].append(to)

dp = [[False] * (n + 1)] # dp[i][j] is boolean «if NFA can be at state j after consuming word[0:i]», dp[0][j] is for empty prefix, j = 0 is nonsense
dp[0][1] = True

for c in word:
	new_layer = [False] * (n + 1)

	for source in range(n + 1):
		if not dp[-1][source]:
			continue
		for target in transitions[source][c]:
			new_layer[target] = True

	dp.append(new_layer)

open("problem2.out", "w").write("Accepts" if any([x in terminal for x in range(n + 1) if dp[-1][x]]) else "Rejects")
