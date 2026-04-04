from testcases import *
from visualize import Visualizer
import numpy as np
import anastruct as anas
from evaluation.anastruct_evaluator import AnastructEvaluator


def main():
    settings = Settings(1e2, 1e1)
    graph = load_struct_D()

    stiffness = AnastructEvaluator.evaluate(graph, settings)

    v = Visualizer(graph)
    v.add_stiffness_visualization(stiffness, "red", "ground_truth")
    v.plot("testcase")

    # ss = anas.SystemElements(EA=10000, EI=100)
    # ss.add_element(location=[[0, 0], [0, 5]])
    # # ss.add_element(location=[[0, 5], [5, 5]])
    # # ss.add_element(location=[[5, 5], [5, 0]])
    # # ss.add_element(location=[[0, 0], [5, 5]])
    #
    # ss.add_support_fixed(node_id=1)
    # # ss.add_support_fixed(node_id=4)
    # ss.point_load(Fx=3, node_id=2)
    #
    # displacement = ss.solve()
    #
    # print(displacement)
    # ss.show_displacement(factor=1)


if __name__ == "__main__":
    main()
