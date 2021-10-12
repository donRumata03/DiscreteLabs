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
	return tuple(map(bool, list(to_counting_system(index, 2))))

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


	def __str__(self):
		return str(as_dict_by_bin_vec(self.truth_ms, self.dims))

	def __repr__(self):
		return str(self.truth_ms)
