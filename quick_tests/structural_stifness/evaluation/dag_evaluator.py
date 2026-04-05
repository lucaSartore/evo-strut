from collections import deque
from evaluation.heuristic_evaluator import GraphDag, HeuristicEvaluator
from custom_types import Graph, NodeId, Settings, Stiffness, StiffnessResult

class DagEvaluator(HeuristicEvaluator):

    @staticmethod
    def evaluate(graph: Graph, settings: Settings) -> StiffnessResult:
        return {
            node: DagEvaluator.evaluate_one_node(graph, settings, node)
            for node in graph.nodes
        }

    @staticmethod
    def evaluate_one_node(graph: Graph, settings: Settings, node_id: NodeId) -> Stiffness:
        dag: GraphDag = {}

        dag = {node: [] for node in graph.nodes}

        visited: set[NodeId] = set()
        to_visit = deque([node_id])

        while len(to_visit) != 0:
            node = to_visit.popleft()

            if node in visited:
                continue

            visited.add(node)

            for adj in graph.nodes[node].adj:
                if adj.id not in visited:
                    dag[node].append((adj.id, 1.0))
                    to_visit.append(adj.id)

        return DagEvaluator.evaluate_from_dag(graph, node_id, settings, dag)
