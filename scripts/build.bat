@echo off
setlocal enabledelayedexpansion

REM Fetch submodules
echo Fetching submodules...
git submodule update --init --recursive


REM Navigate to the libcpuid directory
echo Changing directory to libcpuid...
cd libcpuid

REM Run CMake configuration
echo Running CMake configuration...
cmake -S . -B build -DCMAKE_BUILD_TYPE=RelWithDebInfo -DLIBCPUID_ENABLE_TESTS=ON
if errorlevel 1 (
    echo CMake configuration failed. Exiting.
    exit /b 1
)

REM Build the project
echo Building the project...
cmake --build build
if errorlevel 1 (
    echo Build failed. Exiting.
    exit /b 1
)


echo Installing the build...
cmake --install build

echo Script completed.
exit /b 0
