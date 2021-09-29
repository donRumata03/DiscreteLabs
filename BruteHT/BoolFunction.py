from copy import copy
from typing import List, Tuple, Dict


class BoolFunction:
	"""
	Stores truth table as dict and list and can be called
	// TODO: <b>May be</b> - perceive indexes of values as permutation encodings?
	// TODO: arg (0, ..., 0) is at index 0, so don't need to provide it!
	"""

	# self.truth_table: Tuple[Tuple[Tuple[int, ...], int]] = []
	# truth_map: Dict[Tuple[int, ...], int] = {}

	def __init__(self, truth_table: Tuple[Tuple[Tuple[int, ...], int]]):
		self.truth_table = truth_table[:]
		self.truth_map = {}
		for p in self.truth_table:
			self.truth_map[p[0]] = p[1]
		# print(self.truth_map)

	# def gen_map(self):


	def dims(self):
		return len(self.truth_table[0][0])

	def __call__(self, bool_vector):
		assert len(bool_vector) == self.dims()

		return self.truth_map

	def __str__(self):
		return str(self.truth_map)
