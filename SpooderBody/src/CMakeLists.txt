cmake_minimum_required(VERSION 3.10)
project(Spooder)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# explicit source list
set(SPOODER_SOURCES
  src/main.cpp
  src/leg_servo.cpp
)

add_executable(Spooder ${SPOODER_SOURCES})

# make headers visible to the compiler/IntelliSense
target_include_directories(Spooder PRIVATE ${CMAKE_SOURCE_DIR}/lib)

# optional: add headers so IDEs show them in the project tree
target_sources(Spooder PRIVATE
  ${CMAKE_SOURCE_DIR}/lib/leg_servo.h
)