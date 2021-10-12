from functools import reduce
from operator import xor
from typing import Optional, Union

from BF_lib.BoolFunction import *


class ZhegalkinPolynomial:
	def __init__(self, index_ms):
		self.index_ms = index_ms[:]
		self.dims = round(math.log(len(index_ms), 2))
		assert 2 ** self.dims == len(index_ms)

	def dimensions(self):
		return self.dims

	def at(self, bitmask):
		assert bitmask < 2 ** self.dims
		return self.index_ms[bitmask]


	def __str__(self):
		return str(as_dict_by_bin_vec(self.index_ms, self.dims))


def get_slight_dominated(bitmask) -> list:
	slight_dominated = []
	shift = 0
	while (1 << shift) <= bitmask:
		if (1 << shift) & bitmask:
			slight_dominated.append(bitmask & ~(1 << shift))
		shift += 1

	# print(bin(bitmask)[2:], list(map(lambda x: bin(x)[2:], slight_dominated)))

	return slight_dominated


def get_dominated_or_eq(bitmask):
	if bitmask == 0:
		return [bitmask]
	else:
		return reduce(lambda s1, s2: s1.union(s2), [set(get_dominated_or_eq(d)) for d in get_slight_dominated(bitmask)] + [{bitmask}])


def mobius_transform(f: Union[BoolFunction, ZhegalkinPolynomial]):
	res_vector = [None] * (2 ** f.dimensions())

	def gen_answer(bitmask):
		assert bitmask < len(res_vector)

		if res_vector[bitmask] is not None:
			return res_vector[bitmask]


		res_vector[bitmask] = reduce(xor, [f.at(d) for d in get_dominated_or_eq(bitmask)])
		return res_vector[bitmask]

	# gen_answer(2 ** f.dimensions() - 1)

	for bm in range(2 ** f.dimensions()):
		gen_answer(bm)

	RetType = ZhegalkinPolynomial if isinstance(f, BoolFunction) else BoolFunction
	return RetType(res_vector)
