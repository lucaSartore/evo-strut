from typing import DefaultDict
from evaluation.heuristic_evaluator import GraphDag, HeuristicEvaluator
from custom_types import Graph, NodeId, Point, Settings, Stiffness, StiffnessResult
from queue import PriorityQueue

class WeightedDagEvaluator(HeuristicEvaluator):

    @staticmethod
    def evaluate(graph: Graph, settings: Settings) -> StiffnessResult:
        return {
            node: WeightedDagEvaluator.evaluate_one_node(graph, settings, node)
            for node in graph.nodes
        }

    @staticmethod
    def evaluate_one_node(graph: Graph, settings: Settings, node_id: NodeId) -> Stiffness:
        dag: GraphDag = {}

        dag = {node: [] for node in graph.nodes}
        node_to_incoming_arches: dict[NodeId, list[tuple[NodeId, float]]] = DefaultDict(lambda: [])

        visited: set[NodeId] = set()
        to_visit: PriorityQueue[tuple[float, NodeId]] = PriorityQueue()
        to_visit.put((0.0, node_id))

        while not to_visit.empty():
            (distance, node) = to_visit.get()

            if node in visited:
                continue

            visited.add(node)

            for adj in graph.nodes[node].adj:
                if adj.id not in visited:
                    arch_len = Point.distance(adj.position, graph.nodes[node].position)
                    adj_distance = distance + arch_len
                    dag[node].append((adj.id, 1.0))
                    to_visit.put((adj_distance, adj.id))
                    node_to_incoming_arches[adj.id].append((node, adj_distance))

        for node, incoming_arches in node_to_incoming_arches.items():
            # we only have to re-weight nodes that have multiple incoming arches
            if len(incoming_arches) <= 1:
                continue

            new_weights = WeightedDagEvaluator.distances_to_weights(incoming_arches)

            for node_from, new_weight in new_weights:
                dag[node_from].remove((node, 1.0))
                dag[node_from].append((node, new_weight))

        return WeightedDagEvaluator.evaluate_from_dag(graph, node_id, settings, dag)


    @staticmethod
    def distances_to_weights(distances: list[tuple[NodeId, float]]) -> list[tuple[NodeId,float]]:
        inverse_distances = [1/d for _,d in distances]
        total_inverse_distance = sum(inverse_distances)
        weights = [i/total_inverse_distance for i in inverse_distances]
        return [(id, weight) for (id,_), weight in zip(distances, weights)]
