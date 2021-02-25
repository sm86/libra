#include <string>

// ---------------------------- ethS ---------------------------- //

// --- librescue_prover.a
extern bool rescue_prove(int argc, char** argv);
extern "C" bool rescue_prove_c(int argc, char** argv) {
    return rescue_prove(argc, argv);
}

// --- librescue_verifier.a
extern bool rescue_verify(
    std::string const& proof,
    std::string const& public_input,
    std::string const& parameters,
    std::string const& annotation_file_name
);
extern "C" bool rescue_verify_c(
    const char* proof,
    const char* public_input,
    const char* parameters,
    const char* annotation_file_name
) {
    return rescue_verify(proof, public_input, parameters, annotation_file_name);
}

// ---------------------------- ethS end ----------------------- //

// --- libtest.a
extern void f1(std::string const& str, int reps);
extern "C" void f1_c(const char* name, int reps) {
    f1(name, reps);
}