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

# Prompt for admin privileges for installation with a 20-second timeout
echo "Do you want to install the build? This may require admin privileges. (default: No)"
read -t 20 -p "Press 'y' or 'Y' to confirm, or wait 20 seconds to skip: " confirm

# Check the user's input
if [[ $? -eq 0 && $confirm =~ ^[Yy]$ ]]; then
    echo "Installing the build..."
    sudo cmake --install build
else
    echo "Installation skipped."
fi

echo "Script completed."
