cmake_minimum_required(VERSION 2.8)
project(scuti)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")
find_package(rustc)
find_package(rustdoc)
find_package(cargo)
include(Rust)

cargo_dependency(cargo
        PACKAGE_NAMES glutin gl
        PACKAGE_VERSIONS * 0.6.0)

set(RUSTC_FLAGS -L ${CMAKE_BINARY_DIR}/lib -L ${CMAKE_BINARY_DIR}/cargo/target/debug/deps)
set(RUSTDOC_FLAGS -L ${CMAKE_BINARY_DIR}/lib -L ${CMAKE_BINARY_DIR}/cargo/target/debug/deps)

# Get the dependencies of all the crates
get_rust_deps(src/main.rs MAIN_DEPS)

# Build
rust_crate(src/main.rs
        ALL
        TARGET_NAME scuti
        DESTINATION bin
        DEPENDS "${MAIN_DEPS}"
        OTHER_RUSTC_FLAGS --crate-type bin)

#add_custom_target(scuti ALL ${CARGO_EXECUTABLE} build
#        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
#        SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/main.rs)