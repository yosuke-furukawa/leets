from collections import Counter
import math


class Solution:
    totalSize = 0

    # Space:    O(n)
    # Time:     O(n^2 log n)
    def calculateMaxInfoGain(self, petal_length: List[float], species: List[str]) -> float:
        if petal_length and species:
            self.totalSize = len(petal_length)

            irisFlowers = sorted(zip(petal_length, species))
            sortedSpecies = [specie for (_, specie) in irisFlowers]
            overallEntropy = self.calculateEntropy(sortedSpecies)
            minSum = float("inf")

            for i in range(1, self.totalSize):
                entropySum = self.calculateEntropy(sortedSpecies[:i]) + self.calculateEntropy(sortedSpecies[i:])
                if entropySum < minSum: minSum = entropySum

            return overallEntropy - minSum
        return 0.0

    # Space:    O(n)
    # Time:     O(n log n)
    def calculateEntropy(self, input: List[str]) -> float:
        size, counter, entropy, logDict = len(input), Counter(input), 0.0, {}

        for num in counter:
            probability = counter[num] / size
            if not probability in logDict:
                logDict[probability] = math.log(probability, 2)
            entropy -= probability * logDict[probability]

        contributeRatio = size / self.totalSize

        return entropy * contributeRatio
