from itertools import accumulate

from matplotlib import pyplot as plt
from math import cos, sqrt

def the_row(q):
	return lambda n: (-1) ** n * cos(n) ** 2 / sqrt(n ** q + 1) #  1/sqrt(n ** ) # cos(n) ** 2

def gen(row, n):
	return [row(i) for i in range(1, n + 1)]

def gen_abs(row, n):
	return [abs(row(i)) for i in range(1, n + 1)]

def gen_sum(row, n):
	return list(accumulate(gen(row, n)))

def gen_abs_sum(row, n):
	return list(accumulate(gen_abs(row, n)))


def apl(ms, label):
	plt.plot(range(len(ms)), ms, label = label)


N = 10000
q = 0.8
r = the_row(q)
apl(gen(r, N), "Row")
apl(gen_sum(r, N), "Row sum")

apl(gen_abs(r, N), "Abs row")
apl(gen_abs_sum(r, N), "Abs row sum")

plt.legend()
plt.show()
