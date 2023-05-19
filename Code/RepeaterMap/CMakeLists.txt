cmake_minimum_required(VERSION 3.17)
project(RepeaterMap
  VERSION 1.0
  DESCRIPTION "A Mapping tool that gives the user repeaters along a route"
  LANGUAGES CXX)

add_executable(add
  src/main.cpp
  src/add.cpp)

