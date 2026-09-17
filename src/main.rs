use crate::accel::AccelParams;
use crate::interception::Interceptor;

mod accel;
mod interception;

fn print_usage() {
    eprintln!(
        "usage: mouse-accelerator [--sens N] [--accel N] [--power N] [--cap N]\n\
         --sens   base sensitivity (default 1.0)\n\
         --accel  accel strength   (default 0.03)\n\
         --power  curve exponent   (default 2.0)\n\
         --cap    max multiplier   (default 3.0)"
    );
}

fn parse_args() -> Option<AccelParams> {
    let mut p = AccelParams::default();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let mut next_f64 = || args.next()?.parse::<f64>().ok();
        match arg.as_str() {
            "--sens" => p.sens = next_f64()?,
            "--accel" => p.accel = next_f64()?,
            "--power" => p.power = next_f64()?,
            "--cap" => p.cap = next_f64()?,
            "-h" | "--help" => return None,
            other => {
                eprintln!("unknown argument: {other}");
                return None;
            }
        }
    }
    Some(p)
}

fn main() {
    let Some(params) = parse_args() else {
        print_usage();
        return;
    };
    let interceptor = match Interceptor::new() {
        Ok(i) => i,
        Err(_) => {
            eprintln!("interceptor driver not found");
            return;
        }
    };
    interceptor.run(&params);
}
