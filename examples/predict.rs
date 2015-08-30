#[macro_use]
extern crate log;
extern crate fern;
extern crate time;
extern crate gpredict;

use gpredict::tle;
use gpredict::predict::{Predict, Location};

use std::thread;

fn conf_logger() {
    let logger_config = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
            let t = time::now();
            let ms = t.tm_nsec/1000_000;
            format!("{}.{:3} [{}] {}", t.strftime("%Y-%m-%dT%H:%M:%S").unwrap(), ms, level, msg)
        }),
        output: vec![fern::OutputConfig::stderr()],
        level: log::LogLevelFilter::Trace,
    };

    if let Err(e) = fern::init_global_logger(logger_config, log::LogLevelFilter::Trace) {
        panic!("Failed to initialize global logger: {}", e);
    }
}

fn main() {
    // setup fern logger
    conf_logger();

    // start processing
    info!("predict example started");

    let tle: tle::Tle = tle::Tle{
        name: "ESTCUBE 1".to_string(),
        line1: "1 39161U 13021C   15091.47675532  .00001890  00000-0  31643-3 0  9990".to_string(),
        line2: "2 39161  98.0727 175.0786 0009451 192.0216 168.0788 14.70951130101965".to_string()
    };

    let location: Location = Location{lat_deg:58.64560, lon_deg: 23.15163, alt_m: 8};
    let mut predict: Predict = Predict::new(&tle, &location);

    loop {
        predict.update(Some(time::now_utc()));
        info!("aos        : {:?}", predict.sat.aos.expect("do not have AOS with this satellite"));
        info!("los        : {:?}", predict.sat.los.expect("do not have LOS with this satellite"));
        info!("az         : {:.2}°", predict.sat.az_deg);
        info!("el         : {:.2}°", predict.sat.el_deg);
        info!("range      : {:.0} km", predict.sat.range_km);
        info!("range rate : {:.3} km/sec\n", predict.sat.range_rate_km_sec);

        thread::sleep_ms(1000);
    }
}
