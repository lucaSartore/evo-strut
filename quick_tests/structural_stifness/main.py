from evaluation.bottom_up_evaluator import BottomUpEvaluator
from testcases import *
from visualize import Visualizer
from evaluation.anastruct_evaluator import AnastructEvaluator

def main():
    settings = Settings(100, 10)
    # graph = load_triangle()
    # graph = load_lines()
    graph = load_pillar()
    # graph = load_struct_D()

    stiffness = AnastructEvaluator.evaluate(graph, settings)
    stiffness_2 = BottomUpEvaluator.evaluate(graph, settings)

    v = Visualizer(graph)

    v.add_stiffness_visualization(stiffness, "red", "ground_truth")
    v.add_stiffness_visualization(stiffness_2, "green", "bottom_up")

    v.plot("testcase")


if __name__ == "__main__":
    main()
