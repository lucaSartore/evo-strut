from typing import TYPE_CHECKING
from evaluation.bottom_up_evaluator import BottomUpEvaluator
from evaluation.dag_evaluator import DagEvaluator
from evaluation.interface import Evaluator
from evaluation.opensees_evaluator import OpenSeesEvaluator
from evaluation.util import calculate_accuracy, calculate_beam_stiffness
from evaluation.weighted_dag_evaluator import WeightedDagEvaluator
from testcases import *
from visualize import Visualizer
from collections import defaultdict
import numpy as np

def main():
    import numpy as np
    struct = load_pillars();
    settings = Settings()
    gt = OpenSeesEvaluator.evaluate(struct, settings)
    calc = BottomUpEvaluator.evaluate(struct, settings)
    v = Visualizer(struct)
    v.add_stiffness_visualization(gt, "green", "ground_through")
    v.add_stiffness_visualization(calc, "red", "approximation")
    v.plot("test")
    # node = 8
    # # fn = lambda x: (np.linalg.inv(x[node]) * 1000000).astype(np.int32)
    # fn = lambda x: x[node].astype(np.int32)
    # result = fn(gt)
    # approx = fn(calc)
    # print("actual stiffness")
    # print(result)
    # print("approximated stiffness)")
    # print(approx)
    # print("error")
    # print(result - approx)


if __name__ == "__main__":
    main()
