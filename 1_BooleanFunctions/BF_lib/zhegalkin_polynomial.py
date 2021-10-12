from functools import reduce
from operator import xor
from typing import Optional

from BF_lib.BoolFunction import *


class ZhegalkinPolynomial:
	def __init__(self, index_ms):
		self.index_ms = index_ms[:]
		self.dims = round(math.log(len(index_ms), 2))

	def dimensions(self):
		return self.dims

	def at(self, bitmask):
		assert bitmask < 2 ** self.dims
		return self.index_ms[bitmask]


	def __str__(self):
		return str(as_dict_by_bin_vec(self.index_ms, self.dims))


def mobius_transform(f: Optional[BoolFunction, ZhegalkinPolynomial]):
	res_vector = [None] * f.dimensions()

