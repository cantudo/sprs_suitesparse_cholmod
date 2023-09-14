use std::ffi::{self, CStr, c_void};
use std::convert::Into;

use sprs::{CsMatViewI, SpIndex};
use num_traits::Num;

pub use suitesparse_cholmod_sys::*;

#[allow(non_snake_case)]
pub fn llt_solve<N, I>(A: CsMatViewI<N, I>, b: Vec<f64>) -> &[f64]
where
    I: SpIndex,
    N: Copy + Num + PartialOrd + Into<f64>,
{
    unsafe {
        let mut common = std::mem::zeroed::<CholmodCommon>();

        let mut A_cholmod = build_cholmod_sparse(&A);
        let mut b_cholmod = build_cholmod_dense(&b);

        cholmod_l_start(&mut common);

        common.error_handler = Some(err_handler);
        common.final_ll = 1; // final LL' form (instead of LDL')

        /*
        let name = "A";
        let name_c = std::ffi::CString::new(name).unwrap();
        let name_c_ptr = name_c.as_ptr();
        cholmod_l_print_sparse(&G_cholmod, name_c_ptr, &mut common);
        */


        let L = cholmod_l_analyze(&A_cholmod, &mut common);
        cholmod_l_factorize(&mut A_cholmod, L, &mut common);

        /*
        let name = "L";
        let name_c = std::ffi::CString::new(name).unwrap();
        let name_c_ptr = name_c.as_ptr();
        cholmod_l_print_factor(L, name_c_ptr, &mut common);
        */

        let x = cholmod_l_solve(CHOLMOD_A as i32, L, &b_cholmod, &mut common);

        let x_slice: &[f64] = std::slice::from_raw_parts((*x).x as *mut ffi::c_double, (*x).nrow) ;

        cholmod_l_finish(&mut common);

        x_slice
    }
}

// #[allow(non_snake_case)]
// pub fn llt_factorize<N, I>(A: CsMatViewI<N, I>, b: Vec<f64>) -> &CholmodFactor
//     where
//         I: SpIndex,
//         N: Copy + Num + PartialOrd + Into<f64>,
// {
//     unsafe {
//         let L = cholmod_l_analyze(&A_cholmod, &mut common);
//
//         &(*L)
//     }
// }

/// Builds a CholmodSparse from a sprs CsMatViewI
/// It assumes the given matrix is symmetric.
#[allow(non_snake_case)]
pub fn build_cholmod_sparse<N, I>(A: &CsMatViewI<N, I>) -> CholmodSparse
    where
        I: SpIndex,
        N: Copy + Num + PartialOrd + Into<f64>,
{
    unsafe {
        let mut A_cholmod = std::mem::zeroed::<CholmodSparse>();

        A_cholmod.nrow = A.rows();
        A_cholmod.ncol = A.cols();
        A_cholmod.nzmax = A.nnz();

        let indptr = A.indptr();
        let indptr_proper = indptr.to_proper();

        A_cholmod.p = indptr_proper.as_ptr() as *mut c_void;
        A_cholmod.i = A.indices().as_ptr() as *mut c_void;
        A_cholmod.x = A.data().as_ptr() as *mut ffi::c_void;

        // Matrix is symmetric
        A_cholmod.stype = 1;

        A_cholmod.itype = CHOLMOD_LONG as ffi::c_int;
        A_cholmod.xtype = CHOLMOD_REAL;
        A_cholmod.dtype = 0; // double
        A_cholmod.sorted = 0; // matrix is not sorted
        A_cholmod.packed = 1; // matrix is in packed form (A_cholmod->nz is ignored)

        A_cholmod
    }
}

#[allow(non_snake_case)]
pub fn build_cholmod_dense(b: &Vec<f64>) -> CholmodDense {
    unsafe {
        let mut b_cholmod = std::mem::zeroed::<CholmodDense>();

        b_cholmod.nrow = b.len();
        b_cholmod.ncol = 1;
        b_cholmod.nzmax = b.len();
        b_cholmod.d = b.len();
        b_cholmod.x = b.as_ptr() as *mut c_void;
        b_cholmod.xtype = CHOLMOD_REAL;
        b_cholmod.dtype = 0; // double

        b_cholmod
    }
}
/* halt if an error occurs */
#[no_mangle]
pub extern fn err_handler(
    status: ffi::c_int,
    file: *const ffi::c_char,
    line: ffi::c_int,
    message: *const ffi::c_char,
) {
    let c_file_str = unsafe { CStr::from_ptr(file) };
    let file = c_file_str.to_str().unwrap();

    let c_message_str = unsafe { CStr::from_ptr(message) };
    let message = c_message_str.to_str().unwrap();

    println!(
        "cholmod error: file {}, line {}, status {}, message {}",
        file, line, status, message
    );

    if status < 0 {
        std::process::exit(1);
    }
}

/* Dirty example of how to use the cholmod library */
/*
#[allow(non_snake_case)]
pub fn test2<N, I>(G: CsMatViewI<N, I>, b: Vec<f64>)
where
    I: SpIndex,
    N: Copy + Num + PartialOrd + Into<f64>,
{
    unsafe {
        let mut common = std::mem::zeroed::<CholmodCommon>();
        let mut G_cholmod = std::mem::zeroed::<CholmodSparse>();
        let mut b_cholmod = std::mem::zeroed::<CholmodDense>();

        G_cholmod.nrow = G.rows();
        G_cholmod.ncol = G.cols();
        G_cholmod.nzmax = G.nnz();

        let indptr = G.indptr();
        let indptr_proper = indptr.to_proper();

        G_cholmod.p = indptr_proper.as_ptr() as *mut c_void;
        G_cholmod.i = G.indices().as_ptr() as *mut c_void;
        G_cholmod.x = G.data().as_ptr() as *mut ffi::c_void;

        // G_cholmod.nz = 
        G_cholmod.stype = 0;

        G_cholmod.itype = CHOLMOD_LONG as ffi::c_int;
        G_cholmod.xtype = CHOLMOD_REAL;
        G_cholmod.dtype = 0; // float
        G_cholmod.sorted = 0;
        G_cholmod.packed = 1;

        
        b_cholmod.nrow = b.len();
        b_cholmod.ncol = 1;
        b_cholmod.nzmax = b.len();
        b_cholmod.d = b.len();
        b_cholmod.x = b.as_ptr() as *mut c_void;
        b_cholmod.xtype = CHOLMOD_REAL;
        b_cholmod.dtype = 0; // float

        //print b
        for i in 0..b.len() {
            println!("b: {}", b[i]);
        }

        // common.nmethods = 1;
        // common.method[0].ordering = 2; // AMD

        
        cholmod_l_start(&mut common);
        
        // open file using libc
        let fs: *mut libc::FILE = libc::fopen(
            std::ffi::CString::new("test.mtx").unwrap().as_ptr(),
            std::ffi::CString::new("r").unwrap().as_ptr(),
        );
        
        
        let g = cholmod_l_read_sparse(fs, &mut common);
        
        common.final_ll = 1;

        cholmod_l_print_sparse(g, CString::new("G").unwrap().as_ptr(), &mut common);
        let L = cholmod_l_analyze(g, &mut common);
        cholmod_l_factorize(g, L, &mut common);

        cholmod_l_print_factor(L, CString::new("L").unwrap().as_ptr(), &mut common);

        let x = cholmod_l_solve (1, L, &b_cholmod, &mut common) ;

        cholmod_l_finish(&mut common);

    }
}
*/
