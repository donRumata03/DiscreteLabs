from bool_functions import *

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



if __name__ == '__main__':
    p1 = BoolFunction(bool_for_p1)
    pierce_arrow = BoolFunction(bool_for_pierce_arrow)

    print("Projector(1): ")
    check_bf_properties(p1)

    print("\n\nPierce Arrow: ")
    check_bf_properties(pierce_arrow)

    fs = get_all_functions(4)

    # for f in fs:
    #     print(f)
