# sprs_suitesparse_cholmod
SuiteSparse's CHOLMOD bindings for the SPRS crate. Provides bindings for solving Ax=b using LL' Cholesky factorization, which is faster (although less numerically stable) than sprs's LDL' solver for large matrices.
