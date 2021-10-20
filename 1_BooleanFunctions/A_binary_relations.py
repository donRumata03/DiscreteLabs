def is_reflexive(r):
	return all([r[i][i] for i in range(len(r))])

def is_anti_reflexive(r):
	return all([not r[i][i] for i in range(len(r))])

def is_symmetric(r):
	for i in range(len(r)):
		for j in range(i + 1, len(r)):
			if (r[i][j] and not r[j][i]) or (not r[i][j] and r[j][i]):
				return False
	return True


def is_anti_symmetric(r):
	for i in range(len(r)):
		for j in range(i + 1, len(r)):
			if r[i][j] and r[j][i]:
				return False
	return True


def is_transitive(r):
	n = len(r)

	for i in range(n):
		for j in range(n):
			for k in range(n):
				if r[i][j] and r[j][k] and not r[i][k]:
					return False
	return True


def compose_relations(rl, rr):
	n = len(rl)
	res = [[False] * n for _ in range(n)]

	for i in range(n):
		for j in range(n):
			for k in range(n):
				if rl[i][k] and rr[k][j]:
					res[i][j] = True
					break

	return res


n = int(input())

r1, r2 = ([] for _ in range(2))

for rx in (r1, r2):
	for i in range(n):
		rx.append(list(map(bool, map(int, input().split()))))

for rx in (r1, r2):
	print(*[int(property(rx)) for property in [is_reflexive, is_anti_reflexive, is_symmetric, is_anti_symmetric, is_transitive]])

for l in compose_relations(r1, r2):
	print(*list(map(int, l)))


"""

3
0 1 0
0 0 1
1 0 0
1 1 0
0 1 1
1 0 1

"""
