from dataclasses import dataclass

@dataclass
class ElementarySample:
	value: float


class ProbabilityDistribution:
	def __init__(self, samples):
		self.samples = samples

	def expected_value(self):
		return sum(self.samples) / len(self.samples)

	def dispersion(self):
		e = self.expected_value()
		return ProbabilityDistribution([(s - e)**2 for s in self.samples])\
			.expected_value()

	def plot(self, plotter):
		plotter.plot()


if __name__ == '__main__':
	pass