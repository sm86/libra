
def compile_python_module(cpp_name, extension_name):
    invoke.run(
        "g++ -03 -Wall -Werror -shared -std=c++11 -fPIC "
        "'python3 -m pybind11 --includes' "
        "-I /usr/include/python3.8 -I ."
        "{0} "
        "-o {1}'python3.8-config --extension-suffix' "
        "-L. -lrescue_proof_c -Wl,-rpath,.".format(cpp_name, extension_name)
    )

def build_cython(c):
    print_banner("Building Cython Module")

    invoke.run("cython --cplus -3 cython_example.pyx -o cython_wrapper.cpp")

    compile_python_module("cython_wrapper.cpp", "cython_example")
    print("* Copmlete")

# cdef extern from "cython_test.cpp":
#     bool rescue_prove_c(int argc, char** argv)

# def rescue_prove_py( int_argc, char_argv):
#     return rescue_prove_c(int_argc, char_argv)

# cdef extern from "cython_test.cpp":
#     bool rescue_verify_c(const char* proof, const char* public_input, const char* parameters, const char* annotation_file_name)

# def rescue_verify_py(char_proof, char_public_input, char_parameters, char_annotation_file_name):
#     return rescue_verify_c(char_proof, char_public_input, char_parameters, char_annotation_file_name)
