use crate::polynomial;

pub struct ExplicitRungeKuttaTable<const S: usize, const S2: usize, T> {
    pub order: usize,
    pub order_embedded: usize,
    pub order_interpolant: usize,
    pub a: [T; S2],
    pub b: [T; S],
    pub b2: [T; S],
    pub c: [T; S],
    pub bi: [crate::polynomial::Differentiable<fn(T) -> T, fn(T) -> T>; S],
}
impl<const S: usize, const S2: usize, T> ExplicitRungeKuttaTable<S, S2, T> {
    /// [Self::a] values using two dimensional indexing
    pub fn a_indexed(&self, i: usize, j: usize) -> T
    where
        T: Copy,
    {
        self.a[i * (i - 1) / 2 + j]
    }
}

/// Euler method (<https://en.wikipedia.org/wiki/Euler_method>), with linear interpolation
pub fn euler<T>() -> ExplicitRungeKuttaTable<1, 0, T>
where
    T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + From<f64> + Copy,
{
    ExplicitRungeKuttaTable {
        order: 1,
        order_embedded: 0,
        order_interpolant: 1,
        a: [],
        b: [T::from(1.)],
        b2: [T::from(0.)],
        c: [T::from(0.)],
        bi: [polynomial![T, T::from(0.), T::from(1.)]],
    }
}

/// Macro declares a generic RungeKuttaTable<2> of order 2 with linear interpolation, and Euler method as an
/// embedded scheme.
/// <https://en.wikipedia.org/wiki/List_of_Runge%E2%80%93Kutta_methods#cite_ref-butcher_1-0>
#[macro_export]
macro_rules! generic_rk_order2 {
    ($name:ident, $alpha:expr) => {
        pub fn $name<T>() -> ExplicitRungeKuttaTable<2, 1, T>
        where
            T: std::ops::Add<Output = T>
                + std::ops::Sub<Output = T>
                + std::ops::Mul<Output = T>
                + std::ops::Div<Output = T>
                + From<f64>
                + From<i32>
                + From<T>
                + Copy,
        {
            let alpha = T::from($alpha);
            let half = T::from(0.5);

            ExplicitRungeKuttaTable {
                order: 2,
                order_embedded: 1,
                order_interpolant: 1,
                a: [alpha],
                b: [T::from(1) - half / alpha, half / alpha],
                b2: [T::from(1), T::from(0)],
                c: [T::from(0), alpha],
                bi: [
                    polynomial![T, T::from(0), T::from(1) - T::from(0.5) / T::from($alpha)],
                    polynomial![T, T::from(0), T::from(0.5) / T::from($alpha)],
                ],
            }
        }
    };
}

generic_rk_order2!(midpoint, 0.5);
generic_rk_order2!(heun2, 1);
generic_rk_order2!(ralston2, T::from(2) / T::from(3));

/// Macro declares a generic RungeKuttaTable<3> of order 3 with linear interpolation, and embedded order 2 method
/// <https://en.wikipedia.org/wiki/List_of_Runge–Kutta_methods>
#[macro_export]
macro_rules! generic_rk_order3 {
    ($name:ident, $alpha:expr, $beta:expr) => {
        pub fn $name<T>() -> ExplicitRungeKuttaTable<3, 3, T>
        where
            T: std::ops::Add<Output = T>
                + std::ops::Mul<Output = T>
                + std::ops::Div<Output = T>
                + std::ops::Sub<Output = T>
                + From<f64>
                + From<i32>
                + From<T>
                + Copy,
        {
            let alpha = T::from($alpha);
            let beta = T::from($beta);
            let two = T::from(2);
            let three = T::from(3);
            let six = T::from(6);

            let denom = three * alpha - two;
            let a1 = alpha;
            let a2 = (beta / alpha) * (beta - three * alpha * (T::from(1) - alpha)) / denom;
            let a3 = (beta / alpha) * (alpha - beta) / denom;

            let b1 = T::from(1) - (three * alpha + three * beta - two) / (six * alpha * beta);
            let b2 = (three * beta - two) / (six * alpha * (beta - alpha));
            let b3 = (two - three * alpha) / (six * beta * (beta - alpha));

            let half = T::from(0.5);

            ExplicitRungeKuttaTable {
                order: 3,
                order_embedded: 2,
                order_interpolant: 1,
                a: [a1, a2, a3],
                b: [b1, b2, b3],
                b2: [T::from(1) - half / alpha, half / alpha, T::from(0)],
                c: [T::from(0), alpha, beta],
                bi: [
                    polynomial![
                        T,
                        T::from(0),
                        T::from(1)
                            - (T::from(3) * T::from($alpha) + T::from(3) * T::from($beta)
                                - T::from(2))
                                / (T::from(6) * T::from($alpha) * T::from($beta))
                    ],
                    polynomial![
                        T,
                        T::from(0),
                        (T::from(3) * T::from($beta) - T::from(2))
                            / (T::from(6) * T::from($alpha) * (T::from($beta) - T::from($alpha)))
                    ],
                    polynomial![
                        T,
                        T::from(0),
                        (T::from(2) - T::from(3) * T::from($alpha))
                            / (T::from(6) * T::from($beta) * (T::from($beta) - T::from($alpha)))
                    ],
                ],
            }
        }
    };
}

generic_rk_order3!(kutta3, 0.5, 1);
generic_rk_order3!(heun3, T::from(1) / T::from(3), T::from(2) / T::from(3));
generic_rk_order3!(ralston3, 0.5, 0.75);
generic_rk_order3!(wray3, T::from(8) / T::from(15), T::from(2) / T::from(3));
generic_rk_order3!(ssp3, 1.0, 0.5);

/// "The" Runge-Kutta method, with embedded order 2 method, and with order 3 interpolant.
pub fn classic4<T>() -> ExplicitRungeKuttaTable<4, 6, T>
where
    T: std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Sub<Output = T>
        + From<f64>
        + From<i32>
        + Copy,
{
    let one_sixth = T::from(1) / T::from(6);
    let one_third = T::from(1) / T::from(3);
    let half = T::from(0.5);

    ExplicitRungeKuttaTable {
        order: 4,
        order_embedded: 2,
        order_interpolant: 3,
        a: [half, T::from(0), half, T::from(0), T::from(0), T::from(1)],
        b: [one_sixth, one_third, one_third, one_sixth],
        b2: [T::from(0), T::from(1), T::from(0), T::from(0)],
        c: [T::from(0), half, half, T::from(1)],
        bi: [
            polynomial![
                T,
                T::from(0),
                T::from(1),
                T::from(-1.5),
                T::from(2) / T::from(3)
            ],
            polynomial![
                T,
                T::from(0),
                T::from(0),
                T::from(1),
                T::from(-2) / T::from(3)
            ],
            polynomial![
                T,
                T::from(0),
                T::from(0),
                T::from(1),
                T::from(-2) / T::from(3)
            ],
            polynomial![
                T,
                T::from(0),
                T::from(0),
                T::from(-0.5),
                T::from(2) / T::from(3)
            ],
        ],
    }
}

/// "The" Runge-Kutta method, with embedded order 3 method, and with order 3 interpolant. Also
/// known as the Zonneveld 4(3) method.
pub fn classic43<T>() -> ExplicitRungeKuttaTable<5, 10, T>
where
    T: std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Neg<Output = T>
        + From<f64>
        + From<i32>
        + Copy,
{
    let one_sixth = T::from(1) / T::from(6);
    let one_third = T::from(1) / T::from(3);
    let half = T::from(0.5);

    ExplicitRungeKuttaTable {
        order: 4,
        order_embedded: 3,
        order_interpolant: 3,
        a: [
            half,
            T::from(0),
            half,
            T::from(0),
            T::from(0),
            T::from(1),
            T::from(5) / T::from(32),
            T::from(7) / T::from(32),
            T::from(13) / T::from(32),
            T::from(-1) / T::from(32),
        ],
        b: [one_sixth, one_third, one_third, one_sixth, T::from(0)],
        b2: [
            T::from(-0.5),
            T::from(7) / T::from(3),
            T::from(7) / T::from(3),
            T::from(13) / T::from(6),
            T::from(-16) / T::from(3),
        ],
        c: [T::from(0), half, half, T::from(1), T::from(0.75)],
        bi: [
            polynomial![
                T,
                T::from(0),
                T::from(1),
                T::from(-1.5),
                T::from(2) / T::from(3)
            ],
            polynomial![
                T,
                T::from(0),
                T::from(0),
                T::from(1),
                -T::from(2) / T::from(3)
            ],
            polynomial![
                T,
                T::from(0),
                T::from(0),
                T::from(1),
                -T::from(2) / T::from(3)
            ],
            polynomial![
                T,
                T::from(0),
                T::from(0),
                T::from(-0.5),
                T::from(2) / T::from(3)
            ],
            polynomial![T, T::from(0)],
        ],
    }
}

/// Dormand-Prince method of order 5, with embedded order 4 method, and with order 4 interpolant.
pub fn rk547fm<T>() -> ExplicitRungeKuttaTable<7, 21, T>
where
    T: std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Sub<Output = T>
        + From<f64>
        + From<i32>
        + Copy,
{
    ExplicitRungeKuttaTable {
        order: 5,
        order_embedded: 4,
        order_interpolant: 4,
        a: [
            T::from(1) / T::from(5),

            T::from(3) / T::from(40),
            T::from(9) / T::from(40),

            T::from(44) / T::from(45),
            T::from(-56) / T::from(15),
            T::from(32) / T::from(9),

            T::from(19372) / T::from(6561),
            T::from(-25360) / T::from(2187),
            T::from(64448) / T::from(6561),
            T::from(-212) / T::from(729),

            T::from(-9017) / T::from(3168),
            T::from(-355) / T::from(33),
            T::from(46732) / T::from(5247),
            T::from(49) / T::from(176),
            T::from(-5103) / T::from(18656),
            
            T::from(35) / T::from(384),
            T::from(0),
            T::from(500) / T::from(1113),
            T::from(125) / T::from(192),
            T::from(-2187) / T::from(6784),
            T::from(11) / T::from(84),
        ],
        b: [
            T::from(35) / T::from(384),
            T::from(0),
            T::from(500) / T::from(1113),
            T::from(125) / T::from(192),
            T::from(-2187) / T::from(6784),
            T::from(11) / T::from(84),
            T::from(0),
        ],
        b2: [
            T::from(5179) / T::from(57600),
            T::from(0),
            T::from(7571) / T::from(16695),
            T::from(393) / T::from(640),
            T::from(-92097) / T::from(339200),
            T::from(187) / T::from(2100),
            T::from(1) / T::from(40),
        ],
        c: [
            T::from(0),
            T::from(1) / T::from(5),
            T::from(3) / T::from(10),
            T::from(4) / T::from(5),
            T::from(8) / T::from(9),
            T::from(1),
            T::from(1),
        ],
        bi: [
            polynomial![
                T,
                T::from(0),
                T::from(1),
                T::from(-32272833064.) / T::from(11282082432.),
                T::from(34969693132.) / T::from(11282082432.),
                T::from(-13107642775.) / T::from(11282082432.),
                T::from(157015080) / T::from(11282082432.)
            ],
            polynomial![T, T::from(0)],
            polynomial![
                T,
                T::from(0),
                T::from(0),
                T::from(-132343189600.) / T::from(32700410799.),
                T::from(207495684000.) / T::from(32700410799.),
                T::from(-91412856700.) / T::from(32700410799.),
                T::from(1570150800.) / T::from(32700410799.)
            ],
            polynomial![
                T,
                T::from(0),
                T::from(0),
                T::from(-889289856. * 25.) / T::from(5641041216.),
                T::from(2460397220. * 25.) / T::from(5641041216.),
                T::from(-1518414297. * 25.) / T::from(5641041216.),
                T::from(94209048. * 25.) / T::from(5641041216.)
            ],
            polynomial![
                T,
                T::from(0),
                T::from(0),
                T::from(-259006536. * -2187.) / T::from(199316789632.),
                T::from(687873124. * -2187.) / T::from(199316789632.),
                T::from(-451824525. * -2187.) / T::from(199316789632.),
                T::from(52338360. * -2187.) / T::from(199316789632.),
            ],
            polynomial![
                T,
                T::from(0),
                T::from(0),
                T::from(-361440756. * 11.) / T::from(2767955532.),
                T::from(946554244. * 11.) / T::from(2767955532.),
                T::from(-661884105. * 11.) / T::from(2767955532.),
                T::from(106151040. * 11.) / T::from(2767955532.)
            ],
            polynomial![
                T,
                T::from(0),
                T::from(0),
                T::from(44764047) / T::from(29380423),
                T::from(-82437520 - 44764047) / T::from(29380423),
                T::from(8293050 + 82437520) / T::from(29380423),
                T::from(-8293050) / T::from(29380423)
            ],
        ],
    }
}

/// Runge-Kutta Tsitouras-Papakostas 6(4)
///
/// SIAM J. Sci. Comput., 20 (1999)  2067-2088
///
/// Coefficients: http://users.uoa.gr/~tsitourasc/rktp64.m
///
/// More from the author: http://users.uoa.gr/~tsitourasc/publications.html
pub fn rktp64<T>() -> ExplicitRungeKuttaTable<7, 21, T>
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + From<f64>
        + Copy,
{
    ExplicitRungeKuttaTable {
        order: 6,
        order_embedded: 4,
        order_interpolant: 4,
        a: [
            T::from(0.14814814814814814814814814814814814814814814814815f64),
            T::from(0.05555555555555555555555555555555555555555555555556f64),
            T::from(0.16666666666666666666666666666666666666666666666667f64),
            T::from(0.19241982507288629737609329446064139941690962099125f64),
            T::from(-0.53134110787172011661807580174927113702623906705539f64),
            T::from(0.76749271137026239067055393586005830903790087463557f64),
            T::from(0.27138264973958333333333333333333333333333333333333f64),
            T::from(-0.28179931640625000000000000000000000000000000000000f64),
            T::from(0.10191932091346153846153846153846153846153846153846f64),
            T::from(0.59599734575320512820512820512820512820512820512821f64),
            T::from(-0.12140681348692272679528027730121494345436084170722f64),
            T::from(0.47761410187445690404270285927090660818471469359043f64),
            T::from(0.12192296968479080920271208457739825271063725338342f64),
            T::from(0.00820786686248269381285345905535723094994297948894f64),
            T::from(0.28289264429596155050624264362832208237829668447521f64),
            T::from(0.32310946589106292966684294024325753569539925965098f64),
            T::from(-0.61039132734003172924378635642517186673717609730301f64),
            T::from(0.45846867541639319976612888047255080412929454343221f64),
            T::from(0.57505740806711566278133278660922926335638856572569f64),
            T::from(-0.57379234522267681781745625845989491691225880929347f64),
            T::from(0.82754812318813675484693800756002918046835253778760f64),
        ],
        b: [
            T::from(0.07277777777777777777777777777777777777777777777778f64),
            T::from(0.00000000000000000000000000000000000000000000000000f64),
            T::from(0.28752127070690503526324421846809906511399048712482f64),
            T::from(0.18974846220396832187710941882243328294496258901153f64),
            T::from(0.10581736348682550735167973519824811036097403449285f64),
            T::from(0.26909544328484081804764916719375922411975542905334f64),
            T::from(0.07503968253968253968253968253968253968253968253968f64),
        ],
        b2: [
            T::from(0.10322666047518118524035683798997408464864086165861f64),
            T::from(0.00000000000000000000000000000000000000000000000000f64),
            T::from(0.15611542056134071025476537742246069068941506933613f64),
            T::from(0.38634918851063907802720917783777796648011517818308f64),
            T::from(-0.12073095208684351320487107578989528150071079171750f64),
            T::from(0.40000000000000000000000000000000000000000000000000f64),
            T::from(0.07503968253968253968253968253968253968253968253968f64),
        ],
        c: [
            T::from(0.00000000000000000000000000000000000000000000000000f64),
            T::from(0.14814814814814814814814814814814814814814814814815f64),
            T::from(0.22222222222222222222222222222222222222222222222222f64),
            T::from(0.42857142857142857142857142857142857142857142857143f64),
            T::from(0.68750000000000000000000000000000000000000000000000f64),
            T::from(0.76923076923076923076923076923076923076923076923077f64),
            T::from(1.00000000000000000000000000000000000000000000000000f64),
        ],
        bi: [
            polynomial![
                T,
                T::from(0.00000000000000000000000000000000000000000000000000f64),
                T::from(1.00000000000000000000000000000000000000000000000000f64),
                T::from(-3.18888888888888888888888888888888888888888888888890f64),
                T::from(3.62962962962962962962962962962962962962962962962960f64),
                T::from(-1.36796296296296296296296296296296296296296296296300f64)
            ],
            polynomial![
                T,
                T::from(0.00000000000000000000000000000000000000000000000000f64)
            ],
            polynomial![
                T,
                T::from(0.00000000000000000000000000000000000000000000000000f64),
                T::from(0.00000000000000000000000000000000000000000000000000f64),
                T::from(3.13664097096932917828440216499917992455305888141710f64),
                T::from(-4.87567656224372642283090044284074134820403477119900f64),
                T::from(2.02655686198130227980974249630966048876496637690670f64)
            ],
            polynomial![
                T,
                T::from(0.00000000000000000000000000000000000000000000000000f64),
                T::from(0.00000000000000000000000000000000000000000000000000f64),
                T::from(1.44306660772178013557323902151488358384910109048040f64),
                T::from(-2.68732193732193732193732193732193732193732193732190f64),
                T::from(1.43400379180412550824119233462948702103318343585310f64)
            ],
            polynomial![
                T,
                T::from(0.00000000000000000000000000000000000000000000000000f64),
                T::from(0.00000000000000000000000000000000000000000000000000f64),
                T::from(-1.84105678504031566306399039286326985760850917824670f64),
                T::from(5.43416252072968490878938640132669983416252072968490f64),
                T::from(-3.48728837220254373837371627326518186619303751694540f64)
            ],
            polynomial![
                T,
                T::from(0.00000000000000000000000000000000000000000000000000f64),
                T::from(0.00000000000000000000000000000000000000000000000000f64),
                T::from(0.00000000000000000000000000000000000000000000000000f64),
                T::from(0.00000000000000000000000000000000000000000000000000f64),
                T::from(0.26909544328484081804764916719375922411975542905334f64)
            ],
            polynomial![
                T,
                T::from(0.00000000000000000000000000000000000000000000000000f64),
                T::from(0.00000000000000000000000000000000000000000000000000f64),
                T::from(0.45023809523809523809523809523809523809523809523810f64),
                T::from(-1.50079365079365079365079365079365079365079365079370f64),
                T::from(1.12559523809523809523809523809523809523809523809520f64)
            ],
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn interpolation_continuity_error<const S: usize, const S2: usize>(
        rk: &ExplicitRungeKuttaTable<S, S2, f64>,
    ) -> f64 {
        let mut max = 0f64;
        for i in 0..S {
            max = max.max((rk.b[i] - (rk.bi[i].0)(1.)).abs());
            max = max.max((rk.bi[i].0)(0.).abs());
        }
        max
    }

    fn c_is_sum_of_a_error<const S: usize, const S2: usize>(
        rk: &ExplicitRungeKuttaTable<S, S2, f64>,
    ) -> f64 {
        let mut max = 0f64;
        for i in 0..S {
            let sum = ((i * i - i) / 2..).take(i).map(|j| rk.a[j]).sum::<f64>();
            let diff = (rk.c[i] - sum).abs();
            max = f64::max(max, diff);
        }
        max
    }

    fn order_conditions_error<const S: usize, const S2: usize>(
        rk: &ExplicitRungeKuttaTable<S, S2, f64>,
    ) -> f64 {
        // On derivation and formulas for order conditions, see
        // E. Hairer, G. Wanner, and S. P. Nørsett, Solving ordinary differential equations I, vol. 8. in Springer Series in Computational Mathematics, vol. 8. Berlin, Heidelberg: Springer Berlin Heidelberg, 1993. doi: 10.1007/978-3-540-78862-1.
        let mut max_error = 0.;

        // order is q in the mentioned text
        for order in 1..=rk.order {
            let advance_tree = |tree: &mut [usize]| -> bool {
                let mut i_of_increased = order - 1;

                // while cannot increase in this index
                while i_of_increased > 0 && tree[i_of_increased] + 1 >= i_of_increased {
                    i_of_increased -= 1;
                }

                if i_of_increased > 0 {
                    tree[i_of_increased] += 1;
                    for i in (i_of_increased + 1)..order {
                        tree[i] = tree[i_of_increased];
                    }
                    return true;
                } else {
                    return false;
                }
            };

            let fix_indexes = |tree: &[usize], indexes: &mut [usize]| {
                for i in (1..order).rev() {
                    indexes[tree[i]] = usize::max(indexes[tree[i]], indexes[i] + 1);
                }
            };

            let advance_indexes = |tree: &[usize], indexes: &mut [usize]| -> bool {
                let mut i_of_increased = order - 1;
                while i_of_increased > 0
                    && indexes[i_of_increased] + 1 >= indexes[tree[i_of_increased]]
                {
                    i_of_increased -= 1;
                }

                indexes[i_of_increased] += 1;
                for i in (i_of_increased + 1)..order {
                    indexes[i] = 0;
                }
                for i in ((i_of_increased + 1)..order).rev() {
                    indexes[tree[i]] = usize::max(indexes[tree[i]], indexes[i] + 1);
                }
                if i_of_increased > 0 {
                    return true;
                } else {
                    return false;
                }
            };

            let gamma = |tree: &[usize]| -> f64 {
                let mut rho = vec![0f64; order];
                for i in (1..order).rev() {
                    rho[i] += 1.;
                    rho[tree[i]] += rho[i];
                }
                rho[0] += 1.;

                rho.iter().product()
            };

            let mut tree = vec![0usize; rk.order];

            // index[0] in 0..order, index[j] in 0..index[tree[j]]
            let mut indexes = vec![0usize; rk.order];

            loop
            /* for all trees */
            {
                fix_indexes(&tree, &mut indexes);

                let mut b_phi_sum = 0.;
                let mut b_phi_sum_embedded = 0.;

                println!("{:?}", &tree[..order]);
                for j in indexes[0]..S {
                    // j = 0 => sums are empty
                    indexes.fill(0);
                    indexes[0] = j;
                    fix_indexes(&tree, &mut indexes);

                    let mut phi_j = 0.;

                    loop
                    /* for all index packs starting with j */
                    {
                        // println!("\t{:?}", indexes);
                        phi_j += (1..order).fold(1., |acc, i| {
                            acc * rk.a_indexed(indexes[tree[i]], indexes[i])
                        });

                        if !advance_indexes(&tree, &mut indexes) {
                            indexes.fill(0);
                            break;
                        }
                    }
                    b_phi_sum += phi_j * rk.b[j];
                    b_phi_sum_embedded += phi_j * rk.b2[j];
                }

                let error = f64::abs(b_phi_sum - 1. / gamma(&tree));
                let error_embedded = f64::abs(b_phi_sum_embedded - 1. / gamma(&tree));
                max_error = f64::max(max_error, error);
                if order <= rk.order_embedded {
                    max_error = f64::max(max_error, error_embedded);
                }

                println!("\t{:?}", error);
                println!("\t{:?}", error_embedded);

                if !advance_tree(&mut tree) {
                    break;
                }
            }
        }

        max_error
    }

    macro_rules! test_rk {
        ($name:ident, $tolerance:expr) => {
            #[test]
            fn $name() {
                let rk = super::$name::<f64>();
                // assert_a_has_correct_sizes(&$RK);
                let interpolatiion_continuinuity_error_val = interpolation_continuity_error(&rk);
                let c_is_sum_of_a_error_val = c_is_sum_of_a_error(&rk);
                let order_conditions_error_val = order_conditions_error(&rk);
                println!("tolerance: {}", $tolerance);
                println!("interplation_error: {}", interpolatiion_continuinuity_error_val);
                println!("c_sum_error: {}", c_is_sum_of_a_error_val);
                println!("order_coditions_error: {}", order_conditions_error_val);
                assert!(interpolatiion_continuinuity_error_val < $tolerance);
                assert!(c_is_sum_of_a_error_val < $tolerance);
                assert!(order_conditions_error_val < $tolerance);
            }
        };
    }

    test_rk!(euler, 1e-15);

    test_rk!(heun2, 1e-15);
    test_rk!(ralston2, 1e-15);
    test_rk!(midpoint, 1e-15);

    test_rk!(ssp3, 1e-15);
    test_rk!(heun3, 1e-15);
    test_rk!(wray3, 1e-15);
    test_rk!(kutta3, 1e-15);
    test_rk!(ralston3, 1e-15);

    test_rk!(classic4, 1e-15);
    test_rk!(classic43, 1e-15);

    // fails
    // test_rk!(rk547fm, 1e-11);

    test_rk!(rktp64, 1e-15);

    // #[test]
    // fn rk98_exept_order_conditions() {
    //     // assert_a_has_correct_sizes(&RK98);
    //     assert!(interpolation_continuity_error(&RK98) < 1e-9);
    //     assert!(c_is_sum_of_a_error(&RK98) < 1e-9);
    //     assert!(order_conditions_error(&RK98, [(); 5]) < 1e-7);
    // }
    //
    // #[test]
    // #[ignore] // because takes too long (1.5 hours for 7th order or smth)
    // fn rk98_order_conditions() {
    //     assert!(order_conditions_error(&RK98, [(); RK98.order]) < 1e-15);
    // }
}
