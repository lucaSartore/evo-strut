from typing import Tuple

from const import STIFFNESS_MATRIX_OF_GROUND
from evaluation.util import calculate_stiffness
from .interface import Evaluator
from custom_types import Graph, NodeId, Point, Settings, Stiffness

type GraphDag = dict[NodeId, list[Tuple[NodeId, float]]]

class HeuristicEvaluator(Evaluator):

    @staticmethod
    def evaluate_from_dag(graph: Graph, node_id: NodeId, settings: Settings, dag: GraphDag) -> Stiffness:
        """
        evaluate one single node starting from a Direct Acyclic Graph of the dependency
        that support every other node (with optionally weight for each dependency)
        """

        node = graph.nodes[node_id]
        if node.ground_node:
            return STIFFNESS_MATRIX_OF_GROUND

        supports: list[tuple[Point, Stiffness]] = []

        for (support_id, support_weight) in dag[node_id]:
            support_position = graph.nodes[support_id].position
            support_stiffness = HeuristicEvaluator.evaluate_from_dag(graph, support_id, settings, dag)
            supports.append((support_position, support_stiffness * support_weight))

        assert supports.count != 0
        return calculate_stiffness(node.position, supports, settings)


