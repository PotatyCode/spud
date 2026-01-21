cmake_minimum_required (VERSION 4.2.1)
project(chip) 

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(chip src/main.cpp)
target_link_libraries(chip)
    
