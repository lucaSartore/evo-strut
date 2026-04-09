from typing import Set
from .interface import Evaluator
from custom_types import Graph, NodeId, Settings, StiffnessResult, Stiffness
import numpy as np
from const import STIFFNESS_MATRIX_OF_GROUND
from typing import TYPE_CHECKING
if TYPE_CHECKING:
    from typing import Any, cast
    ops = cast(Any, None)
else:
    import openseespy.opensees as ops

class OpenSeesEvaluator(Evaluator):

    @staticmethod
    def evaluate(graph: Graph, settings: Settings) -> StiffnessResult:
        return {
            node_id: OpenSeesEvaluator.evaluate_node(graph, node_id, settings)
            for node_id in graph.nodes
        }

    @staticmethod
    def evaluate_node(graph: Graph, node_id: NodeId, settings: Settings) -> Stiffness:
        # Displacements for each degree of freedom
        displacements = []
        for load_case in OpenSeesEvaluator.get_load_cases():
            displacements.append(OpenSeesEvaluator.get_displacement(graph, node_id, settings, *load_case))

        # Construct the displacement matrix
        m = np.array(displacements).T

        if (m == 0).all():
            m = STIFFNESS_MATRIX_OF_GROUND
        else:
            m = np.linalg.inv(m)

        return m

    @staticmethod
    def get_displacement(graph: Graph, node_id: NodeId, settings: Settings, fx: float, fy: float, fz: float, mx: float, my: float, mz: float) -> tuple[float, float, float, float, float, float]:
        OpenSeesEvaluator.build_simulator(graph, settings)

        # wrench we want to apply
        wrench = [fx, fy, fz, mx, my, mz]

        ops.timeSeries("Linear", 1)
        ops.pattern("Plain", 1, 1)

        # Create the nodal load - command: load nodeID xForce yForce
        ops.load(node_id, *wrench)

        ops.system("BandSPD")
        ops.numberer("RCM")
        ops.constraints("Plain")
        ops.integrator("LoadControl", 1.0)
        ops.algorithm("Linear")
        ops.analysis("Static")

        ops.analyze(1)

        disp = ops.nodeDisp(node_id)
        to_return =  tuple([float(x) for x in disp])
        assert len(to_return) == 6
        return to_return

    @staticmethod
    def build_simulator(graph: Graph, settings: Settings):

        ops.wipe()
        ops.model('basic', '-ndm', 3, '-ndf', 6)

        # Add nodes
        for node_id, node in graph.nodes.items():
            ops.node(node.id, *node.position.as_list())
            if node.ground_node:
                # lock all 6 degrees of freedom
                ops.fix(node.id, 1, 1, 1, 1, 1, 1)

        ops.geomTransf("Linear", 1, *[1.0, 0.0, 0.0])

        elementId = 1
        for node_id, node in graph.nodes.items():
            for adj in node.adj:
                if adj.id < node_id:  # Avoid duplicating elements
                    ops.element(
                        'elasticBeamColumn',
                        elementId,
                        *[node.id, adj.id],
                        settings.area,
                        settings.e_mod,
                        settings.g_mod,
                        settings.jxx,
                        settings.iy,
                        settings.iz,
                        1
                    )
                    elementId += 1

    @staticmethod
    def get_load_cases():
        # Define unit load cases for each degree of freedom
        return [
            (1.0, 0.0, 0.0, 0.0, 0.0, 0.0),  # Fx
            (0.0, 1.0, 0.0, 0.0, 0.0, 0.0),  # Fy
            (0.0, 0.0, 1.0, 0.0, 0.0, 0.0),  # Fz
            (0.0, 0.0, 0.0, 1.0, 0.0, 0.0),  # Mx
            (0.0, 0.0, 0.0, 0.0, 1.0, 0.0),  # My
            (0.0, 0.0, 0.0, 0.0, 0.0, 1.0),  # Mz
        ]
