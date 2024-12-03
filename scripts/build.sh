# Exit on error
set -e

cd ..

# Fetch submodules
echo "Fetching submodules..."
git submodule update --init --recursive

# Navigate to the libcpuid directory
echo "Changing directory to libcpuid..."
cd libcpuid

# Run CMake configuration
echo "Running CMake configuration..."
cmake -S . -B build -DCMAKE_BUILD_TYPE=RelWithDebInfo -DLIBCPUID_ENABLE_TESTS=ON

# Build the project
echo "Building the project..."
cmake --build build

echo "Installing the build..."
cmake --install build

echo "Script completed."
