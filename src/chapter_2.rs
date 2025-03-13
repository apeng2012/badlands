mod tests {
    use nalgebra::{matrix, Matrix3, RowVector3, SVD};

    /// Exercise 13
    /// 设
    ///     a = (1, 1, 1)
    ///
    /// 则 |a^Ta| = 0.0
    #[test]
    fn exercise_13() {
        let a = RowVector3::new(1.0, 1.0, 1.0);
        let a_transpose_a = a.transpose() * a;
        let result = a_transpose_a.determinant();
        assert_eq!(result, 0.0);
    }

    /// Exercise 19
    /// 设
    ///         [3 0 0]
    ///     a = [1 4 0]
    ///         [0 0 3]
    ///
    /// 则 (A-2E)^{-1} = ?
    #[test]
    fn exercise_19() {
        let a = Matrix3::new(3.0, 0.0, 0.0, 1.0, 4.0, 0.0, 0.0, 0.0, 3.0);
        let a_inverse = (a - Matrix3::identity() * 2.0).try_inverse();
        assert_eq!(
            a_inverse,
            Some(Matrix3::new(1.0, 0.0, 0.0, -0.5, 0.5, 0.0, 0.0, 0.0, 1.0))
        );
    }

    /// Exercise 23
    /// 矩阵
    ///         [2 -1  2]
    ///         [4  0  2]
    ///         [0 -3  3]
    ///
    /// 的秩为？
    #[test]
    fn exercise_23() {
        let matrix = Matrix3::new(2.0, -1.0, 2.0, 4.0, 0.0, 2.0, 0.0, -3.0, 3.0);
        let svd = SVD::new(matrix, true, true);
        let singular_values = svd.singular_values;
        let rank = singular_values.iter().filter(|&&x| x > 1e-10).count();
        assert_eq!(rank, 2);
    }

    /// Exercise 62
    /// 矩阵
    ///         [5 2 0]
    ///     A = [1 3 0]
    ///         [0 0 4]
    ///
    /// 满足 AB = A + 2B
    /// 求 B
    #[test]
    fn exercise_62() {
        let a = Matrix3::new(5.0, 2.0, 0.0, 1.0, 3.0, 0.0, 0.0, 0.0, 4.0);
        let a_minus_2i: Matrix3<f64> = a - 2.0 * Matrix3::identity();
        let a_minus_2i_inv = a_minus_2i.try_inverse().unwrap();
        let b = a * a_minus_2i_inv;
        assert_eq!(
            b,
            Matrix3::new(3.0, -4.0, 0.0, -2.0, 7.0, 0.0, 0.0, 0.0, 2.0)
        );
    }

    /// Exercise 78
    /// 矩阵
    ///         [2 1 -1]
    ///     A = [2 1 0]
    ///         [1 -1 1]
    ///
    ///     B = [1 -1 3]
    ///         [4 3 2]
    ///
    /// 满足 XA = B
    /// 求 X
    #[test]
    fn exercise_78() {
        let a = matrix![2.0, 1.0, -1.0;
                                                            2.0, 1.0, 0.0;
                                                            1.0, -1.0, 1.0];

        let b = matrix![1.0, -1.0, 3.0;
                                                            4.0, 3.0, 2.0];
        let a_inv: Matrix3<f64> = a.try_inverse().unwrap();
        let x = b * a_inv;
        assert!(x.relative_eq(
            &matrix![-2.0, 2.0, 1.0;
                     -8.0/3.0, 5.0, -2.0/3.0],
            1e-6,
            1e-6
        ));
    }
}
