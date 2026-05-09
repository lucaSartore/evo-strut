to run:

```
# start the shell (path:. is optional, if you don't want to depend on the original git repo)
nix develop path:.

# configure the build do output stuff on the "build" directory
cmake -S . -B build

# build
cmake --build build 

# run the file
./build/openvdb



```
