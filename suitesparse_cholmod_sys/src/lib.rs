#![allow(non_camel_case_types, non_snake_case)]

use std::ffi;

pub type cholmod_int = std::ffi::c_int;
pub type size_t = usize;

#[cfg(target_os = "windows")]
pub type cholmod_long = i64;
#[cfg(not(target_os = "windows"))]
pub type cholmod_long = std::ffi::c_long;
pub type cholmod_double = std::ffi::c_double;

const CHOLMOD_MAXMETHODS: usize = 9;
const CHOLMOD_HOST_SUPERNODE_BUFFERS: usize = 8;

pub const CHOLMOD_INT: usize = 0;
pub const CHOLMOD_INTLONG: usize = 1;
pub const CHOLMOD_LONG: usize = 2;

pub const CHOLMOD_REAL: ffi::c_int = 1;
pub const CHOLMOD_COMPLEX: usize = 2;
pub const CHOLMOD_ZOMPLEX: usize = 3;

pub const CHOLMOD_DOUBLE: usize = 0;

pub const CHOLMOD_A: usize = 0;


#[repr(C)]
pub struct CholmodMethod {
    lnz: ffi::c_double,
    fl: ffi::c_double,
    prune_dense: ffi::c_double,
    prune_dense2: ffi::c_double,
    nd_oksep: ffi::c_double,
    other_1: [ffi::c_double; 4],
    nd_small: size_t,
    other_2: [size_t; 4],
    aggressive: ffi::c_int,
    order_for_lu: ffi::c_int,
    nd_compress: ffi::c_int,
    nd_camd: ffi::c_int,
    nd_components: ffi::c_int,
    ordering: ffi::c_int,
    other_3: [size_t; 4],

}

#[repr(C)]
pub struct CholmodCommon {
    pub dbound: ffi::c_double,
    pub grow0: ffi::c_double,
    pub grow1: ffi::c_double,
    pub grow2: size_t,
    pub maxrank: size_t,
    pub supernodal_switch: ffi::c_double,
    pub supernodal: ffi::c_int,
    pub final_asis: ffi::c_int,
    pub final_super: ffi::c_int,
    pub final_ll: ffi::c_int,
    pub final_pack: ffi::c_int,
    pub final_monotonic: ffi::c_int,
    pub final_resymbol: ffi::c_int,
    pub zrelax: [ffi::c_double; 3],
    pub nrelax: [size_t; 3],
    pub prefer_zomplex: ffi::c_int,
    pub prefer_upper: ffi::c_int,
    pub quick_return_if_not_posdef: ffi::c_int,
    pub prefer_binary: ffi::c_int,
    pub print: ffi::c_int,
    pub precise: ffi::c_int,
    pub try_catch: ffi::c_int,

    pub error_handler: Option<
        extern fn(
            status: ffi::c_int,
            file: *const ffi::c_char,
            line: ffi::c_int,
            message: *const ffi::c_char,
        ),
    >,
    pub nmethods: ffi::c_int,
    pub current: ffi::c_int,
    pub selected: ffi::c_int,

    pub method: [CholmodMethod; CHOLMOD_MAXMETHODS + 1 ],
    pub postorder: ffi::c_int,
    pub default_nesdis: ffi::c_int,
    pub metis_memory: ffi::c_double,
    pub metis_dswitch: ffi::c_double,
    pub metis_nswitch: size_t,

    pub nrow: size_t,
    pub mark: i64,
    pub iworksize: size_t,
    pub xworksize: size_t,

    pub Flag: *mut ffi::c_void,
    pub Head: *mut ffi::c_void,
    pub Xwork: *mut ffi::c_void,
    pub Iwork: *mut ffi::c_void,

    pub itype: ffi::c_int,
    pub dtype: ffi::c_int,

    pub no_workspace_reallocate: ffi::c_int,

    pub status: ffi::c_int,

    pub fl: ffi::c_double,
    pub lnz: ffi::c_double,
    pub anz: ffi::c_double,
    
    pub modfl: ffi::c_double,
    
    pub malloc_count: size_t,
    pub memory_usage: size_t,
    pub memory_inuse: size_t,

    pub nrealloc_col: ffi::c_double,
    pub nrealloc_factor: ffi::c_double,
    pub ndbounds_hit: ffi::c_double,

    pub rowfacfl: ffi::c_double,
    pub aatfl: ffi::c_double,

    pub called_nd: ffi::c_int,
    pub blas_ok: ffi::c_int,

    pub SPQR_grain: ffi::c_double,
    pub SPQR_small: ffi::c_double,
    pub SPQR_shrink: ffi::c_int,
    pub SPQR_nthreads: ffi::c_int,

    pub SPQR_flopcount: ffi::c_double,
    pub SPQR_analyze_time: ffi::c_double,
    pub SPQR_factorize_time: ffi::c_double,
    pub SPQR_solve_time: ffi::c_double,

    pub SPQR_flopcount_bound: ffi::c_double,
    pub SPQR_tol_used: ffi::c_double,
    pub SPQR_norm_E_fro: ffi::c_double,

    pub SPQR_istat: [i64; 10],

    pub useGPU: ffi::c_int,

    pub maxGpuMemBytes: size_t,
    pub maxGpuMemFraction: ffi::c_double,

    pub gpuMemorySize: size_t,
    pub gpuKernelTime: ffi::c_double,
    pub gpuFlops: i64,
    pub gpuNumKernelLaunches: ffi::c_int,

    // No CUDA for now
    pub cublasHandle: *mut ffi::c_void,
    pub gpuStream: [*mut ffi::c_void; CHOLMOD_HOST_SUPERNODE_BUFFERS],
    pub cublasEventPotrf: [*mut ffi::c_void; 3],
    pub updateCKernelsComplete: *mut ffi::c_void,
    pub updateCBuffersFree: [*mut ffi::c_void; CHOLMOD_HOST_SUPERNODE_BUFFERS],

    pub dev_mempool: *mut ffi::c_void,
    pub dev_mempool_size: size_t,

    pub host_pinned_mempool: *mut ffi::c_void,
    pub host_pinned_mempool_size: size_t,

    pub devBuffSize: size_t,
    pub ibuffer: ffi::c_int,

    pub syrkStart: ffi::c_double,

    /* run times of the different parts of CHOLMOD (GPU and CPU) */
    pub cholmod_cpu_gemm_time: ffi::c_double,
    pub cholmod_cpu_syrk_time: ffi::c_double,
    pub cholmod_cpu_trsm_time: ffi::c_double,
    pub cholmod_cpu_potrf_time: ffi::c_double,
    pub cholmod_gpu_gemm_time: ffi::c_double,
    pub cholmod_gpu_syrk_time: ffi::c_double,
    pub cholmod_gpu_trsm_time: ffi::c_double,
    pub cholmod_gpu_potrf_time: ffi::c_double,
    pub cholmod_assemble_time: ffi::c_double,
    pub cholmod_assemble_time2: ffi::c_double,

    /* number of times the BLAS are called on the CPU and the GPU */
    pub cholmod_cpu_gemm_calls: size_t,
    pub cholmod_cpu_syrk_calls: size_t,
    pub cholmod_cpu_trsm_calls: size_t,
    pub cholmod_cpu_potrf_calls: size_t,
    pub cholmod_gpu_gemm_calls: size_t,
    pub cholmod_gpu_syrk_calls: size_t,
    pub cholmod_gpu_trsm_calls: size_t,
    pub cholmod_gpu_potrf_calls: size_t,

    pub chunk: ffi::c_double,

    pub nthreads_max: ffi::c_int,
}

#[repr(C)]
pub struct CholmodSparse {
    pub nrow: size_t,
    pub ncol: size_t,
    pub nzmax: size_t,

    pub p: *mut ffi::c_void,
    pub i: *mut ffi::c_void,

    pub nz: *mut ffi::c_void,

    pub x: *mut ffi::c_void,
    pub z: *mut ffi::c_void,

    pub stype: ffi::c_int,

    pub itype: ffi::c_int,
    pub xtype: ffi::c_int,
    pub dtype: ffi::c_int,
    pub sorted: ffi::c_int,
    pub packed: ffi::c_int,
}

#[repr(C)]
pub struct CholmodDense {
    pub nrow: size_t,
    pub ncol: size_t,
    pub nzmax: size_t,
    pub d: size_t,
    pub x: *mut ffi::c_void,
    pub z: *mut ffi::c_void,
    pub xtype: ffi::c_int,
    pub dtype: ffi::c_int,
}

#[repr(C)]
pub struct CholmodFactor {
    pub n: size_t,
    pub minor: size_t,
    
    pub Perm: *mut ffi::c_void,
    pub ColCount: *mut ffi::c_void,
    pub IPerm: *mut ffi::c_void,
    
    pub nzmax: size_t,
    pub p: *mut ffi::c_void,
    pub i: *mut ffi::c_void,
    pub x: *mut ffi::c_void,
    pub z: *mut ffi::c_void,
    pub nz: *mut ffi::c_void,
    pub next: *mut ffi::c_void,
    pub prev: *mut ffi::c_void,

    pub nsuper: size_t,
    pub ssize: size_t,
    pub xsize: size_t,
    pub maxcsize: size_t,
    pub maxesize: size_t,

    pub super_: *mut ffi::c_void,
    pub pi: *mut ffi::c_void,
    pub px: *mut ffi::c_void,
    pub s: *mut ffi::c_void,

    pub ordering: ffi::c_int,
    pub is_ll: ffi::c_int,
    pub is_super: ffi::c_int,
    pub is_monotonic: ffi::c_int,

    pub itype: ffi::c_int,
    pub xtype: ffi::c_int,
    pub dtype: ffi::c_int,
    pub useGPU: ffi::c_int
}

extern "C" {

    pub fn cholmod_l_start(common: *mut CholmodCommon);
    pub fn cholmod_l_finish(common: *mut CholmodCommon);
    pub fn cholmod_l_print_sparse(A: *const CholmodSparse, name: *const ffi::c_char, common: *mut CholmodCommon);
    pub fn cholmod_l_analyze(A: *const CholmodSparse, common: *mut CholmodCommon) -> *mut CholmodFactor;
    // pub fn cholmod_solve(
    //     sys: ffi::c_int,
    //     L: *const CholmodFactor,
    //     B: *const CholmodDense,
    //     common: *mut CholmodCommon,
    // ) -> *mut CholmodDense;

    pub fn cholmod_l_print_factor(L: *const CholmodFactor, name: *const ffi::c_char, common: *mut CholmodCommon);

    pub fn cholmod_l_solve(
        sys: ffi::c_int,
        L: *const CholmodFactor,
        B: *const CholmodDense,
        common: *mut CholmodCommon,
    ) -> *mut CholmodDense;

    pub fn cholmod_l_factorize(A: *mut CholmodSparse, L: *mut CholmodFactor, common: *mut CholmodCommon);

    pub fn cholmod_l_read_sparse(file: *mut libc::FILE, common: *mut CholmodCommon) -> *mut CholmodSparse;

    pub fn cholmod_l_free_factor(cholmod_factor: *mut *mut CholmodFactor, common: *mut CholmodCommon) -> ffi::c_int;

    pub fn cholmod_l_free_sparse(cholmod_sparse: *mut *mut CholmodSparse, common: *mut CholmodCommon) -> ffi::c_int;

    pub fn cholmod_l_free_dense(cholmod_dense: *mut *mut CholmodDense, common: *mut CholmodCommon) -> ffi::c_int;
}

#[test]
fn sanity() {
    // let n = 1;
    // let ap = &[0, 1];
    // let ai = &[0];
    // let valid;
    // unsafe {
    //     valid = ldl_valid_matrix(n, ap.as_ptr(), ai.as_ptr());
    // }
    // assert_eq!(valid, 1);
}
