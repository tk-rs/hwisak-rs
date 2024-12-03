cmake -S . -B build -DCMAKE_BUILD_TYPE=RelWithDebInfo -DLIBCPUID_ENABLE_TESTS=ON
cmake --build build
cmake --install build