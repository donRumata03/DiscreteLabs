from copy import copy
from typing import List, Tuple, Dict


def int_table_to_bool(t):
	return [(tuple(bool(b) for b in p[0]), bool(p[1])) for p in t]


def inverse_bool_vec(vec):
	return tuple(not i for i in vec)


class BoolFunction:
	"""
	Stores truth table as dict and list and can be called
	// TODO: <b>May be</b> - perceive indexes of values as permutation encodings?
	// TODO: arg (0, ..., 0) is at index 0, so don't need to provide it!
	"""

	def __init__(self, truth_table: Tuple[Tuple[Tuple[bool, ...], bool]]):
		self.truth_table = truth_table[:]
		self.truth_map = {}
		for p in self.truth_table:
			self.truth_map[p[0]] = p[1]

	def dims(self):
		return len(self.truth_table[0][0])

	def __call__(self, bool_vector):
		assert len(bool_vector) == self.dims()

		return self.truth_map[tuple(bool_vector)]

	def __str__(self):
		return str(self.truth_map)

	def __repr__(self):
		return str(self.truth_table)
