from BF_lib.bool_function_properties import function_set_is_basis
from BF_lib.linear_zhegalkin_polynomial import *



if __name__ == '__main__':
	functions = []

	n = int(input())
	for i in range(n):
		functions.append(BoolFunction(list(map(lambda c: bool(int(c)), input().split(" ")[1]))))

	print("YES" if function_set_is_basis(functions) else "NO")

