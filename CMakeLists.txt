cmake_minimum_required(VERSION 3.1)
project(SudokuSolver)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")
set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake_modules" ${CMAKE_MODULE_PATH})

find_package(SFML 2 REQUIRED system window graphics network audio)
find_package(TGUI 0.6 REQUIRED)

if(NOT SFML_FOUND)
    message(FATAL_ERROR "Could not find SFML")
endif()

if(NOT TGUI_FOUND)
    message(FATAL_ERROR "Could not find TGUI")
endif()



set(SOURCE_FILES main.cpp)
add_executable(SudokuSolver ${SOURCE_FILES})
target_link_libraries(SudokuSolver ${TGUI_LIBRARY} ${SFML_LIBRARIES})
