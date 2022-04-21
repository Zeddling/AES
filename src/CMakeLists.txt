set(BINARY ${CMAKE_PROJECT_NAME})

#   GLOB_RECURSE traverses all subdirectories of the matched directory
#   and matches the files.
#   LIST_DIRECTORIES if false, directories are omitted in the result
file(GLOB_RECURSE SOURCES LIST_DIRECTORIES true *.h *.cpp)
set(SOURCES ${SOURCES})
add_executable(${BINARY}_run ${SOURCES})

#   allow code in src to be used as a static link library
#   during testing
add_library(${BINARY}_lib STATIC ${SOURCES})
