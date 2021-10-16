

###############################################################################################


import math
from copy import copy
from functools import reduce
from operator import xor
from typing import List, Tuple, Dict, Iterable, Collection, Union


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
	return tuple(res)


def bin_vec_to_mask(vec: Iterable[bool]):
	res = 0

	for pos in range(len(vec)):
		res |= (vec[pos] << pos)

	return res


def set_bit_to(mask: int, index: int, value: bool):
	return (mask & ~(1 << index)) | (value << index)


def as_dict_by_bin_vec(ms, dims):
	res = {}
	for i in range(len(ms)):
		res[to_bin_vec(i, dims)] = ms[i]

	return str(res)


class BoolFunction:
	"""
	Stores truth table as list of booleans and can be called
	"""

	def __init__(self, truth_ms: Union[List[bool], Tuple[bool]]):
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


##################################################################

from itertools import product


def get_all_functions(n: int) -> List[BoolFunction]:
	possible_args = tuple(product((False, True), repeat=n))
	possible_function_values = list(product((False, True), repeat=len(possible_args)))

	res = []
	for value_set in possible_function_values:
		res.append(BoolFunction(
			# tuple(zip(possible_args, value_set))
			tuple(map(bool, value_set))
		))

	return res


# Checking lying Post classes:
def is_preserving_zero(bf: BoolFunction):
	return not bf(0)


def is_preserving_one(bf: BoolFunction):
	return bf.truth_ms[-1]


def is_monotonous(bf: BoolFunction):
	n = bf.dimensions()

	for (mask, value) in enumerate(bf.truth_ms):
		for i in range(n):
			if not (mask & (1 << i)):
				dominator = mask | (1 << i)
				if bf(dominator) < value:
					return False

	return True


def is_self_dual(bf: BoolFunction):
	for (mask, value) in enumerate(bf.truth_ms):
		if value == bf(bin_vec_to_mask(inverse_bin_vec(to_bin_vec(mask, bf.dimensions())))):
			return False
	return True


def is_linear(bf: BoolFunction):
	# a_0 = bf([False] * n)
	# a_s = [a_0 ^ bf([False] * i + [True] + [False] * (n - i - 1)) for i in range(n)]
	# pz = LinearZhegalkinPolynomial(a_0, a_s)

	pz = build_linear(bf)

	for (mask, value) in enumerate(bf.truth_ms):
		if pz(to_bin_vec(mask, bf.dimensions())) != value:
			return False

	return True


post_properties = [
	("Preserving zero", is_preserving_zero),
	("Preserving one", is_preserving_one),
	("Self-Dual", is_self_dual),
	("Monotonous", is_monotonous),
	("Linear", is_linear),
]
post_class_checkers = [
	p[1] for p in post_properties
]

def check_bf_properties(bf: BoolFunction):
	for p in post_properties + [("Significantly depends on all args", significantly_depends_on_all)]:
		print(p[0] + ":", int(p[1](bf)))

### Significantly depends:


def significantly_depends(bf: BoolFunction, arg_index: int):
	for (mask, value) in enumerate(bf.truth_ms):
		alter_vector = set_bit_to(mask, arg_index, not (mask & (1 << arg_index)))
		# alter_vector[arg_index] = not k[arg_index]
		if bf(alter_vector) != value:
			return True
	return False

def significantly_depends_on_at_least(bf: BoolFunction, at_least: int):
	return sum([significantly_depends(bf, i) for i in range(bf.dimensions())]) >= at_least

def significantly_depends_on_all(bf: BoolFunction):
	return significantly_depends_on_at_least(bf, bf.dimensions())


### Post theorem:

def function_set_is_basis(functions: List[BoolFunction]):
	for (p_name, p_checker) in post_properties:
		sats = [p_checker(f) for f in functions]
		if all(sats):
			# print(f"All functions are in {p_name} class!")
			return False

	return True


##################################################################

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

#################           MAIN            #################

if __name__ == '__main__':
	functions = []

	n = int(input())
	for i in range(n):
		functions.append(BoolFunction(list(map(lambda c: bool(int(c)), input().split(" ")[1]))))

	print("YES" if function_set_is_basis(functions) else "NO")

