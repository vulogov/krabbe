# Defines the CMake commands/policies
CMAKE_MINIMUM_REQUIRED( VERSION 2.8.5 )

# Set the project name
PROJECT( KRABBE  )

# Make the scripts available in the 'cmake' directory available for the
# 'include()' command, 'find_package()' command.
SET( CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} ${CMAKE_SOURCE_DIR}/cmake )
SET( PYTHON_INCLUDE_DIRS ${PYTHON_INCLUDE_DIRS} ${CMAKE_SOURCE_DIR}/include )


# Include the CMake script UseCython.cmake.  This defines add_cython_module().
# Instruction for use can be found at the top of cmake/UseCython.cmake.
INCLUDE( UseCython )
INCLUDE( krabbe )

# With CMake, a clean separation can be made between the source tree and the
# build tree.  When all source is compiled, as with pure C/C++, the source is
# no-longer needed in the build tree.  However, with pure *.py source, the
# source is processed directly.  To handle this, we reproduce the availability
# of the source files in the build tree.
ADD_CUSTOM_TARGET( ReplicatePythonSourceTree ALL ${CMAKE_COMMAND} -P
  ${CMAKE_CURRENT_SOURCE_DIR}/cmake/ReplicatePythonSourceTree.cmake
  ${CMAKE_CURRENT_BINARY_DIR}
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR} )


INCLUDE_DIRECTORIES( ${CYTHON_CMAKE_EXAMPLE_SOURCE_DIR}/include )

ENABLE_TESTING()
FIND_FILE( NOSETESTS_EXECUTABLE nosetests )
ADD_TEST( nosetests "${NOSETESTS_EXECUTABLE}" -v --with-xunit )


ADD_SUBDIRECTORY( src )







