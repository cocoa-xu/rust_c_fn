use rustler::{Env, Term, Error, ResourceArc};
use std::sync::Mutex;

#[repr(C)]
struct ArrowArrayStream {
    get_schema: usize,
    get_next: usize,
    get_last_error: usize,
    release: usize,
    private_data: usize
}

struct ArrowArrayStreamResource {
    pub stream: Mutex<ArrowArrayStream>,
}

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(ArrowArrayStreamResource, env);
    true
}

#[rustler::nif]
fn rust_adbc_statement_execute_query(func_ptr: u64, stmt_ptr: u64, error_ptr: u64, _ref: Term) -> Result<(ResourceArc<ArrowArrayStreamResource>, i64), Error> {
    let ptr = func_ptr as *const ();
    let code: extern "C" fn(u64, &mut ArrowArrayStream, &mut i64, u64) -> u8 = unsafe { std::mem::transmute(ptr) };
    let mut rows_affected: i64 = 0;
    let mut stream = ArrowArrayStream {
        get_schema: 0, 
        get_next: 0, 
        get_last_error: 0, 
        release: 0, 
        private_data: 0
    };
    let ok = (code)(stmt_ptr, &mut stream, &mut rows_affected, error_ptr);

    if ok != 0 {
        Err(Error::Term(Box::new(error_ptr)))
    } else {
        let resource = ResourceArc::new(ArrowArrayStreamResource {
            stream: Mutex::new(stream),
        });

        Ok((resource, rows_affected))
    }
}

rustler::init!("Elixir.RustNIF", [
    rust_adbc_statement_execute_query
], load = load);
