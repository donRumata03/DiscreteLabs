from BF_lib.bool_function_properties import *

for_p1 = (
	((0, 0), 0),
	((0, 1), 0),
	((1, 0), 1),
	((1, 1), 1)
)
bool_for_p1 = int_table_to_bool(for_p1)

for_pierce_arrow = (
	((0, 0), 1),
	((0, 1), 0),
	((1, 0), 0),
	((1, 1), 0)
)
bool_for_pierce_arrow = int_table_to_bool(for_pierce_arrow)


def prop_check_sample():
	p1 = BoolFunction(bool_for_p1)
	pierce_arrow = BoolFunction(bool_for_pierce_arrow)

	print("Projector(1): ")
	check_bf_properties(p1)

	print("\n\nPierce Arrow: ")
	check_bf_properties(pierce_arrow)

def mass_check():
	functions = get_all_functions(2)

	for f in functions:
		print(str(f) + ": ")
		check_bf_properties(f)
		print()


def depends_on_at_least_three_not_lies_in_any():
	fs = get_all_functions(3)

	good_fs = [
		f for f in fs if significantly_depends_on_at_least(f, 3) and not any([cl_checker(f) for cl_checker in post_class_checkers])
	]

	print("Found:", len(good_fs))
	for f in good_fs:
		print(repr(f))


def depends_on_at_least_x_lies_in_all(x, collect_down_from=4):
	fs = sum([get_all_functions(i) for i in range(collect_down_from)], [])

	good_fs = [
		f for f in fs if significantly_depends_on_at_least(f, x) and all([cl_checker(f) for cl_checker in post_class_checkers])
	]

	print("Found:", len(good_fs))
	for f in good_fs:
		print(repr(f))


def only_in_x_class():
	fs = get_all_functions(3)

	for cls_name, cls_checker in post_properties:
		print(cls_name, ":")
		other_class_checkers = [other_cls[1] for other_cls in post_properties if other_cls[0] != cls_name]
		succ = False
		for f in fs:
			if cls_checker(f) and not any([c(f) for c in other_class_checkers]):
				print(repr(f))
				succ = True
				break
		if succ:
			continue
		print("Fail...")
		for f in fs:
			if cls_checker(f) and sum([c(f) for c in other_class_checkers]) == 1 and not is_preserving_one(f) and not is_preserving_zero(f):
				print("Almost: ", repr(f))
				for name, ch in post_properties:
					if name != cls_name and ch(f):
						print(name)


	print("ZP for s-dual:")
	sdf = BoolFunction((((False, False, False), True), ((False, False, True), False), ((False, True, False), False), ((False, True, True), False), ((True, False, False), True), ((True, False, True), True), ((True, True, False), True), ((True, True, True), False)))
	print(build_linear(sdf))


def not_lies_only_in_x_class():
	fs = get_all_functions(3)

	for cls_name, cls_checker in post_properties:
		print(cls_name, ":")
		other_class_checkers = [other_cls[1] for other_cls in post_properties if other_cls[0] != cls_name]
		succ = False
		for f in fs:
			if not cls_checker(f) and all([c(f) for c in other_class_checkers]):
				print(repr(f))
				succ = True
				break


def how_much_dep_on_all(n):
	fs = get_all_functions(n)
	return sum([1 for f in fs if significantly_depends_on_all(f)])


def check_dep_on_all_amount_dynamic():
	for i in range(5):
		print(i, how_much_dep_on_all(i))


def lin_mon():
	fs = get_all_functions(4)

	for f in fs:
		if is_linear(f) and is_monotonous(f):
			print(build_linear(f))


def play_games_with_bool_functions():
	# print(bin_vec_to_mask(inverse_bin_vec(to_bin_vec(1, 2))))

	mass_check()
	# not_lies_only_in_x_class()
	# only_in_x_class()
	# depends_on_at_least_three_not_lies_in_any()
	# depends_on_at_least_x_lies_in_all(1)
	# lin_mon()

	# check_dep_on_all_amount_dynamic()

	# 2 ** (2 ** i) -

if __name__ == '__main__':
	play_games_with_bool_functions()

