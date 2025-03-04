mod tests {
    use nalgebra::RowVector3;

    /// Exercise 13
    /// 设有行列式
    ///
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
}
