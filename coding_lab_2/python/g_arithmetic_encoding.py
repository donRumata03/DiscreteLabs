from collections import Counter
from fractions import Fraction
from typing import List, Callable, Optional


class SegmentExclusive:
    def __init__(self, l: Fraction, r: Fraction):
        self.r = r
        self.l = l

    def contains(self, point: Fraction):
        return self.l <= point < self.r


def discrete_binary_search(pred: Callable, l, r):
    """ Pred for l should be False, for r — True, for all — «increasing» """
    while l + 1 < r:
        m = (l + r) // 2

        if pred(m):
            r = m
        else:
            l = m

    return l, r


def arithmetic_encode_to_fraction(fr) -> Fraction:
    pass


def segment_to_smallest_bitstring(segment: SegmentExclusive):
    p, q = segment_to_smallest_fraction(segment)
    valuable_bits = bin(p)[2:]
    return "0" * (q - len(valuable_bits)) + valuable_bits


def segment_to_smallest_fraction(segment: SegmentExclusive) -> (int, int):
    # To find smallest q such that 2^q € [l, r) — use binary search (float log wouldn't work)

    # For q there exists proper fraction => for q' > q — it exists, too
    # Inside q we use bs again to get p if it exists

    # So, it's O(log^2)

    def p_is_big_enough_for_q(p, q):
        # Check that p/2^q >= l
        val = Fraction(p, 2 ** q)
        return val >= segment.l

    def try_get_p_for_q(q) -> Optional[int]:
        # Use bin search by p:
        # p > -1
        # p < 2 ** q
        possible_p = discrete_binary_search(lambda p: p_is_big_enough_for_q(p, q), -1, 2 ** q)[1]
        return possible_p if segment.contains(Fraction(possible_p, 2 ** q)) else None

    def q_is_ok(q):
        return try_get_p_for_q(q) is not None


    # Upper bound:
    too_big_q = 3
    while not q_is_ok(too_big_q):
        too_big_q *= 2
    too_big_q *= 2

    # Finally, get q here:

    final_q = discrete_binary_search(q_is_ok, -1, too_big_q)[1]

    return try_get_p_for_q(final_q), final_q



def construct_interval_seq(l: Fraction, r: Fraction, fr: List[int]) -> List[SegmentExclusive]:
    res = []
    atomic_step = Fraction(r - l, sum(fr))

    current_border = l
    for f in fr:
        next_border = current_border + atomic_step * f
        res.append(SegmentExclusive(current_border, next_border))
        current_border = next_border

    return res


# print(segment_to_smallest_bitstring(SegmentExclusive(
#     Fraction(9, 10),
#     Fraction(95, 100)
# )))


n = int(input())

input_chars = list(map(lambda c: ord(c) - ord('a'), input()))

c = Counter(input_chars)
freqs = [c[i] for i in range(n)]




"""

3
abacaba

"""