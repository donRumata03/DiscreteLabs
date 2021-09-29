from bool_functions.bool_functions import *

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


if __name__ == '__main__':
    # mass_check()
    fs = get_all_functions(3)

    good_fs = [
        f for f in fs if significantly_depends_on_at_least(f, 3) and not any([cl_checker(f) for cl_checker in post_class_checkers])
    ]

    print("Found:", len(good_fs))
    for f in good_fs:
        print(f)
