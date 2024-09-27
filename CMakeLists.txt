cmake_minimum_required(VERSION 3.10)

# Set the project name
project(DataExample)

# Specify C++ standard
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Add executable
add_executable(DataExample main.cc data.cc mydata.cc)

add_library(data SHARED data.cc mydata.cc)