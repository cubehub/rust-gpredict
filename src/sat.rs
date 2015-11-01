/*
 * The MIT License (MIT)
 *
 * Copyright (c) 2015 Andres Vahter (andres.vahter@gmail.com)
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use time;
use coordinates::LLA;


#[derive(Default, Debug)]
pub struct Sat {
    /// next AOS
    pub aos:                Option<time::Tm>,

    /// next LOS
    pub los:                Option<time::Tm>,

    /// azimuth [deg]
    pub az_deg:             f64,

    /// elevation [deg]
    pub el_deg:             f64,

    /// range [km]
    pub range_km:           f64,

    /// range rate [km/sec]
    pub range_rate_km_sec:  f64,

    /// SSP latitude [deg]
    pub lat_deg:            f64,

    /// SSP longitude [deg]
    pub lon_deg:            f64,

    /// altitude [km]
    pub alt_km:             f64,

    /// velocity [km/s]
    pub vel_km_s:           f64,

    /// orbit number
    pub orbit_nr:           u64,
}

impl Sat {
    pub fn location<T: From<LLA>>(&self) -> T {
        LLA {
            lat_deg: self.lat_deg,
            lon_deg: self.lon_deg,
            alt_m:   self.alt_km*1000.,
        }.into()
    }
}

#[test]
fn sat_location_formats() {
    use coordinates::ECEF;
    let sat = Sat::default();
    let _lla:  LLA  = sat.location();
    let _ecef: ECEF = sat.location();
}
