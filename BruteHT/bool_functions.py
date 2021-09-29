from BoolFunction import *

from itertools import product

def get_all_functions(n: int) -> List[BoolFunction]:
	possible_args = tuple(product((False, True), repeat=n))
	possible_function_values = list(product((False, True), repeat=len(possible_args)))

	res = []
	for value_set in possible_function_values:
		res.append(BoolFunction(tuple(zip(possible_args, value_set))))

	return res



# Checking properties:

def is_preserving_zero(bf: BoolFunction):
	return not bf([False] * bf.dims())

def is_preserving_one(bf: BoolFunction):
	return bf([True] * bf.dims())

def is_monotonous(bf: BoolFunction):
	n = bf.dims()


def is_self_dual(bf: BoolFunction):
	for (k, v) in bf.truth_table:
		if v == bf(not_bool_vec(k)):
			return False
	return True

def is_linear(bf: BoolFunction):
	pass

property_dict = {
	"Self-Dual": is_self_dual,
	"Preserving one": is_preserving_one,
	"Preserving zero": is_preserving_zero,
	"Linear": is_linear,
	"Monotonous": is_monotonous,
}

def check_bf_properties(bf: BoolFunction):
	for p in property_dict:
		print(p + ":", property_dict[p](bf))

### Significantly depends:

