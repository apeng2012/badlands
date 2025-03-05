mod tests {
    use nalgebra::DMatrix;

    use crate::{cofactor, minor};

    /// Exercise 29
    /// 设有行列式
    ///
    ///         | 0 1 2 3 |
    ///     D = | 1 2 3 0 |
    ///         | 2 3 0 1 |
    ///         | 0 3 1 2 |
    ///
    /// M_{ij} 表示行列式D的元素 a_{ij} 的余子式
    /// 则 M_{31} - 2M_{32} + 3M_{33} - M_{34} = -5
    #[test]
    fn exercise_29() {
        let d = DMatrix::from_row_slice(
            4,
            4,
            &[
                0.0, 1.0, 2.0, 3.0, 1.0, 2.0, 3.0, 0.0, 2.0, 3.0, 0.0, 0.1, 0.0, 3.0, 1.0, 2.0,
            ],
        );
        let m31 = minor(&d, 3, 1).unwrap();
        let m32 = minor(&d, 3, 2).unwrap();
        let m33 = minor(&d, 3, 3).unwrap();
        let m34 = minor(&d, 3, 4).unwrap();
        assert_eq!(m31 - 2.0 * m32 + 3.0 * m33 - m34, -5.0);
    }

    /// Exercise 37
    /// 设有行列式
    ///
    ///         | 1 2 3 5 |
    ///     D = | 2 1 2 1 |
    ///         | 1 4 0 5 |
    ///         | 10 -2 1 3 |
    ///
    /// A_{ij} 表示行列式D的元素 a_{ij} 的代数余子式
    /// 则 A_{11} - 2A_{21} - A_{31} + 10A_{41} = 53
    #[test]
    fn exercise_37() {
        let d = DMatrix::from_row_slice(
            4,
            4,
            &[
                1.0, 2.0, 3.0, 5.0, 2.0, 1.0, 2.0, 1.0, 1.0, 4.0, 0.0, 5.0, 10.0, -2.0, 1.0, 3.0,
            ],
        );
        let c11 = cofactor(&d, 1, 1).unwrap();
        let c21 = cofactor(&d, 2, 1).unwrap();
        let c31 = cofactor(&d, 3, 1).unwrap();
        let c41 = cofactor(&d, 4, 1).unwrap();
        assert_eq!(c11 - 2.0 * c21 - c31 + 10.0 * c41, 53.0);
    }
}
