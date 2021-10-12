from BF_lib.zhegalkin_polynomial import *


n = int(input())
truth_table = []
for i in range(2 ** n):
	truth_table.append(bool(int(input().split()[-1])))

b = BoolFunction(truth_table)
# print("Input:", b)

z = mobius_transform(b)
# print(f"Output: {z}")

for bm in range(2 ** n):
	print("".join(map(lambda b: str(int(b)), to_bin_vec(bm, n))), int(z.at(bm)))

