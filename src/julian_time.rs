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

// Calculates Julian Day Number
pub fn julian_timestamp(t: time::Tm) -> f64 {
    let year = t.tm_year + 1900;
    let month = t.tm_mon + 1;

    julian_date_of_year(year) +
        day_of_the_year(year, month, t.tm_mday) as f64 +
        fraction_of_day(t.tm_hour, t.tm_min, t.tm_sec) +
        t.tm_nsec as f64 / 1000_f64 / 8.64e+10
}

pub fn julian_to_unix(julian: f64) -> time::Tm {
    let unix = (julian - 2440587.5) * 86400.;
    let t = time::Timespec::new(unix.trunc() as i64, unix.fract() as i32);
    time::at(t)
}

fn fraction_of_day(h: i32, m: i32, s: i32) -> f64{
    (h as f64 + (m as f64 + s as f64 / 60.0) / 60.0) / 24.0
}

/// Astronomical Formulae for Calculators, Jean Meeus, pages 23-25.
/// Calculate Julian Date of 0.0 Jan year
fn julian_date_of_year(yr: i32) -> f64 {
    let year: u64 = yr as u64 -1;
    let mut i: f64 = (year as f64 / 100.).trunc();
    let a: f64 = i;
    i = (a / 4.).trunc();
    let b: f64 = (2. - a + i).trunc();
    i = (365.25 * year as f64).trunc();
    i += (30.6001_f64 * 14.0_f64).trunc();

    i + 1720994.5 + b
}

/// Calculates the day of the year for the specified date.
fn day_of_the_year(yr: i32, mo: i32, dy: i32) -> i32 {
    let days: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut day: i32 = 0;

    for d in &days[0 .. mo as usize - 1] {
        day += *d as i32;
    }

    day += dy;
    if (yr % 4 == 0) && ((yr % 100 != 0) || (yr % 400 == 0)) && (mo > 2) {
        day += 1;
    }

    day
}

#[test]
fn test_julian_timestamp() {
    // http://en.wikipedia.org/wiki/Julian_day#Converting_Julian_or_Gregorian_calendar_date_to_Julian_Day_Number
    let t = time::strptime("2000-1-1 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    assert_eq!(julian_timestamp(t), 2451545.0);

    let t = time::strptime("1970-1-1 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    assert_eq!(julian_timestamp(t), 2440587.5);
}
