use crate::interception::Interceptor;

mod interception;

fn main() {
    println!(" ----- just need this for cs2 idk will someone use this crap -----");
    let interceptor = match Interceptor::new() {
        Ok(interceptor) => interceptor,
        Err(_) => {
            eprintln!("interceptor driver not found");
            return;
        }
    };

    interceptor.run();
}
