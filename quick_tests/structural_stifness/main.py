from testcases import *
from visualize import Visualizer

def main():
    frame, stiffness = load_triangle_frame()

    v = Visualizer(frame)
    v.add_stiffness_visualization(stiffness, "red", "ground_truth")
    v.plot("testcase")


if __name__ == "__main__":
    main()
