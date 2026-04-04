from testcases import *
from visualize import Visualizer

def main():
    frame, stiffness = load_struct_D()

    v = Visualizer(frame)
    v.add_stiffness_visualization(stiffness, "red", "ground_truth")
    v.plot("testcase")


if __name__ == "__main__":
    main()
