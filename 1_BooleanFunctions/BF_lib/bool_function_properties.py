from BF_lib.BoolFunction import *
from BF_lib.linear_zhegalkin_polynomial import *
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
			print(f"All functions are in {p_name} class!")
			return False

	return True



if __name__ == '__main__':
	for fn in get_all_functions(2):
		print(fn)
