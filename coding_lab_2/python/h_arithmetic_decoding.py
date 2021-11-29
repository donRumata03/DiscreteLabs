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


def arithmetic_decode_from_fraction(target_fraction, char_freqs) -> str:
    current_segment = SegmentExclusive(Fraction(0), Fraction(1))
    target_len = sum(char_freqs)
    res = []

    for i in range(target_len):
        segments = construct_interval_seq(current_segment, char_freqs)
        current_segment = None

        resultant_si = None
        for (si, s) in enumerate(segments):
            if s.contains(target_fraction):
                current_segment = s
                resultant_si = si

        if current_segment is None:
            raise RuntimeError()

        res.append(resultant_si)

    return res


def construct_interval_seq(segment: SegmentExclusive, fr: List[int]) -> List[SegmentExclusive]:
    res = []
    atomic_step = Fraction(segment.r - segment.l, sum(fr))

    current_border = segment.l
    for f in fr:
        next_border = current_border + atomic_step * f
        res.append(SegmentExclusive(current_border, next_border))
        current_border = next_border

    return res


def frac_from_smallest_bitstring(bitstring: str) -> Fraction:
    q = len(bitstring)
    p = int(bitstring, 2)

    return Fraction(p, 2 ** q)


n = int(input())
freqs = list(map(int, input().split()))
bs = input()

targ_frac = frac_from_smallest_bitstring(bs)
chars = arithmetic_decode_from_fraction(targ_frac, freqs)
print("".join([chr(c + ord('a')) for c in chars]))

"""

3
4 2 1
0110100101
__________
abacaba


- - - - - - - - - - - -

2
0 3
0
_________
bbb


"""