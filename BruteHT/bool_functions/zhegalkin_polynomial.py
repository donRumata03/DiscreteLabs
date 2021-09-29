from functools import reduce
from operator import xor

class ZhegalkinPolynomial:
	def __init__(self, a_0, a_s):
		self.a_0 = a_0
		self.a_s = a_s

	def dims(self):
		return len(self.a_s)

	def __call__(self, bool_vector):
		assert self.dims() == len(bool_vector)

		return reduce(xor, [self.a_0] + [bool_vector[i] and self.a_s[i] for i in range(self.dims())])
