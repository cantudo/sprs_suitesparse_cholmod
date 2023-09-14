fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=SuiteSparse");
    let root = std::env::var_os("OUT_DIR").unwrap();
    std::env::set_var("CFLAGS", "-Wno-unused-variable -lblas");


    cc::Build::new()
        .flag("-lblas")
        .flag("-llapack")
        .include("SuiteSparse/SuiteSparse_config")
        // .define("DLONG", None)
        .include("SuiteSparse/COLAMD/Include")
        .file("SuiteSparse/COLAMD/Source/colamd.c")
        .file("SuiteSparse/COLAMD/Source/colamd_l.c")

        .include("SuiteSparse/CCOLAMD/Include")
        .file("SuiteSparse/CCOLAMD/Source/ccolamd.c")
        .file("SuiteSparse/CCOLAMD/Source/ccolamd_l.c")

        .include("SuiteSparse/AMD/Include")
        .file("SuiteSparse/AMD/Source/amd_1.c")
        .file("SuiteSparse/AMD/Source/amd_2.c")
        .file("SuiteSparse/AMD/Source/amd_aat.c")
        .file("SuiteSparse/AMD/Source/amdbar.f")
        .file("SuiteSparse/AMD/Source/amd_control.c")
        .file("SuiteSparse/AMD/Source/amd_defaults.c")
        .file("SuiteSparse/AMD/Source/amd_dump.c")
        .file("SuiteSparse/AMD/Source/amd.f")
        .file("SuiteSparse/AMD/Source/amd_info.c")
        .file("SuiteSparse/AMD/Source/amd_l1.c")
        .file("SuiteSparse/AMD/Source/amd_l2.c")
        .file("SuiteSparse/AMD/Source/amd_l_aat.c")
        .file("SuiteSparse/AMD/Source/amd_l_control.c")
        .file("SuiteSparse/AMD/Source/amd_l_defaults.c")
        .file("SuiteSparse/AMD/Source/amd_l_dump.c")
        .file("SuiteSparse/AMD/Source/amd_l_info.c")
        .file("SuiteSparse/AMD/Source/amd_l_order.c")
        .file("SuiteSparse/AMD/Source/amd_l_postorder.c")
        .file("SuiteSparse/AMD/Source/amd_l_post_tree.c")
        .file("SuiteSparse/AMD/Source/amd_l_preprocess.c")
        .file("SuiteSparse/AMD/Source/amd_l_valid.c")
        .file("SuiteSparse/AMD/Source/amd_order.c")
        .file("SuiteSparse/AMD/Source/amd_postorder.c")
        .file("SuiteSparse/AMD/Source/amd_post_tree.c")
        .file("SuiteSparse/AMD/Source/amd_preprocess.c")
        .file("SuiteSparse/AMD/Source/amd_valid.c")
        .include("SuiteSparse/CAMD/Include")
        .file("SuiteSparse/CAMD/Source/camd_1.c")
        .file("SuiteSparse/CAMD/Source/camd_2.c")
        .file("SuiteSparse/CAMD/Source/camd_aat.c")
        .file("SuiteSparse/CAMD/Source/camd_control.c")
        .file("SuiteSparse/CAMD/Source/camd_defaults.c")
        .file("SuiteSparse/CAMD/Source/camd_dump.c")
        .file("SuiteSparse/CAMD/Source/camd_info.c")
        .file("SuiteSparse/CAMD/Source/camd_l1.c")
        .file("SuiteSparse/CAMD/Source/camd_l2.c")
        .file("SuiteSparse/CAMD/Source/camd_l_aat.c")
        .file("SuiteSparse/CAMD/Source/camd_l_control.c")
        .file("SuiteSparse/CAMD/Source/camd_l_defaults.c")
        .file("SuiteSparse/CAMD/Source/camd_l_dump.c")
        .file("SuiteSparse/CAMD/Source/camd_l_info.c")
        .file("SuiteSparse/CAMD/Source/camd_l_order.c")
        .file("SuiteSparse/CAMD/Source/camd_l_postorder.c")
        .file("SuiteSparse/CAMD/Source/camd_l_preprocess.c")
        .file("SuiteSparse/CAMD/Source/camd_l_valid.c")
        .file("SuiteSparse/CAMD/Source/camd_order.c")
        .file("SuiteSparse/CAMD/Source/camd_postorder.c")
        .file("SuiteSparse/CAMD/Source/camd_preprocess.c")
        .file("SuiteSparse/CAMD/Source/camd_valid.c")
        .include("SuiteSparse/CHOLMOD/SuiteSparse_metis/GKlib")
        .include("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis")
        .include("SuiteSparse/CHOLMOD/SuiteSparse_metis/include")
        .include("SuiteSparse/CHOLMOD/Partition")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/auxapi.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/balance.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/bucketsort.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/checkgraph.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/coarsen.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/compress.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/contig.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/debug.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/fm.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/fortran.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/frename.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/gklib.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/graph.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/initpart.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/kmetis.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/kwayfm.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/kwayrefine.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/mcutil.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/mesh.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/meshpart.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/minconn.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/mincover.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/mmd.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/ometis.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/options.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/parmetis.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/pmetis.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/refine.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/separator.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/sfm.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/srefine.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/stat.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/timing.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/util.c")
        .file("SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis/wspace.c")
        .include("SuiteSparse/CHOLMOD")
        .include("SuiteSparse/CHOLMOD/Include")
        .include("SuiteSparse/COLAMD/Include")
        .file("SuiteSparse/SuiteSparse_config/SuiteSparse_config.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_aat.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_add.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_band.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_change_factor.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_common.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_complex.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_copy.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_dense.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_error.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_factor.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_l_aat.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_l_add.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_l_band.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_l_change_factor.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_l_common.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_l_complex.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_l_copy.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_l_dense.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_l_error.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_l_factor.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_l_memory.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_l_sparse.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_l_transpose.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_l_triplet.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_l_version.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_memory.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_sparse.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_transpose.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_triplet.c")
        .file("SuiteSparse/CHOLMOD/Core/cholmod_version.c")
        // .file("SuiteSparse/CHOLMOD/Core/t_cholmod_change_factor.c")
        // .file("SuiteSparse/CHOLMOD/Core/t_cholmod_dense.c")
        // .file("SuiteSparse/CHOLMOD/Core/t_cholmod_transpose.c")
        // .file("SuiteSparse/CHOLMOD/Core/t_cholmod_triplet.c")
        .file("SuiteSparse/CHOLMOD/Cholesky/cholmod_analyze.c")
        .file("SuiteSparse/CHOLMOD/Cholesky/cholmod_colamd.c")
        .file("SuiteSparse/CHOLMOD/Cholesky/cholmod_etree.c")
        .file("SuiteSparse/CHOLMOD/Cholesky/cholmod_factorize.c")
        .file("SuiteSparse/CHOLMOD/Cholesky/cholmod_l_amd.c")
        .file("SuiteSparse/CHOLMOD/Cholesky/cholmod_l_analyze.c")
        .file("SuiteSparse/CHOLMOD/Cholesky/cholmod_l_colamd.c")
        .file("SuiteSparse/CHOLMOD/Cholesky/cholmod_l_etree.c")
        .file("SuiteSparse/CHOLMOD/Cholesky/cholmod_l_factorize.c")
        .file("SuiteSparse/CHOLMOD/Cholesky/cholmod_l_postorder.c")
        .file("SuiteSparse/CHOLMOD/Cholesky/cholmod_l_rcond.c")
        .file("SuiteSparse/CHOLMOD/Cholesky/cholmod_l_resymbol.c")
        .file("SuiteSparse/CHOLMOD/Cholesky/cholmod_l_rowcolcounts.c")
        .file("SuiteSparse/CHOLMOD/Cholesky/cholmod_l_rowfac.c")
        .file("SuiteSparse/CHOLMOD/Cholesky/cholmod_l_solve.c")
        .file("SuiteSparse/CHOLMOD/Cholesky/cholmod_l_spsolve.c")
        .file("SuiteSparse/CHOLMOD/Cholesky/cholmod_postorder.c")
        .file("SuiteSparse/CHOLMOD/Cholesky/cholmod_rcond.c")
        .file("SuiteSparse/CHOLMOD/Cholesky/cholmod_resymbol.c")
        .file("SuiteSparse/CHOLMOD/Cholesky/cholmod_rowcolcounts.c")
        .file("SuiteSparse/CHOLMOD/Cholesky/cholmod_rowfac.c")
        .file("SuiteSparse/CHOLMOD/Cholesky/cholmod_solve.c")
        .file("SuiteSparse/CHOLMOD/Cholesky/cholmod_spsolve.c")
        // .file("SuiteSparse/CHOLMOD/Cholesky/t_cholmod_lsolve.c")
        // .file("SuiteSparse/CHOLMOD/Cholesky/t_cholmod_ltsolve.c")
        // .file("SuiteSparse/CHOLMOD/Cholesky/t_cholmod_rowfac.c")
        // .file("SuiteSparse/CHOLMOD/Cholesky/t_cholmod_solve.c")
        .file("SuiteSparse/CHOLMOD/Modify/cholmod_rowadd.c")
        .file("SuiteSparse/CHOLMOD/Modify/cholmod_rowdel.c")
        .file("SuiteSparse/CHOLMOD/Modify/cholmod_updown.c")
        // .file("SuiteSparse/CHOLMOD/Modify/t_cholmod_updown.c")
        // .file("SuiteSparse/CHOLMOD/Modify/t_cholmod_updown_numkr.c")
        .file("SuiteSparse/CHOLMOD/Check/cholmod_check.c")
        .file("SuiteSparse/CHOLMOD/Check/cholmod_l_check.c")
        .file("SuiteSparse/CHOLMOD/Check/cholmod_read.c")
        .file("SuiteSparse/CHOLMOD/Check/cholmod_write.c")
        // .file("SuiteSparse/CHOLMOD/MatrixOps/cholmod_l_check.c")
        .file("SuiteSparse/CHOLMOD/MatrixOps/cholmod_drop.c")
        .file("SuiteSparse/CHOLMOD/MatrixOps/cholmod_horzcat.c")
        .file("SuiteSparse/CHOLMOD/MatrixOps/cholmod_norm.c")
        .file("SuiteSparse/CHOLMOD/MatrixOps/cholmod_scale.c")
        .file("SuiteSparse/CHOLMOD/MatrixOps/cholmod_sdmult.c")
        .file("SuiteSparse/CHOLMOD/MatrixOps/cholmod_ssmult.c")
        .file("SuiteSparse/CHOLMOD/MatrixOps/cholmod_submatrix.c")
        .file("SuiteSparse/CHOLMOD/MatrixOps/cholmod_symmetry.c")
        .file("SuiteSparse/CHOLMOD/MatrixOps/cholmod_vertcat.c")
        // .file("SuiteSparse/CHOLMOD/MatrixOps/t_cholmod_sdmult.c")
        .file("SuiteSparse/CHOLMOD/Partition/cholmod_camd.c")
        .file("SuiteSparse/CHOLMOD/Partition/cholmod_ccolamd.c")
        .file("SuiteSparse/CHOLMOD/Partition/cholmod_csymamd.c")
        .file("SuiteSparse/CHOLMOD/Partition/cholmod_l_camd.c")
        .file("SuiteSparse/CHOLMOD/Partition/cholmod_l_ccolamd.c")
        .file("SuiteSparse/CHOLMOD/Partition/cholmod_l_csymamd.c")
        .file("SuiteSparse/CHOLMOD/Partition/cholmod_l_metis.c")
        .file("SuiteSparse/CHOLMOD/Partition/cholmod_l_nesdis.c")
        .file("SuiteSparse/CHOLMOD/Partition/cholmod_metis.c")
        .file("SuiteSparse/CHOLMOD/Partition/cholmod_metis_wrapper.c")
        // .file("SuiteSparse/CHOLMOD/Partition/cholmod_metis_wrapper.h")
        .file("SuiteSparse/CHOLMOD/Partition/cholmod_nesdis.c")
        .file("SuiteSparse/CHOLMOD/Supernodal/cholmod_l_super_numeric.c")
        .file("SuiteSparse/CHOLMOD/Supernodal/cholmod_l_super_solve.c")
        .file("SuiteSparse/CHOLMOD/Supernodal/cholmod_l_super_symbolic.c")
        .file("SuiteSparse/CHOLMOD/Supernodal/cholmod_super_numeric.c")
        .file("SuiteSparse/CHOLMOD/Supernodal/cholmod_super_solve.c")
        .file("SuiteSparse/CHOLMOD/Supernodal/cholmod_super_symbolic.c")
        // .file("SuiteSparse/CHOLMOD/Supernodal/t_cholmod_super_numeric.c")
        // .file("SuiteSparse/CHOLMOD/Supernodal/t_cholmod_super_solve.c")
        .cargo_metadata(true)
        .flag("-lblas")
        .compile("cholmod");

    let suitesparse_config = true;

    if suitesparse_config {
        cc::Build::new()
            .include("SuiteSparse/SuiteSparse_config")
            .file("SuiteSparse/SuiteSparse_config/SuiteSparse_config.c")
            .cargo_metadata(false)
            .compile("suitesparseconfig");
    }
    println!("cargo:root={}", root.to_string_lossy());
}
