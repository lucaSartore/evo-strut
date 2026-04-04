from typing import Set
from .interface import Evaluator
from custom_types import Graph, NodeId, Settings, StiffnessResult, Stiffness
import anastruct as anas
import numpy as np

class AnastructEvaluator(Evaluator):

    @staticmethod
    def evaluate(graph: Graph, settings: Settings) -> StiffnessResult:
        return {
            node_id: AnastructEvaluator.evaluate_node(graph, node_id, settings)
            for node_id in graph.nodes
        }

    @staticmethod
    def evaluate_node(graph: Graph, node_id: NodeId, settings: Settings) -> Stiffness:
        # displacement we get by applying a force on the x axis
        [dx_x, dy_x] = AnastructEvaluator.get_displacement(graph, node_id, settings, 1.0, 0);
        # displacement we get by applying a force on the y axis
        [dx_y, dy_y] = AnastructEvaluator.get_displacement(graph, node_id, settings, 0.0, 1.0);
        m = np.asarray([[dx_x, dx_y], [dy_x, dy_y]])
        if (m == 0).all():
            m = np.asarray([[10e10, 0], [0, 10e10]])
        else:
            m = np.linalg.inv(m)
        # print(f"matrix for node {node_id} is {m}")
        return m


    @staticmethod
    def get_displacement(graph: Graph, node_id: NodeId, settings: Settings, fx: float, fy: float) -> tuple[float, float]:
        ss = AnastructEvaluator.build_simulator(graph, settings)
        id = ss.find_node_id(graph.nodes[node_id].position.as_list());
        assert id != None
        ss.point_load(Fx=fx, Fy = fy, node_id=id)
        displacement = ss.solve()
        dx = displacement[(id-1) * 3]
        dy = -displacement[(id-1) * 3 + 1]
        print(dx, dy)
        ss.show_structure()
        ss.show_displacement()
        return (float(dx), float(dy))


    @staticmethod
    def build_simulator(graph: Graph, settings: Settings) -> anas.SystemElements:
        ss = anas.SystemElements(EA=settings.ea, EI=settings.ei)
        ids = [x for x in graph.nodes.keys()]
        ids.sort()

        for id in ids:
            node = graph.nodes[id]
            for adj in node.adj:
                id = ss.add_element(location=[node.position.as_list(), adj.position.as_list()])

        for node in graph.nodes.values():
            if node.ground_node:
                id = ss.find_node_id(node.position.as_list())
                assert id != None
                ss.add_support_fixed(id)

        return ss
