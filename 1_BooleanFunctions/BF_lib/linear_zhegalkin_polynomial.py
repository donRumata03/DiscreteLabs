from functools import reduce
from operator import xor

from BF_lib.BoolFunction import *


class LinearZhegalkinPolynomial:
	def __init__(self, a_0, a_s):
		self.a_0 = a_0
		self.a_s = a_s

	def dims(self):
		return len(self.a_s)

	def __call__(self, bool_vector):
		assert self.dims() == len(bool_vector)

		return reduce(xor, [self.a_0] + [bool_vector[i] and self.a_s[i] for i in range(self.dims())])

	def __str__(self):
		return str(self.a_0) + ", " + str(self.a_s)


def build_linear(bf: BoolFunction):
	n = bf.dimensions()
	a_0 = bf(bin_vec_to_mask([False] * n))
	a_s = [a_0 ^ bf(bin_vec_to_mask([False] * i + [True] + [False] * (n - i - 1))) for i in range(n)]
	return LinearZhegalkinPolynomial(a_0, a_s)


