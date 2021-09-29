from BoolFunction import *

from itertools import product

def get_all_functions(n: int) -> List[BoolFunction]:
	possible_args = tuple(product((0, 1), repeat=n))
	possible_function_values = list(product((0, 1), repeat=len(possible_args)))

	res = []
	for value_set in possible_function_values:
		res.append(BoolFunction(tuple(zip(possible_args, value_set))))

	return res



# Checking properties:

def is_preserving_zero():
	pass

def is_preserving_one():
	pass

def is_monotonous():
	pass

def is_self_dual():
	pass

def is_linear(func):
	pass
