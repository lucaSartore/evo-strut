from typing import TYPE_CHECKING
from evaluation.bottom_up_evaluator import BottomUpEvaluator
from evaluation.dag_evaluator import DagEvaluator
from evaluation.interface import Evaluator
from evaluation.opensees_evaluator import OpenSeesEvaluator
from evaluation.util import calculate_accuracy
from evaluation.weighted_dag_evaluator import WeightedDagEvaluator
from testcases import *
from visualize import Visualizer
from collections import defaultdict
import numpy as np

def main():
    import numpy as np
    struct = load_struct_A();
    settings = Settings()
    results = OpenSeesEvaluator.evaluate(struct, settings)
    m = np.linalg.inv(results[6])
    print(m)


if __name__ == "__main__":
    main()
