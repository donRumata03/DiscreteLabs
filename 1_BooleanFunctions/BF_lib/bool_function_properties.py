from BF_lib.BoolFunction import *
from BF_lib.linear_zhegalkin_polynomial import *
from itertools import product


def get_all_functions(n: int) -> List[BoolFunction]:
	possible_args = tuple(product((False, True), repeat=n))
	possible_function_values = list(product((False, True), repeat=len(possible_args)))

	res = []
	for value_set in possible_function_values:
		res.append(BoolFunction(tuple(zip(possible_args, value_set))))

	return res


# Checking lying Post classes:
def is_preserving_zero(bf: BoolFunction):
	return not bf([False] * bf.dims())


def is_preserving_one(bf: BoolFunction):
	return bf([True] * bf.dims())


def is_monotonous(bf: BoolFunction):
	n = bf.dims()

	for (k, v) in bf.truth_table:
		for i in range(n):
			if not k[i]:
				dominator = list(k)
				dominator[i] = True
				if bf(tuple(dominator)) < v:
					return False

	return True


def is_self_dual(bf: BoolFunction):
	for (k, v) in bf.truth_table:
		if v == bf(inverse_bin_vec(k)):
			return False
	return True


def is_linear(bf: BoolFunction):
	n = bf.dims()
	a_0 = bf([False] * n)
	a_s = [a_0 ^ bf([False] * i + [True] + [False] * (n - i - 1)) for i in range(n)]
	pz = LinearZhegalkinPolynomial(a_0, a_s)

	for p in bf.truth_table:
		if pz(p[0]) != p[1]:
			return False

	return True


property_dict = [
	("Preserving zero", is_preserving_zero),
	("Preserving one", is_preserving_one),
	("Self-Dual", is_self_dual),
	("Monotonous", is_monotonous),
	("Linear", is_linear),
]
post_class_checkers = [
	p[1] for p in property_dict
]

def check_bf_properties(bf: BoolFunction):
	for p in property_dict + [("Significantly depends on all args", significantly_depends_on_all)]:
		print(p[0] + ":", int(p[1](bf)))

### Significantly depends:


def significantly_depends(bf: BoolFunction, arg_index: int):
	for (k, v) in bf.truth_table:
		alter_vector = list(k)[:]
		alter_vector[arg_index] = not k[arg_index]
		if bf(alter_vector) != v:
			return True
	return False

def significantly_depends_on_at_least(bf: BoolFunction, at_least: int):
	return sum([significantly_depends(bf, i) for i in range(bf.dims())]) >= at_least

def significantly_depends_on_all(bf: BoolFunction):
	return significantly_depends_on_at_least(bf, bf.dims())

