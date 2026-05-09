cmake_minimum_required(VERSION 3.10) 

project(OpenVDBTest VERSION 1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# this is necessary, as OpenVDB does not provide a "openvdb-config.cmake" file
# https://github.com/AcademySoftwareFoundation/openvdb/issues/1160
# the variable must be set to a path containing FindOpenVDB.cmake 
# (if you are using the flake, this will be set automatically)
list(APPEND CMAKE_MODULE_PATH "$ENV{OpenVDB_DIR}")
find_package(OpenVDB REQUIRED)
find_package(TBB REQUIRED)

add_executable(${PROJECT_NAME} main.cpp)

target_link_libraries(${PROJECT_NAME} PRIVATE 
  ${OpenVDB_LIBRARIES}
  TBB::tbb
)
