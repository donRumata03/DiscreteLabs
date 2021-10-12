# _______________________       BoolFunction        ______________________________

import math
from copy import copy
from typing import List, Tuple, Dict


def int_table_to_bool(t):
	return [(tuple(bool(b) for b in p[0]), bool(p[1])) for p in t]


def inverse_bin_vec(vec):
	return tuple(not i for i in vec)

def to_counting_system(number: int, base: int):
	if number < base:
		return str(number)
	return to_counting_system(number // base, base) + to_counting_system(number % base, base)

def to_bin_vec(index, dims):
	assert index < 2 ** dims

	res = []
	for d in range(dims):
		res.append(bool((index >> d) % 2))
	return tuple(reversed(res))

def as_dict_by_bin_vec(ms, dims):
	res = {}
	for i in range(len(ms)):
		res[to_bin_vec(i, dims)] = ms[i]

	return str(res)


class BoolFunction:
	"""
	Stores truth table as list of booleans and can be called
	"""

	def __init__(self, truth_ms: List[bool]):
		self.truth_ms = truth_ms[:]
		self.dims = round(math.log(len(truth_ms), 2))

	def dimensions(self):
		return self.dims

	def __call__(self, index):
		assert index < len(self.truth_ms)

		return self.truth_ms[index]

	def at(self, index):
		return self(index)

	def __str__(self):
		return str(as_dict_by_bin_vec(self.truth_ms, self.dims))

	def __repr__(self):
		return str(self.truth_ms)


# _______________________       Zhegalkin Polynomial        ______________________________

from functools import reduce
from operator import xor
from typing import Optional, Union

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


# _______________________       MAIN        ______________________________

n = int(input())
truth_table = []
for i in range(2 ** n):
	truth_table.append(bool(int(input().split()[-1])))

b = BoolFunction(truth_table)
# print("Input:", b)

z = mobius_transform(b)
# print(f"Output: {z}")

for bm in range(2 ** n):
	print("".join(map(lambda b: str(int(b)), to_bin_vec(bm, n))), int(z.at(bm)))

