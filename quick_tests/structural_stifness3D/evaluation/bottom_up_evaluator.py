from evaluation.heuristic_evaluator import GraphDag, HeuristicEvaluator
from custom_types import Graph, Settings, StiffnessResult

class BottomUpEvaluator(HeuristicEvaluator):

    @staticmethod
    def evaluate(graph: Graph, settings: Settings) -> StiffnessResult:
        dag: GraphDag = {}

        for node in graph.nodes.values():
            dag[node.id] = []
            for adj in node.adj:
                lower = adj.position.y < node.position.y
                same_hight_lower_id = adj.position.y == node.position.y and adj.id < node.id
                is_ground_node = adj.ground_node
                if lower or same_hight_lower_id or is_ground_node:
                    dag[node.id].append((adj.id, 1.0))

        return {
            node: BottomUpEvaluator.evaluate_from_dag(graph, node, settings, dag)
            for node in graph.nodes
        }
