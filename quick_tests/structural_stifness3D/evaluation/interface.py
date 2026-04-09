from abc import ABC, abstractmethod
from custom_types import Graph, Settings, StiffnessResult


class Evaluator(ABC):
    @staticmethod
    @abstractmethod
    def evaluate(graph: Graph, settings: Settings) -> StiffnessResult:
        pass

