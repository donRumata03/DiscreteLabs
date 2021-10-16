def get_false():
	return "(((A0|A0)|A0)|((A0|A0)|A0))"

def get_true():
	return f"({get_false()}|{get_false()})"

def get_not(clause):
	return f"({clause}|{get_true()})"

def get_and(c1, c2):
	return get_not(f"({c1}|{c2})")

def get_or(c1, c2):
	return get_not(get_and(get_not(c1), get_not(c2)))



def get_primitive_and(c1, c2):
	return f"(({c1}|{c2})|({c1}|{c2})"

def get_primitive_not(c1):
	return f"({c1}|{c1})"

def get_primitive_or(c1, c2):
	return get_primitive_not(get_primitive_and(get_primitive_not(c1), get_primitive_not(c2)))


def get_overflow_bit(n):
	if n == 1:
		return "((A0|B0)|(A0|B0))"

	bi = n - 1
	a = f"A{bi}"
	b = f"B{bi}"

	# this_conjunction = get_primitive_and(f"A{bi}", f"B{bi}")
	# this_disjunction = get_primitive_or(f"A{bi}", f"B{bi}")
	#
	# return get_or(this_conjunction, get_and(get_overflow_bit(n - 1), this_disjunction))

	return f"(({get_overflow_bit(n - 1)}|(({a}|{a})|({b}|{b})))|({a}|{b}))"


N = int(input())
res = get_overflow_bit(N)
# print(f"Length: {len(res)}, max: {N * 50}")
print(res)

exit()
xs = list(range(1, 100))
ys = [len(get_overflow_bit(i)) for i in xs]

from matplotlib import pyplot as plt
plt.plot(xs, ys)
plt.show()

# ((((A0|B0)|(A0|B0))|((A1|A1)|(B1|B1)))|(A1|B1))
