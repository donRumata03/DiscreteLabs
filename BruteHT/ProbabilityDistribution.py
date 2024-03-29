import random
from dataclasses import dataclass
from typing import Callable

from matplotlib import pyplot as plt

from BruteHT.smoother import count_density


# @dataclass
# class ElementarySample:
# 	value: float


def steps_until_pattern(function: Callable, pattern: list):
	generated = []

	while len(generated) < len(pattern) or generated[-len(pattern):] != pattern:
		generated.append(function())
		assert type(generated[-1]) == type(pattern[0])

	return len(generated)


def fair_coin():
	return random.randint(0, 1)


def unfair_coin(p):
	return lambda: random.random() < p


class ProbabilityDistribution:
	@staticmethod
	def generate(function: Callable, amount):
		return ProbabilityDistribution([function() for _ in range(amount)])

	def __init__(self, samples):
		self.samples = samples

	def expected_value(self):
		return sum(self.samples) / len(self.samples)

	def dispersion(self):
		e = self.expected_value()
		return ProbabilityDistribution([(s - e) ** 2 for s in self.samples]) \
			.expected_value()

	def plot(self, plotter):
		res = count_density(self.samples, 0.03, 200)
		plotter.plot(
			[x for (x, y) in res],
			[y for (x, y) in res]
		)

def plot_generated(f, count):
	p = ProbabilityDistribution.generate(f, count)
	print(p.samples)
	print(f"Expected value: {p.expected_value()}")
	print(f"Dispersion: {p.dispersion()}")
	# p.plot(plt)
	# plt.show()


if __name__ == '__main__':
	plot_generated(lambda: steps_until_pattern(fair_coin, [0, 0 ,0]), 10000)
