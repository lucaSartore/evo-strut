from .interface import Evaluator
from custom_types import Graph, Node, Point, Settings, StiffnessResult, Stiffness
from evaluation.util import calculate_stiffness
from const import STIFFNESS_MATRIX_OF_GROUND

class BottomUpEvaluator(Evaluator):

    @staticmethod
    def evaluate(graph: Graph, settings: Settings) -> StiffnessResult:

        to_return: StiffnessResult = {}

        nodes: list[Node] = []
        for node in graph.nodes.values():
            if node.ground_node:
                to_return[node.id] = STIFFNESS_MATRIX_OF_GROUND
            else:
                nodes.append(node)

        nodes.sort(key=lambda v: v.position.y)

        for node in nodes:
            s = BottomUpEvaluator.evaluate_node(node, to_return, settings)
            # print(f"final stiffness {s}")
            to_return[node.id] = s

        return to_return

    @staticmethod
    def evaluate_node(node: Node, stiffness_result: StiffnessResult, settings: Settings) -> Stiffness:
        supports: list[tuple[Point, Stiffness]] = []

        for adj in node.adj:
            stiffness = stiffness_result.get(adj.id)
            if stiffness is not None:
                supports.append((adj.position, stiffness))

        return calculate_stiffness(node.position, supports, settings)
