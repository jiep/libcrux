use crate::{
    arithmetic::FieldElementTimesMontgomeryR,
    constants::COEFFICIENTS_IN_RING_ELEMENT,
    polynomial::PolynomialRingElement,
    simd::traits::{montgomery_multiply_by_fer, Operations, COEFFICIENTS_IN_SIMD_UNIT},
};

const ZETAS_TIMES_MONTGOMERY_R: [FieldElementTimesMontgomeryR; 256] = [
    0, 25847, -2608894, -518909, 237124, -777960, -876248, 466468, 1826347, 2353451, -359251,
    -2091905, 3119733, -2884855, 3111497, 2680103, 2725464, 1024112, -1079900, 3585928, -549488,
    -1119584, 2619752, -2108549, -2118186, -3859737, -1399561, -3277672, 1757237, -19422, 4010497,
    280005, 2706023, 95776, 3077325, 3530437, -1661693, -3592148, -2537516, 3915439, -3861115,
    -3043716, 3574422, -2867647, 3539968, -300467, 2348700, -539299, -1699267, -1643818, 3505694,
    -3821735, 3507263, -2140649, -1600420, 3699596, 811944, 531354, 954230, 3881043, 3900724,
    -2556880, 2071892, -2797779, -3930395, -1528703, -3677745, -3041255, -1452451, 3475950,
    2176455, -1585221, -1257611, 1939314, -4083598, -1000202, -3190144, -3157330, -3632928, 126922,
    3412210, -983419, 2147896, 2715295, -2967645, -3693493, -411027, -2477047, -671102, -1228525,
    -22981, -1308169, -381987, 1349076, 1852771, -1430430, -3343383, 264944, 508951, 3097992,
    44288, -1100098, 904516, 3958618, -3724342, -8578, 1653064, -3249728, 2389356, -210977, 759969,
    -1316856, 189548, -3553272, 3159746, -1851402, -2409325, -177440, 1315589, 1341330, 1285669,
    -1584928, -812732, -1439742, -3019102, -3881060, -3628969, 3839961, 2091667, 3407706, 2316500,
    3817976, -3342478, 2244091, -2446433, -3562462, 266997, 2434439, -1235728, 3513181, -3520352,
    -3759364, -1197226, -3193378, 900702, 1859098, 909542, 819034, 495491, -1613174, -43260,
    -522500, -655327, -3122442, 2031748, 3207046, -3556995, -525098, -768622, -3595838, 342297,
    286988, -2437823, 4108315, 3437287, -3342277, 1735879, 203044, 2842341, 2691481, -2590150,
    1265009, 4055324, 1247620, 2486353, 1595974, -3767016, 1250494, 2635921, -3548272, -2994039,
    1869119, 1903435, -1050970, -1333058, 1237275, -3318210, -1430225, -451100, 1312455, 3306115,
    -1962642, -1279661, 1917081, -2546312, -1374803, 1500165, 777191, 2235880, 3406031, -542412,
    -2831860, -1671176, -1846953, -2584293, -3724270, 594136, -3776993, -2013608, 2432395, 2454455,
    -164721, 1957272, 3369112, 185531, -1207385, -3183426, 162844, 1616392, 3014001, 810149,
    1652634, -3694233, -1799107, -3038916, 3523897, 3866901, 269760, 2213111, -975884, 1717735,
    472078, -426683, 1723600, -1803090, 1910376, -1667432, -1104333, -260646, -3833893, -2939036,
    -2235985, -420899, -2286327, 183443, -976891, 1612842, -3545687, -554416, 3919660, -48306,
    -1362209, 3937738, 1400424, -846154, 1976782,
];

#[inline(always)]
fn ntt_at_layer_0<SIMDUnit: Operations>(re: &mut PolynomialRingElement<SIMDUnit>) {
    macro_rules! round {
        ($i:literal, $zeta_0:literal, $zeta_1:literal, $zeta_2:literal, $zeta_3:literal) => {
            re.simd_units[$i] =
                SIMDUnit::ntt_at_layer_0(re.simd_units[$i], $zeta_0, $zeta_1, $zeta_2, $zeta_3);
        };
    }

    round!(0, 2091667, 3407706, 2316500, 3817976);
    round!(1, -3342478, 2244091, -2446433, -3562462);
    round!(2, 266997, 2434439, -1235728, 3513181);
    round!(3, -3520352, -3759364, -1197226, -3193378);
    round!(4, 900702, 1859098, 909542, 819034);
    round!(5, 495491, -1613174, -43260, -522500);
    round!(6, -655327, -3122442, 2031748, 3207046);
    round!(7, -3556995, -525098, -768622, -3595838);
    round!(8, 342297, 286988, -2437823, 4108315);
    round!(9, 3437287, -3342277, 1735879, 203044);
    round!(10, 2842341, 2691481, -2590150, 1265009);
    round!(11, 4055324, 1247620, 2486353, 1595974);
    round!(12, -3767016, 1250494, 2635921, -3548272);
    round!(13, -2994039, 1869119, 1903435, -1050970);
    round!(14, -1333058, 1237275, -3318210, -1430225);
    round!(15, -451100, 1312455, 3306115, -1962642);
    round!(16, -1279661, 1917081, -2546312, -1374803);
    round!(17, 1500165, 777191, 2235880, 3406031);
    round!(18, -542412, -2831860, -1671176, -1846953);
    round!(19, -2584293, -3724270, 594136, -3776993);
    round!(20, -2013608, 2432395, 2454455, -164721);
    round!(21, 1957272, 3369112, 185531, -1207385);
    round!(22, -3183426, 162844, 1616392, 3014001);
    round!(23, 810149, 1652634, -3694233, -1799107);
    round!(24, -3038916, 3523897, 3866901, 269760);
    round!(25, 2213111, -975884, 1717735, 472078);
    round!(26, -426683, 1723600, -1803090, 1910376);
    round!(27, -1667432, -1104333, -260646, -3833893);
    round!(28, -2939036, -2235985, -420899, -2286327);
    round!(29, 183443, -976891, 1612842, -3545687);
    round!(30, -554416, 3919660, -48306, -1362209);
    round!(31, 3937738, 1400424, -846154, 1976782);
}

#[inline(always)]
fn ntt_at_layer_1<SIMDUnit: Operations>(re: &mut PolynomialRingElement<SIMDUnit>) {
    macro_rules! round {
        ($i:literal, $zeta_0:literal, $zeta_1:literal) => {
            re.simd_units[$i] = SIMDUnit::ntt_at_layer_1(re.simd_units[$i], $zeta_0, $zeta_1);
        };
    }

    round!(0, -3930395, -1528703);
    round!(1, -3677745, -3041255);
    round!(2, -1452451, 3475950);
    round!(3, 2176455, -1585221);
    round!(4, -1257611, 1939314);
    round!(5, -4083598, -1000202);
    round!(6, -3190144, -3157330);
    round!(7, -3632928, 126922);
    round!(8, 3412210, -983419);
    round!(9, 2147896, 2715295);
    round!(10, -2967645, -3693493);
    round!(11, -411027, -2477047);
    round!(12, -671102, -1228525);
    round!(13, -22981, -1308169);
    round!(14, -381987, 1349076);
    round!(15, 1852771, -1430430);
    round!(16, -3343383, 264944);
    round!(17, 508951, 3097992);
    round!(18, 44288, -1100098);
    round!(19, 904516, 3958618);
    round!(20, -3724342, -8578);
    round!(21, 1653064, -3249728);
    round!(22, 2389356, -210977);
    round!(23, 759969, -1316856);
    round!(24, 189548, -3553272);
    round!(25, 3159746, -1851402);
    round!(26, -2409325, -177440);
    round!(27, 1315589, 1341330);
    round!(28, 1285669, -1584928);
    round!(29, -812732, -1439742);
    round!(30, -3019102, -3881060);
    round!(31, -3628969, 3839961);
}

#[inline(always)]
fn ntt_at_layer_2<SIMDUnit: Operations>(re: &mut PolynomialRingElement<SIMDUnit>) {
    macro_rules! round {
        ($i:literal, $zeta:literal) => {
            re.simd_units[$i] = SIMDUnit::ntt_at_layer_2(re.simd_units[$i], $zeta);
        };
    }

    round!(0, 2706023);
    round!(1, 95776);
    round!(2, 3077325);
    round!(3, 3530437);
    round!(4, -1661693);
    round!(5, -3592148);
    round!(6, -2537516);
    round!(7, 3915439);
    round!(8, -3861115);
    round!(9, -3043716);
    round!(10, 3574422);
    round!(11, -2867647);
    round!(12, 3539968);
    round!(13, -300467);
    round!(14, 2348700);
    round!(15, -539299);
    round!(16, -1699267);
    round!(17, -1643818);
    round!(18, 3505694);
    round!(19, -3821735);
    round!(20, 3507263);
    round!(21, -2140649);
    round!(22, -1600420);
    round!(23, 3699596);
    round!(24, 811944);
    round!(25, 531354);
    round!(26, 954230);
    round!(27, 3881043);
    round!(28, 3900724);
    round!(29, -2556880);
    round!(30, 2071892);
    round!(31, -2797779);
}

#[inline(always)]
fn outer_3_plus<
    SIMDUnit: Operations,
    const OFFSET: usize,
    const STEP_BY: usize,
    const ZETA: FieldElementTimesMontgomeryR,
>(
    re: &mut PolynomialRingElement<SIMDUnit>,
) {
    for j in OFFSET..OFFSET + STEP_BY {
        let t = montgomery_multiply_by_fer::<SIMDUnit>(re.simd_units[j + STEP_BY], ZETA);

        re.simd_units[j + STEP_BY] = SIMDUnit::subtract(&re.simd_units[j], &t);
        re.simd_units[j] = SIMDUnit::add(&re.simd_units[j], &t);
    }
}

#[inline(always)]
fn ntt_at_layer_3<SIMDUnit: Operations>(re: &mut PolynomialRingElement<SIMDUnit>) {
    // const LAYER: usize = 3;
    // const ITERATIONS: usize = 16; // 128 >> LAYER

    const STEP: usize = 8; // 1 << LAYER;
    const STEP_BY: usize = 1; // step / COEFFICIENTS_IN_SIMD_UNIT;

    outer_3_plus::<SIMDUnit, { (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 2725464>(re);
    outer_3_plus::<SIMDUnit, { (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 1024112>(re);
    outer_3_plus::<SIMDUnit, { (2 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -1079900>(re);
    outer_3_plus::<SIMDUnit, { (3 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 3585928>(re);
    outer_3_plus::<SIMDUnit, { (4 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -549488>(re);
    outer_3_plus::<SIMDUnit, { (5 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -1119584>(re);
    outer_3_plus::<SIMDUnit, { (6 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 2619752>(re);
    outer_3_plus::<SIMDUnit, { (7 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2108549>(re);
    outer_3_plus::<SIMDUnit, { (8 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2118186>(re);
    outer_3_plus::<SIMDUnit, { (9 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -3859737>(re);
    outer_3_plus::<SIMDUnit, { (10 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -1399561>(
        re,
    );
    outer_3_plus::<SIMDUnit, { (11 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -3277672>(
        re,
    );
    outer_3_plus::<SIMDUnit, { (12 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 1757237>(re);
    outer_3_plus::<SIMDUnit, { (13 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -19422>(re);
    outer_3_plus::<SIMDUnit, { (14 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 4010497>(re);
    outer_3_plus::<SIMDUnit, { (15 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 280005>(re);
}

#[inline(always)]
fn ntt_at_layer_4<SIMDUnit: Operations>(re: &mut PolynomialRingElement<SIMDUnit>) {
    // const LAYER: usize = 4;
    // const ITERATIONS: usize = 8; // 128 >> LAYER

    const STEP: usize = 16; // 1 << LAYER;
    const STEP_BY: usize = 2; // step / COEFFICIENTS_IN_SIMD_UNIT;

    outer_3_plus::<SIMDUnit, { (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 1826347>(re);
    outer_3_plus::<SIMDUnit, { (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 2353451>(re);
    outer_3_plus::<SIMDUnit, { (2 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -359251>(re);
    outer_3_plus::<SIMDUnit, { (3 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2091905>(re);
    outer_3_plus::<SIMDUnit, { (4 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 3119733>(re);
    outer_3_plus::<SIMDUnit, { (5 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2884855>(re);
    outer_3_plus::<SIMDUnit, { (6 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 3111497>(re);
    outer_3_plus::<SIMDUnit, { (7 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 2680103>(re);
}

#[inline(always)]
fn ntt_at_layer_5<SIMDUnit: Operations>(re: &mut PolynomialRingElement<SIMDUnit>) {
    // const LAYER: usize = 5;
    // const ITERATIONS: usize = 4; // 128 >> LAYER

    const STEP: usize = 32; // 1 << LAYER;
    const STEP_BY: usize = 4; // step / COEFFICIENTS_IN_SIMD_UNIT;

    outer_3_plus::<SIMDUnit, { (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 237124>(re);
    outer_3_plus::<SIMDUnit, { (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -777960>(re);
    outer_3_plus::<SIMDUnit, { (2 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -876248>(re);
    outer_3_plus::<SIMDUnit, { (3 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 466468>(re);
}

#[inline(always)]
fn ntt_at_layer_6<SIMDUnit: Operations>(re: &mut PolynomialRingElement<SIMDUnit>) {
    // const LAYER: usize = 6;
    // const ITERATIONS: usize = 2; // 128 >> LAYER

    const STEP: usize = 64; // 1 << LAYER;
    const STEP_BY: usize = 8; // step / COEFFICIENTS_IN_SIMD_UNIT;

    outer_3_plus::<SIMDUnit, { (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2608894>(re);
    outer_3_plus::<SIMDUnit, { (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -518909>(re);
}

#[inline(always)]
fn ntt_at_layer_7<SIMDUnit: Operations>(re: &mut PolynomialRingElement<SIMDUnit>) {
    // const LAYER: usize = 7;
    // const ITERATIONS: usize = 1; // 128 >> LAYER

    const STEP: usize = 128; // 1 << LAYER;
    const STEP_BY: usize = 16; // step / COEFFICIENTS_IN_SIMD_UNIT;

    outer_3_plus::<SIMDUnit, { (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 25847>(re);
}

#[inline(always)]
pub(crate) fn ntt<SIMDUnit: Operations>(
    mut re: PolynomialRingElement<SIMDUnit>,
) -> PolynomialRingElement<SIMDUnit> {
    ntt_at_layer_7::<SIMDUnit>(&mut re);
    ntt_at_layer_6::<SIMDUnit>(&mut re);
    ntt_at_layer_5::<SIMDUnit>(&mut re);
    ntt_at_layer_4::<SIMDUnit>(&mut re);
    ntt_at_layer_3::<SIMDUnit>(&mut re);
    ntt_at_layer_2::<SIMDUnit>(&mut re);
    ntt_at_layer_1::<SIMDUnit>(&mut re);
    ntt_at_layer_0::<SIMDUnit>(&mut re);

    re
}

#[inline(always)]
fn invert_ntt_at_layer_0<SIMDUnit: Operations>(
    zeta_i: &mut usize,
    re: &mut PolynomialRingElement<SIMDUnit>,
) {
    *zeta_i -= 1;

    for round in 0..re.simd_units.len() {
        re.simd_units[round] = SIMDUnit::invert_ntt_at_layer_0(
            re.simd_units[round],
            ZETAS_TIMES_MONTGOMERY_R[*zeta_i],
            ZETAS_TIMES_MONTGOMERY_R[*zeta_i - 1],
            ZETAS_TIMES_MONTGOMERY_R[*zeta_i - 2],
            ZETAS_TIMES_MONTGOMERY_R[*zeta_i - 3],
        );

        *zeta_i -= 4;
    }

    *zeta_i += 1;
}

#[inline(always)]
fn invert_ntt_at_layer_1<SIMDUnit: Operations>(
    zeta_i: &mut usize,
    re: &mut PolynomialRingElement<SIMDUnit>,
) {
    *zeta_i -= 1;

    for round in 0..(256 / COEFFICIENTS_IN_SIMD_UNIT) {
        re.simd_units[round] = SIMDUnit::invert_ntt_at_layer_1(
            re.simd_units[round],
            ZETAS_TIMES_MONTGOMERY_R[*zeta_i],
            ZETAS_TIMES_MONTGOMERY_R[*zeta_i - 1],
        );
        *zeta_i -= 2;
    }

    *zeta_i += 1;
}

#[inline(always)]
fn invert_ntt_at_layer_2<SIMDUnit: Operations>(
    zeta_i: &mut usize,
    re: &mut PolynomialRingElement<SIMDUnit>,
) {
    for round in 0..(256 / COEFFICIENTS_IN_SIMD_UNIT) {
        *zeta_i -= 1;
        re.simd_units[round] = SIMDUnit::invert_ntt_at_layer_2(
            re.simd_units[round],
            ZETAS_TIMES_MONTGOMERY_R[*zeta_i],
        );
    }
}

#[inline(always)]
fn invert_ntt_at_layer_3_plus<SIMDUnit: Operations, const LAYER: usize>(
    zeta_i: &mut usize,
    re: &mut PolynomialRingElement<SIMDUnit>,
) {
    let step = 1 << LAYER;

    for round in 0..(128 >> LAYER) {
        *zeta_i -= 1;

        let offset = (round * step * 2) / COEFFICIENTS_IN_SIMD_UNIT;
        let step_by = step / COEFFICIENTS_IN_SIMD_UNIT;

        for j in offset..offset + step_by {
            let a_minus_b = SIMDUnit::subtract(&re.simd_units[j + step_by], &re.simd_units[j]);
            re.simd_units[j] = SIMDUnit::add(&re.simd_units[j], &re.simd_units[j + step_by]);
            re.simd_units[j + step_by] =
                montgomery_multiply_by_fer(a_minus_b, ZETAS_TIMES_MONTGOMERY_R[*zeta_i]);
        }
    }
}

#[inline(always)]
pub(crate) fn invert_ntt_montgomery<SIMDUnit: Operations>(
    mut re: PolynomialRingElement<SIMDUnit>,
) -> PolynomialRingElement<SIMDUnit> {
    let mut zeta_i = COEFFICIENTS_IN_RING_ELEMENT;

    invert_ntt_at_layer_0(&mut zeta_i, &mut re);
    invert_ntt_at_layer_1(&mut zeta_i, &mut re);
    invert_ntt_at_layer_2(&mut zeta_i, &mut re);
    invert_ntt_at_layer_3_plus::<SIMDUnit, 3>(&mut zeta_i, &mut re);
    invert_ntt_at_layer_3_plus::<SIMDUnit, 4>(&mut zeta_i, &mut re);
    invert_ntt_at_layer_3_plus::<SIMDUnit, 5>(&mut zeta_i, &mut re);
    invert_ntt_at_layer_3_plus::<SIMDUnit, 6>(&mut zeta_i, &mut re);
    invert_ntt_at_layer_3_plus::<SIMDUnit, 7>(&mut zeta_i, &mut re);

    for i in 0..re.simd_units.len() {
        // After invert_ntt_at_layer, elements are of the form a * MONTGOMERY_R^{-1}
        // we multiply by (MONTGOMERY_R^2) * (1/2^8) mod Q = 41,978 to both:
        //
        // - Divide the elements by 256 and
        // - Convert the elements form montgomery domain to the standard domain.
        re.simd_units[i] = SIMDUnit::montgomery_multiply_by_constant(re.simd_units[i], 41_978);
    }

    re
}

#[inline(always)]
pub(crate) fn ntt_multiply_montgomery<SIMDUnit: Operations>(
    lhs: &PolynomialRingElement<SIMDUnit>,
    rhs: &PolynomialRingElement<SIMDUnit>,
) -> PolynomialRingElement<SIMDUnit> {
    let mut out = PolynomialRingElement::ZERO();

    for i in 0..out.simd_units.len() {
        out.simd_units[i] = SIMDUnit::montgomery_multiply(lhs.simd_units[i], rhs.simd_units[i]);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{polynomial::PolynomialRingElement, simd::traits::Operations};

    fn test_ntt_generic<SIMDUnit: Operations>() {
        let coefficients = [
            245230, -429681, -35753, 256940, 138755, -82158, -453212, -296769, 106884, -496329,
            -275542, 350156, 295061, 462432, 162727, 219494, 43263, -84315, -100731, 5560, -38846,
            343612, 76881, 427547, 165700, -361163, -18964, 270770, -289948, -326181, -17540,
            -376674, -101359, 324588, 265493, -376942, -270029, -201717, -350446, 222164, -314686,
            -60609, 172509, -199265, 391809, 375196, 333441, -433240, -28862, 274251, -218805,
            400627, -408915, 131269, -305167, 78967, -487687, 98675, -430105, 293491, 317484,
            -180888, -333359, -263010, 258853, -84618, -350795, 334736, -438451, 479262, -265874,
            -115692, -521929, -220715, -456043, -24131, 94695, 473893, -503297, 75679, -129421,
            83315, -248504, -64226, -24884, 316438, 264565, -248440, 222228, -386736, 89534,
            196079, -196063, 434306, -388976, -29596, 424028, 290804, -348654, 208245, 394447,
            -105640, -522040, 250479, -443666, -503110, 299944, 497539, -28052, 30579, -332034,
            492009, -327080, -173581, -94157, -126088, 388734, 468785, -120589, -146970, -291234,
            337402, 311007, -289990, 506654, -431388, 410292, -376624, 422627, 246536, -273872,
            443039, -265954, -250947, 451185, -386654, -19185, -171927, -128698, -277965, 142565,
            -229030, -470985, 511916, -68612, 272580, -293969, 151888, -53429, -115171, 234680,
            -482360, -399860, -268942, 146734, -414798, 502035, -157203, -328592, 266628, 95760,
            107840, 354606, -367167, 396086, -287062, 57888, 140152, 442747, -217984, -69604,
            -136006, -56581, 202803, -440282, -290558, -192319, -49121, -76454, 426678, 433484,
            -93094, 244295, -195275, -262446, -169118, 187824, -60480, -206921, -204671, -407794,
            -139194, -182819, 133480, 520760, -17757, -444106, -214471, 457449, 29697, -149734,
            -497293, -177518, -266611, -133962, -40139, 9030, 37706, 300290, -370302, 257446,
            -290991, 353260, 393727, -269498, 249049, -166327, -354566, -309239, 481747, 82459,
            -425894, 107583, 10935, -498533, 437188, -121594, -90890, -261475, -44165, 394580,
            -392499, 206781, -222053, -334528, -194081, -373973, -356982, -27220, -444980, 449174,
            -391807, 392057, -132521, -441664, -349459, -373059, -296519, 274235, 42417, 47385,
            -104540, 142532, 246380, -515363, -422665,
        ];
        let re = PolynomialRingElement::<SIMDUnit>::from_i32_array(&coefficients);

        let expected_coefficients = [
            -17129289, -17188287, -11027856, -7293060, -14589541, -12369669, -1420304, -9409026,
            -2745174, -2813844, -1829426, 2574100, -5026817, -9781421, -9951567, -7272515, 4818335,
            -3195023, -6970219, -7364953, 1800133, -219955, 5457527, -2421101, -2719347, 4851863,
            -5375188, -6373272, -6881235, 1470681, 2364683, 4847471, 2424421, -2276079, 2780402,
            3720484, 6345079, -150847, 4499295, 3841925, -4612874, 227272, -1650880, -4068714,
            1238348, -6241908, 674916, 8597432, 1045161, 2838309, -4022618, -8710072, -3036374,
            -3401044, -6864890, -4717312, -3844346, 3755766, 4699242, -1232858, -1007843, -2372141,
            -5151898, 2215126, 5056427, 5704699, 11731990, 12381420, 2784890, -2861996, 1452131,
            5933279, 4031780, 5298922, 3626052, 4969414, 3453854, -4627414, -1023658, -5769310,
            1437156, 1156658, -2817787, -8761943, -2668956, -9522412, -12938019, -10322153,
            -9811386, -8779334, 2078963, -4674611, -4110129, 2451543, -4834924, -2503578, -5536189,
            -1677443, -6867926, -4019342, -10584384, -7739886, -6447026, -13889812, -6819207,
            587959, -7563216, -14153360, -5061746, -11893138, -2225507, -1089121, -1869464,
            3296810, 6674836, -1150818, 324295, -509763, -1197550, -5578514, -5136666, -4382368,
            3113889, -3428119, 235128, 4223510, 70873, -1793487, 1662772, 7347100, 15227445,
            9348419, 9598008, 9940972, 7506539, 9092233, 4526452, 9976840, 6619274, 8638534,
            8098748, 4080374, 9497479, 9356635, -239442, 6155758, -2930736, -4891836, 2066938,
            7359172, 597336, 7980226, -1781310, -5283606, 596800, 3537228, -8539373, -4044371,
            -1411916, 4051564, 2598458, 9958426, 1194732, 9002276, -926584, 5985194, 980962,
            856944, 6456619, 8929175, 9047642, 12797200, 11248612, 4324864, 18190009, 10462927,
            4906049, 2341517, 3945796, 8377830, 5195877, 10702083, -247762, 4149842, -4852089,
            -1576975, 516061, 1908067, 2840273, -4492477, 9446409, 3700267, 346209, 2692483,
            -7029253, -5625659, 4093774, -3922644, 2578212, 6694254, -1244120, -1475796, -9388817,
            -5401831, -6934520, -8620440, -5385728, -6961628, -8648379, -2747757, -10439151,
            -5664161, -1208977, -8828047, -1715189, 5918789, 2038973, -5412689, 4197315, -3211379,
            12103869, 4104929, 3182052, 6094506, 1986313, -481257, -3678130, -673934, 2320744,
            1656034, -5630954, -3497176, 6334075, 11828589, 6053995, -1775095, 6687195, 7765831,
            7946592, 7821130, -2626065, 4613455, 10127838, 3728296, 9154301, 11337805, 8531104,
            15979738, 1459696, 8351548, 3335586, 1150210, -2462074, -4642922, 4538634, 1858098,
        ];

        assert_eq!(ntt(re).to_i32_array(), expected_coefficients);
    }

    fn test_invert_ntt_montgomery_generic<SIMDUnit: Operations>() {
        let coefficients = [
            -1799977, -2102152, -2642101, -635466, -1853482, 642462, 1199623, -2231752, -3968977,
            1443304, 1461464, 2556315, 4140492, -1725885, 4153465, -556916, -2133612, 1372025,
            3676714, 3519610, 706947, 788194, 3622849, -2734117, 3727454, -2190265, 208958,
            2555531, -1748893, -256927, 1863384, -135807, -2321243, 2766307, -707368, 1548297,
            1449797, 3892466, 3417597, -2439676, 1503122, -1273655, 4077536, 6648, -1763675,
            2151278, -2147862, -4095105, 1452564, -194892, 150869, -168533, 3172154, -4157552,
            950081, 1263047, 1073782, -2392089, 3913931, 2548808, -3641576, -133231, -345656,
            -3993400, -772374, 650580, 11714, 2745379, 4102554, -1493814, -1216073, -3687790,
            -520554, 674428, -2363771, -2062557, 1353060, 421679, -1736183, 3309070, 3705199,
            -1614110, -1885448, 1502936, -3904361, 2506554, 101679, -2500045, 2538220, 3019542,
            1264486, 1681771, 889126, 951808, -3807112, -1917333, 2530518, 2276961, 3921082,
            1553244, -2044159, -2836376, 498383, 2971233, 3286160, 1149491, -3659209, -1963092,
            -2566288, 2114154, 34024, -2989138, 424058, 3042007, -135014, -2866292, -1138173,
            -3010844, -2893275, -2118818, 3839605, 2956371, 2356470, 2895933, 390703, -1316703,
            -2882388, 3833928, 2118987, -2371764, 7210, 2032760, 770491, -2466615, -3672908,
            -2397815, -3106703, 3523515, 1794988, 2551854, -134246, -4189103, 3840541, -3703204,
            -2229747, 1599893, 1611447, -1126296, -1497526, 1422269, -1183163, 861126, -155866,
            1642344, 3459388, 2621579, 1200190, -1791368, -2396064, 3313131, -1704442, -1632644,
            3659167, 3290628, 1933900, 475446, 1952630, -847369, 2639611, 2205667, -767651,
            2248190, -1679262, 2250674, 3194928, 3674776, 2014792, -1384769, -2579573, 1424682,
            1150591, 1027245, -2676627, 3620918, -2364392, 971022, 170291, -16161, 3517252,
            -4070880, -2207879, -577017, 2484069, 927714, 2453609, -2953744, 3140280, 3160147,
            1667259, -3082713, -4047424, 124404, 3473451, 1419723, 161430, 483773, 2459342,
            1207398, 3486346, -2400797, 3217001, 2022150, -694480, -919315, -3442035, -1734814,
            -3231832, 2955471, 2104900, 1922217, 1829070, 1605538, 3862195, 1423572, 3831618,
            2188925, -967302, 677729, 3187197, 1048944, 1276467, -3329616, 3735664, 3795986,
            4038386, -3516780, -1902194, -880027, -1787327, -869158, 3693240, 494399, -3852589,
            -3881813, 2536840, -2924666, 2425664, 2635292, 2752536, -136653, 4057087, -633680,
            3039079, -2733512, 1734173, -2109687,
        ];
        let re = PolynomialRingElement::<SIMDUnit>::from_i32_array(&coefficients);

        let expected_coefficients = [
            3966085, -2067161, 579114, -3597478, 2232818, -17588, 1194752, -1205114, -4058138,
            1212005, -523747, -3757135, -2096288, 1564176, -2621702, 3098337, 1686358, 3045166,
            -190650, -3650792, -4016863, -3509278, 53081, -2698465, -3058034, 1934801, -489614,
            2562002, 135070, -561684, 1429883, 2143581, -2641675, -2638118, -2881420, 951375,
            -1178399, 3905449, -909202, 2293747, -977585, 2405262, -2582841, -3503339, 372978,
            -2217708, -2992060, -1261148, 1429205, -1436912, 1169879, 2688127, -1902970, -818037,
            1527388, 515446, -1660913, -1628614, 1155517, -2384683, 2424576, -207150, 3423525,
            196083, -1457572, 3843617, 670886, -3116174, -630147, 3833721, 162664, 1173694,
            3200069, 994675, -354381, 2157831, 3701560, -3878865, 3783818, -3698782, 2695001,
            3599085, 2818801, 1802598, 277871, 1672290, -928625, -1037863, 787843, -361648, 182577,
            3733189, -2641972, -3072669, 3466030, -2878519, -1137138, -2234722, 892883, -209264,
            -3945665, 1153968, -1994007, 1819301, -647462, 831906, 1571924, -1135087, 1990613,
            2944454, -2464655, 522799, -3957487, -3013253, -1137760, 1106259, 3564711, -2315418,
            -548862, -119514, -3611453, -3293829, 1519241, 1021839, -1511635, 1732685, 702257,
            -3656778, 3962669, -944275, 3309609, -1039174, 2265306, -3153610, -410668, -393039,
            -2356731, -4083957, 1859494, -2076440, -3697967, -2186461, 931673, -995414, -3309480,
            -1686811, -3134252, -680168, 378149, 3825792, -2700073, -1365989, 3367427, -2958570,
            2528663, 3774899, -1901765, -2885701, 286776, 2131014, 298955, -4037068, -3037990,
            2455918, 704405, -1034441, -1506766, 2257217, -3924289, 1910829, 2993230, -1731617,
            -4161994, 3826182, 3755775, 1410753, 302082, 4181290, 369048, -3123017, 1037659,
            2483263, -3207, 3709718, -1929249, 1661215, -1332343, 3103632, -1390082, 3718199,
            -2596263, 43403, -2068945, 1769551, 1148998, 3519758, 3484982, 1229675, -3917179,
            -1790200, 2942297, -367881, -2727323, 1780713, -972875, -4100902, 2103216, 1089969,
            3802059, -3600967, 3714015, 2262528, -836384, -4049058, -881894, -3019639, 1871325,
            -1127081, 2468781, 396133, -2210254, 1879546, 1761434, -2556875, -2260147, 2063043,
            593247, -1102091, 4017202, 1982539, 4103863, -3242, -1313258, -1128572, -1496667,
            3051626, 2072070, 1085473, 455772, 311363, -4073347, -1058544, 1001208, -3106675,
            1281322, 1054592, 2483921, -893262, 392334, 3052309, -3717274, 1212358, -4009407,
            -3909173, 1453538, -4079655,
        ];

        assert_eq!(
            invert_ntt_montgomery(re).to_i32_array(),
            expected_coefficients
        );
    }

    #[cfg(not(feature = "simd256"))]
    mod portable {
        use super::{test_invert_ntt_montgomery_generic, test_ntt_generic};

        #[test]
        fn test_ntt() {
            test_ntt_generic::<crate::simd::portable::PortableSIMDUnit>();
        }
        #[test]
        fn test_invert_ntt_montgomery() {
            test_invert_ntt_montgomery_generic::<crate::simd::portable::PortableSIMDUnit>();
        }
    }

    #[cfg(feature = "simd256")]
    mod avx2 {
        use super::{test_invert_ntt_montgomery_generic, test_ntt_generic};

        #[test]
        fn test_ntt() {
            test_ntt_generic::<crate::simd::avx2::AVX2SIMDUnit>();
        }
        #[test]
        fn test_invert_ntt_montgomery() {
            test_invert_ntt_montgomery_generic::<crate::simd::avx2::AVX2SIMDUnit>();
        }
    }
}
