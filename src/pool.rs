use once_cell::sync::Lazy;
use std::thread;
use std::sync::{Arc, Mutex};

static POOL: Lazy<Arc<Mutex<Vec<Box<(dyn for<'a> Fn(&str) + Send + 'static)>>>>> = Lazy::new(|| {
    let pl = Arc::new(Mutex::new(Vec::new()));
    pl
});

static NTHREADS: Lazy<Arc<Mutex<i32>>> = Lazy::new(|| {
    let nt = Arc::new(Mutex::new(0));
    nt
});

fn exec<F>(f: F, param: &str)
    where F: Fn(&str)
{
    f(param);
}

/// Add function to the POOL
/// Pool will processed by threads
/// After process a function, the thread will remove that function from the POOL
pub fn add<F>(f: F, param: &str)
    where F: Fn(&str)
{

    let threads = &NTHREADS.lock().unwrap();
    let pool = POOL.as_ref().lock().unwrap();
    pool.push(Box::new(f));
    if threads.to_owned() < &4 {
        threads.to_owned().clone_from(&(threads.to_owned() + &1));
        thread::spawn(move || {
            while pool.len() > 0 {
                // TODO ejecutar funcion y eliminar
                let f = pool.last();
                exec(pool.last(), param);
                pool.remove(&(pool.len() - &1));
            }
            threads.to_owned().clone_from(&(threads.to_owned() - &1));
        });
    }

}


// fn main() {
//     let functions = vec![first, second, third];
//
//     for func in &functions {
//         dbg!(func(5));
//     }
//
//     dbg!(type_name_of(functions));
// }
//
// fn first(a: i32) -> i32 {
//     a + 2
// }
//
// fn second(a: i32) -> i32 {
//     a * a
// }
//
// fn third(a: i32) -> i32 {
//     -a
// }
//
// fn type_name_of<T>(_: T) -> &'static str {
//     std::any::type_name::<T>()
// }
