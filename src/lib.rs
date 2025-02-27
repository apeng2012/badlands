use nalgebra::DMatrix;

pub mod chapter_1;

/// 计算矩阵中指定位置的余子式。
///
/// # 参数
/// - `matrix`: 输入的方阵，类型为 `DMatrix<f64>`。
/// - `row`: 要移除的行索引。
/// - `col`: 要移除的列索引。
///
/// # 返回值
/// - 返回一个 `Option<f64>`：
///   - 如果输入矩阵是方阵且行和列索引有效，则返回余子式的值。
///   - 如果输入矩阵不是方阵或行/列索引无效，则返回 `None`。
///
/// # 示例
/// ```
/// use nalgebra::DMatrix;
/// use badlands::minor;
///
/// let matrix = DMatrix::from_row_slice(3, 3, &[
///     1.0, 2.0, 3.0,
///     4.0, 5.0, 6.0,
///     7.0, 8.0, 9.0,
/// ]);
///
/// // 计算第2行第3列的余子式
/// if let Some(minor_value) = minor(&matrix, 2, 3) {
///     println!("余子式的值为: {}", minor_value); // 输出: -6.0
/// } else {
///     println!("输入无效！");
/// }
/// ```
pub fn minor(matrix: &DMatrix<f64>, row: usize, col: usize) -> Option<f64> {
    // 检查输入矩阵是否为方阵
    if matrix.nrows() != matrix.ncols() {
        return None;
    }

    // 检查行和列索引是否有效
    if row == 0 || row > matrix.nrows() || col == 0 || col > matrix.ncols() {
        return None;
    }

    // 移除指定的行和列，得到子矩阵
    let sub_matrix = matrix.clone().remove_row(row - 1).remove_column(col - 1);

    // 计算子矩阵的行列式
    Some(sub_matrix.determinant())
}

/// 计算矩阵中指定位置的代数余子式。
///
/// # 参数
/// - `matrix`: 输入的方阵，类型为 `&DMatrix<f64>`。
/// - `row`: 要计算的行索引（从 1 开始）。
/// - `col`: 要计算的列索引（从 1 开始）。
///
/// # 返回值
/// - 返回一个 `Option<f64>`：
///   - 如果输入矩阵不是方阵，或者行/列索引无效，则返回 `None`。
///   - 否则，返回代数余子式的值。
///
/// # 示例
/// ```
/// use nalgebra::DMatrix;
/// let matrix = DMatrix::from_row_slice(3, 3, &[
///     1.0, 2.0, 3.0,
///     4.0, 5.0, 6.0,
///     7.0, 8.0, 9.0,
/// ]);
/// let cofactor_value = cofactor(&matrix, 2, 2);
/// assert_eq!(cofactor_value, Some(-3.0));
/// ```
pub fn cofactor(matrix: &DMatrix<f64>, row: usize, col: usize) -> Option<f64> {
    minor(matrix, row, col).map(|m| if (row + col) % 2 == 0 { m } else { -1.0 * m })
}
