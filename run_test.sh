#!/run/current-system/sw/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# 1. Take the first argument ($1) as the test name. 
# If not provided, use test_{current_time}
if [ -z "$1" ]; then
    CURRENT_TIME=$(date +"%Y%m%d_%H%M%S")
    TEST_NAME="test_${CURRENT_TIME}"
else
    TEST_NAME="$1"
fi

TARGET_DIR="./test_results/${TEST_NAME}"

# 2. Make a directory with the test name inside ./test_results 
# (if the file/dir already exists => print error and stop)
if [ -e "$TARGET_DIR" ]; then
    echo "Error: Directory or file '${TARGET_DIR}' already exists." >&2
    exit 1
fi

mkdir -p "$TARGET_DIR"

# 3. Remove everything that is inside ./optimization_logs
# (Creates it if it doesn't exist, or clears it safely if it does)
if [ -d "./optimization_logs" ]; then
    rm -rf ./optimization_logs/*
else
    mkdir -p "./optimization_logs"
fi

# 4. Run cargo run --release
echo "Running build and execution via Cargo..."
cargo run --release

# 5. Copy every file inside optimization_logs into the dir you created 
# (in a sub folder "optimization_logs")
if [ -d "./optimization_logs" ] && [ "$(ls -A ./optimization_logs)" ]; then
    mkdir -p "${TARGET_DIR}/optimization_logs"
    cp -r ./optimization_logs/* "${TARGET_DIR}/optimization_logs/"
else
    echo "Warning: ./optimization_logs was empty or did not exist after cargo run."
fi

# 6. Copy specific mesh and settings files inside the folder of the test
echo "Copying build artifacts..."

# Array of specific files to copy
FILES_TO_COPY=(
    "test_meshes/output.json"
    "test_meshes/output.stl"
    "test_meshes/settings.json"
)

for FILE in "${FILES_TO_COPY[@]}"; do
    if

