
cdef extern from "cython_test.cpp":
    bool rescue_prove_c(int argc, char** argv)

def rescue_prove_py( int_argc, char_argv):
    return rescue_prove_c(int_argc, char_argv)
