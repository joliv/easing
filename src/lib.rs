// Copyright 2016 Jack Ward under LGPLv3
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTIBILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// Lesser General Public License `LICENSE` for details.

use std::f64::consts::{FRAC_PI_2};

macro_rules! easer {
    ($f:ident, $t:ident, $e:expr) => (
        pub struct $t {
            start: f64,
            dist: f64,
            step: u64,
            steps: u64,
        }

        pub fn $f(start: f64, end: f64, steps: u64) -> $t {
            $t {start: start, dist: end-start, step: 0, steps: steps}
        }

        impl Iterator for $t {
            type Item = f64;

            fn next(&mut self) -> Option<f64> {
                self.step += 1;
                if self.step > self.steps {
                    None
                } else {
                    let x = self.step as f64 / self.steps as f64;
                    Some($e(x) * self.dist + self.start)
                }
            }
        }
    )
}

easer!(linear, Linear, |x:f64| {
    x
});
easer!(quad_in, QuadIn, |x:f64| {
    x * x
});
easer!(quad_out, QuadOut, |x:f64| {
    -(x * (x - 2.))
});
easer!(quad_inout, QuadInOut, |x:f64| {
    if x < 0.5 { 2. * x * x }
    else { (-2. * x * x) + (4. * x) - 1. }
});
easer!(cubic_in, CubicIn, |x:f64| {
    x * x * x
});
easer!(cubic_out, CubicOut, |x:f64| {
    let y = x - 1.;
    y * y * y + 1.
});
easer!(cubic_inout, CubicInOut, |x:f64| {
    if x < 0.5 { 4. * x * x * x }
    else {
        let y = (2. * x) - 2.;
        0.5 * y * y * y + 1.
    }
});
easer!(quartic_in, QuarticIn, |x:f64| {
    x * x * x * x
});
easer!(quartic_out, QuarticOut, |x:f64| {
    let y = x - 1.;
    y * y * y * (1. - x) + 1.
});
easer!(quartic_inout, QuarticInOut, |x:f64| {
    if x < 0.5 { 8. * x * x * x * x }
    else {
        let y = x - 1.;
        -8. * y * y * y * y + 1.
    }
});
easer!(sin_in, SinIn, |x:f64| {
    let y = (x - 1.) * FRAC_PI_2;
    y.sin() + 1.
});
easer!(sin_out, SinOut, |x:f64| {
    (x * FRAC_PI_2).sin()
});

#[cfg(test)]
mod test {
    use super::*;

    // Rounds a float to five places after the decimal.
    // We can't use the exact value because of floating point
    // problems (13 != 12.999999999999999), and five decimal
    // points of precision is fine for this.
    fn round_5(x: f64) -> f64 {
        (x * 10E+5).round() / 10E+5
    }

    #[test]
    fn linear_test() {
        let model = vec![
            0.1,
            0.2,
            0.3,
            0.4,
            0.5,
            0.6,
            0.7,
            0.8,
            0.9,
            1.0,
        ];
        let try: Vec<f64> = linear(0f64, 1f64, 10).collect();
        assert_eq!(try, model);
    }

    #[test]
    fn quad_in_test() {
        let model = vec![
            100.,
            400.,
            900.,
            1600.,
            2500.,
            3600.,
            4900.,
            6400.,
            8100.,
            10000.,
        ];
        let try: Vec<f64> = quad_in(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try, model);
    }

    #[test]
    fn quad_out_test() {
        let model = vec![
            1900.,
            3600.,
            5100.,
            6400.,
            7500.,
            8400.,
            9100.,
            9600.,
            9900.,
            10000.,
        ];
        let try: Vec<f64> = quad_out(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try, model);
    }

    #[test]
    fn quad_inout_test() {
        let model = vec![
            200.,
            800.,
            1800.,
            3200.,
            5000.,
            6800.,
            8200.,
            9200.,
            9800.,
            10000.,
        ];
        let try: Vec<f64> = quad_inout(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try, model);
    }

    #[test]
    fn cubic_in_test() {
        let model = vec![
            10.,
            80.,
            270.,
            640.,
            1250.,
            2160.,
            3430.,
            5120.,
            7290.,
            10000.,
        ];
        let try: Vec<f64> = cubic_in(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try, model);
    }

    #[test]
    fn cubic_out_test() {
        let model = vec![
            2710.,
            4880.,
            6570.,
            7840.,
            8750.,
            9360.,
            9730.,
            9920.,
            9990.,
            10000.,
        ];
        let try: Vec<f64> = cubic_out(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try, model);
    }

    #[test]
    fn quartic_in_test() {
        let model = vec![
            1.,
            16.,
            81.,
            256.,
            625.,
            1296.,
            2401.,
            4096.,
            6561.,
            10000.,
        ];
        let try: Vec<f64> = quartic_in(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try, model);
    }

    #[test]
    fn quartic_out_test() {
        let model = vec![
            3439.,
            5904.,
            7599.,
            8704.,
            9375.,
            9744.,
            9919.,
            9984.,
            9999.,
            10000.,
        ];
        let try: Vec<f64> = quartic_out(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try, model);
    }

    #[test]
    fn quartic_inout_test() {
        let model = vec![
            8.,
            128.,
            648.,
            2048.,
            5000.,
            7952.,
            9352.,
            9872.,
            9992.,
            10000.,
        ];
        let try: Vec<f64> = quartic_inout(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try, model);
    }

    #[test]
    fn sin_in_test() {
        let model = vec![
            123.116594,
            489.434837,
            1089.934758,
            1909.830056,
            2928.932188,
            4122.147477,
            5460.095003,
            6909.830056,
            8435.655350,
            10000.,
        ];
        let try: Vec<f64> = sin_in(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try, model);
    }

    #[test]
    fn sin_out_test() {
        let model = vec![
            1564.344650,
            3090.169944,
            4539.904997,
            5877.852523,
            7071.067812,
            8090.169944,
            8910.065242,
            9510.565163,
            9876.883406,
            10000.
        ];
        let try: Vec<f64> = sin_out(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try, model);
    }
}
