mod tests {
    use nalgebra::{Matrix3, RowVector3};

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
}
