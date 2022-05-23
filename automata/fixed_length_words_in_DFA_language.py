"""
Count words of length l in language
© Vladimir Latypov™
"""

file = open("problem4.in", "r")

n, m, k, l = map(int, file.readline().split())
terminal = list(map(int, file.readline().split()))
# assert len(terminal) == k

transitions = [dict() for i in range(n + 1)] # For all states keys are chars, values are lists of state indexes

for i in range(m):
	fr, to, c = file.readline().split()
	fr = int(fr)
	to = int(to)
	transitions[fr][c] = to

# dp[i][j] is number of words (€ \Sigma^*) with length i such that this word consumption at automata leads to state j, j = 0 is nonsense, i = 0 is for empty word
dp = [[0] * (n + 1)]
dp[0][1] = 1

for i in range(1, l + 1):
	new_layer = [0] * (n + 1)

	for source in range(n + 1):
		for c in transitions[source]:
			new_layer[transitions[source][c]] += dp[-1][source]

	dp.append(new_layer)

open("problem4.out", "w").write(str((sum([dp[-1][i] for i in range(n + 1) if i in terminal])) % (10**9 + 7)))
