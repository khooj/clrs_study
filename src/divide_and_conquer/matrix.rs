use std::cell::RefCell;
use std::ops::{Add, Range, Sub};

type InnerData<const N: usize> = [[i32; N]; N];

#[derive(Debug, Clone)]
pub struct Matrix<const N: usize> {
    data: RefCell<InnerData<N>>,
}

mod view {
    // use super::MatrixData;
    use super::{BoxedMatrixWithView, InnerData, Matrix};
    use std::{
        cell::RefCell,
        ops::{Add, Range, Sub},
    };

    #[derive(Clone, Debug)]
    pub struct MatrixView<'a, const N: usize> {
        // probably need smth like &[&[i32]], but i struggle to construct it
        data: &'a RefCell<InnerData<N>>,
        pub rows: Range<usize>,
        pub columns: Range<usize>,
    }

    impl<'a, const N: usize> MatrixView<'a, N> {
        pub fn from_matrix(m: &'a Matrix<N>, rows: Range<usize>, columns: Range<usize>) -> Self {
            MatrixView::assert_data(&m.data, &rows, &columns);

            MatrixView {
                data: &m.data,
                rows,
                columns,
            }
        }

        fn assert_data(m: &'a RefCell<InnerData<N>>, rows: &Range<usize>, columns: &Range<usize>) {
            if rows.end > m.borrow().len() {
                panic!("rows range is not within matrix: {:?}", rows);
            }
            if columns.end > m.borrow().len() {
                panic!("columns range is not within matrix: {:?}", rows);
            }

            let n = rows.len();
            let n2 = columns.len();
            if n != n2 {
                panic!("ranges should be equal: {} != {}", n, n2);
            }
        }

        pub fn from_view(m: &MatrixView<'a, N>, rows: Range<usize>, columns: Range<usize>) -> Self {
            MatrixView::assert_data(&m.data, &rows, &columns);

            MatrixView {
                data: &m.data,
                rows,
                columns,
            }
        }

        #[cfg(test)]
        pub fn inner_cloned(&self) -> Vec<Vec<i32>> {
            self.data.borrow()[self.rows.start..self.rows.end]
                .iter()
                .map(|e| {
                    e[self.columns.start..self.columns.end]
                        .iter()
                        .map(|e| *e)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        }

        pub fn data(&self, i: usize, j: usize) -> i32 {
            self.assert_idx(i, j);
            self.data.borrow()[self.rows.start + i][self.columns.start + j]
        }

        fn assert_idx(&self, i: usize, j: usize) {
            let i = self.rows.start + i;
            let j = self.columns.start + j;
            if i >= self.rows.end || j >= self.columns.end {
                panic!(
                    "indexes exceeded: {} > {} or {} > {}",
                    i - self.rows.start,
                    self.rows.end,
                    j - self.columns.start,
                    self.columns.end
                );
            }
        }

        pub fn len(&self) -> usize {
            self.rows.len()
        }

        pub fn set_data(&mut self, i: usize, j: usize, v: i32) {
            self.assert_idx(i, j);
            self.data.borrow_mut()[self.rows.start + i][self.columns.start + j] = v;
        }

        pub fn set_matrix(
            &mut self,
            rows: Range<usize>,
            columns: Range<usize>,
            input: &MatrixView<'_, N>,
        ) {
            if rows.end > self.len() {
                panic!("rows exceed: {:?} > {}", rows, self.len());
            }

            if columns.end > self.len() {
                panic!("columns exceed: {:?} > {}", columns, self.len());
            }

            if rows.len() != input.len() {
                panic!("rows exceed input view: {:?} != {}", rows, input.len());
            }

            if columns.len() != input.len() {
                panic!(
                    "columns exceed input view: {:?} != {}",
                    columns,
                    input.len()
                );
            }

            for i in 0..input.len() {
                for j in 0..input.len() {
                    self.set_data(rows.start + i, columns.start + j, input.data(i, j));
                }
            }
        }

        pub fn set_self_matrix(&mut self, input: &MatrixView<'_, N>) {
            self.set_matrix(0..self.len(), 0..self.len(), input);
        }

        pub fn split_evenly(&self) -> (Self, Self, Self, Self) {
            let n = self.len();
            let rows1 = self.rows.start..self.rows.start + n / 2;
            let rows2 = self.rows.start + n / 2..self.rows.end;
            let columns1 = self.columns.start..self.columns.start + n / 2;
            let columns2 = self.columns.start + n / 2..self.columns.end;
            let c11 = MatrixView::from_view(&self, rows1.clone(), columns1.clone());
            let c12 = MatrixView::from_view(&self, rows1.clone(), columns2.clone());
            let c21 = MatrixView::from_view(&self, rows2.clone(), columns1);
            let c22 = MatrixView::from_view(&self, rows2, columns2);
            (c11, c12, c21, c22)
        }
    }

    impl<const N: usize> std::fmt::Display for MatrixView<'_, N> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[")?;
            for i in 0..self.len() {
                write!(f, "[")?;
                for j in 0..self.len() {
                    write!(f, "{},", self.data(i, j))?;
                }
                write!(f, "],")?;
            }
            Ok(())
        }
    }

    impl<'a, const N: usize> Add for MatrixView<'a, N> {
        type Output = BoxedMatrixWithView<N>;

        fn add(self, rhs: MatrixView<'a, N>) -> Self::Output {
            let result = Matrix::from_iter(std::iter::repeat(0));
            let mut m = result.view_range(0..self.len(), 0..self.len());
            let len = m.len();
            for i in 0..len {
                for j in 0..len {
                    let v = rhs.data(i, j);
                    m.set_data(i, j, self.data(i, j) + v);
                }
            }

            BoxedMatrixWithView {
                matrix: result,
                rows: self.rows,
                columns: self.columns,
            }
        }
    }

    impl<'a, const N: usize> Sub for MatrixView<'a, N> {
        type Output = BoxedMatrixWithView<N>;

        fn sub(self, rhs: MatrixView<'a, N>) -> Self::Output {
            let result = Matrix::from_iter(std::iter::repeat(0));
            let mut m = result.view_range(0..self.len(), 0..self.len());
            let len = m.len();
            for i in 0..len {
                for j in 0..len {
                    m.set_data(i, j, self.data(i, j) - rhs.data(i, j));
                }
            }

            BoxedMatrixWithView {
                matrix: result,
                rows: self.rows,
                columns: self.columns,
            }
        }
    }
}

use view::MatrixView;

#[derive(Clone)]
pub struct BoxedMatrixWithView<const N: usize> {
    matrix: Matrix<N>,
    rows: Range<usize>,
    columns: Range<usize>,
}

impl<const N: usize> std::fmt::Display for BoxedMatrixWithView<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.view())
    }
}

impl<const N: usize> BoxedMatrixWithView<N> {
    fn view<'a>(&'a self) -> MatrixView<'a, N> {
        self.matrix
            .view_range(self.rows.clone(), self.columns.clone())
    }
}

impl<const N: usize> Add for BoxedMatrixWithView<N> {
    type Output = BoxedMatrixWithView<N>;

    fn add(self, rhs: BoxedMatrixWithView<N>) -> Self::Output {
        self.view() + rhs.view()
    }
}

impl<const N: usize> Sub for BoxedMatrixWithView<N> {
    type Output = BoxedMatrixWithView<N>;

    fn sub(self, rhs: BoxedMatrixWithView<N>) -> Self::Output {
        self.view() - rhs.view()
    }
}

impl<const N: usize> Matrix<N> {
    pub fn new(input: [[i32; N]; N]) -> Self {
        Self {
            data: RefCell::new(input),
        }
    }

    pub fn from_iter<T: Iterator<Item = i32>>(iter: T) -> Self {
        let size = N;
        let mut data: [[i32; N]; N] = [[0; N]; N];
        let mut i = 0usize;
        let mut j = 0usize;
        for v in iter {
            data[i][j] = v;
            j += 1;
            if j + 1 >= size && i + 1 >= size {
                break;
            }
            if j >= size {
                i += 1;
                j = 0;
            }
        }

        Matrix::new(data)
    }

    pub fn view(&self) -> MatrixView<'_, N> {
        MatrixView::from_matrix(&self, 0..self.len(), 0..self.len())
    }

    pub fn view_range(&self, rows: Range<usize>, columns: Range<usize>) -> MatrixView<'_, N> {
        MatrixView::from_matrix(&self, rows, columns)
    }

    pub fn mul(self, rhs: Matrix<N>) -> Matrix<N> {
        let mut m = Matrix::new([[0; N]; N]);
        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    m.data.get_mut()[i][j] += self.data.borrow()[i][k] * rhs.data.borrow()[k][j];
                }
            }
        }

        m
    }

    #[cfg(test)]
    pub fn inner_cloned(&self) -> Vec<Vec<i32>> {
        self.data
            .borrow()
            .iter()
            .map(|e| e.iter().map(|e| *e).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    fn dnc_mul_impl<'a, 'b>(m: MatrixView<'a, N>, rhs: MatrixView<'a, N>) -> BoxedMatrixWithView<N>
    where
        'b: 'a,
    {
        let c = Matrix::from_iter(std::iter::repeat(0));
        let mut c_view = c.view_range(0..m.len(), 0..m.len());

        if c_view.len() == 1 {
            c_view.set_data(0, 0, m.data(0, 0) * rhs.data(0, 0));
        } else {
            let (a11, a12, a21, a22) = m.split_evenly();
            let (b11, b12, b21, b22) = rhs.split_evenly();
            let (mut c11, mut c12, mut c21, mut c22) = c_view.split_evenly();

            let func = |a1, b1, a2, b2| {
                let c1 = Matrix::dnc_mul_impl(a1, b1);
                let c2 = Matrix::dnc_mul_impl(a2, b2);
                c1.add(c2)
            };

            c11.set_self_matrix(&func(a11.clone(), b11.clone(), a12.clone(), b21.clone()).view());
            c12.set_self_matrix(&func(a11, b12.clone(), a12, b22.clone()).view());
            c21.set_self_matrix(&func(a21.clone(), b11, a22.clone(), b21).view());
            c22.set_self_matrix(&func(a21, b12, a22, b22).view());
        }

        BoxedMatrixWithView {
            matrix: c,
            rows: 0..m.len(),
            columns: 0..m.len(),
        }
    }

    pub fn dnc_mul(&self, input: &Matrix<N>) -> Matrix<N> {
        let view1 = self.view();
        let view2 = input.view();
        let c = Matrix::dnc_mul_impl(view1, view2);
        c.matrix
    }

    fn strassen_mul_impl<'a>(
        m: MatrixView<'a, N>,
        rhs: MatrixView<'a, N>,
    ) -> BoxedMatrixWithView<N> {
        let c = Matrix::from_iter(std::iter::repeat(0));
        let mut c_view = c.view_range(0..m.len(), 0..m.len());

        println!("c before {}", c_view);
        println!("m before {}", m);
        println!("rhs before {}", rhs);

        if c_view.len() == 1 {
            c_view.set_data(0, 0, m.data(0, 0) * rhs.data(0, 0));
            println!("c after one {}", c_view);
            return BoxedMatrixWithView {
                matrix: c,
                rows: 0..m.len(),
                columns: 0..m.len(),
            };
        }

        let (a11, a12, a21, a22) = m.split_evenly();
        let (b11, b12, b21, b22) = rhs.split_evenly();
        let (mut c11, mut c12, mut c21, mut c22) = c_view.split_evenly();

        let s1 = b12.clone() - b22.clone(); // 2-4=-2
        let s2 = a11.clone() + a12.clone(); // 1+2=3
        let s3 = a21.clone() + a22.clone(); // 3+4=7
        let s4 = b21.clone() - b11.clone(); // 3-1=2
        let s5 = a11.clone() + a22.clone(); // 1+4=5
        let s6 = b11.clone() + b22.clone(); // 1+4=5
        let s7 = a12 - a22.clone(); //2-4=-2
        let s8 = b21 + b22.clone(); //3+4=7
        let s9 = a11.clone() - a21; //1-3=-2
        let s10 = b11.clone() + b12; //1+2=3

        let p1 = Matrix::strassen_mul_impl(a11, s1.view()); // 1*-2=-2
        let p2 = Matrix::strassen_mul_impl(s2.view(), b22); // 3*4=12
        let p3 = Matrix::strassen_mul_impl(s3.view(), b11); //7*1=1
        let p4 = Matrix::strassen_mul_impl(a22, s4.view()); //4*2=8
        let p5 = Matrix::strassen_mul_impl(s5.view(), s6.view()); // 5*5=25
        let p6 = Matrix::strassen_mul_impl(s7.view(), s8.view()); // -2*7=-14
        let p7 = Matrix::strassen_mul_impl(s9.view(), s10.view()); // -2*3=-6

        c11.set_self_matrix(&(p5.clone() + p4.clone() - p2.clone() + p6).view()); // 25+8-12+(-14)=7
        c12.set_self_matrix(&(p1.clone() + p2).view()); // -2+12=10
        c21.set_self_matrix(&(p3.clone() + p4).view()); // 1+8=9
        c22.set_self_matrix(&(p5 + p1 - p3 - p7).view()); // 25+(-2)-1-(-6)=28

        println!("c after {}", c_view);

        BoxedMatrixWithView {
            matrix: c,
            rows: 0..m.len(),
            columns: 0..m.len(),
        }
    }

    pub fn strassen_mul(&self, rhs: Matrix<N>) -> Matrix<N> {
        let view1 = self.view();
        let view2 = rhs.view();
        let c = Matrix::strassen_mul_impl(view1, view2);
        c.matrix
    }

    pub fn len(&self) -> usize {
        self.data.borrow().len()
    }
}

// impl<'a, const N: usize> std::ops::Add<MatrixView<'a, N>> for Matrix<N> {
//     type Output = Matrix<N>;

//     fn add(self, rhs: MatrixView<'a, N>) -> Self::Output {
//         self.view
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_range_len() {
        assert_eq!((0..4).len(), 4);
    }

    #[test]
    fn check_matrix_ops_simple() {
        let a = Matrix::new([[1, 2], [3, 4]]);
        let b = Matrix::new([[1, 2], [3, 4]]);

        let res = a.clone().mul(b.clone());
        assert_eq!(res.data.borrow().as_ref(), &[[7, 10], [15, 22]]);

        let res = a.dnc_mul(&b);
        assert_eq!(res.data.borrow().as_ref(), &[[7, 10], [15, 22]]);

        let res = a.strassen_mul(b);
        assert_eq!(res.data.borrow().as_ref(), &[[7, 10], [15, 22]]);
    }

    #[test]
    fn check_matrix_ops() {
        let a = Matrix::new([[1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4]]);
        let b = a.clone();

        let res = a.clone().mul(b.clone());
        assert_eq!(
            res.data.borrow().as_ref(),
            &[
                [10, 20, 30, 40],
                [10, 20, 30, 40],
                [10, 20, 30, 40],
                [10, 20, 30, 40]
            ]
        );

        let res = a.dnc_mul(&b);
        assert_eq!(
            res.data.borrow().as_ref(),
            &[
                [10, 20, 30, 40],
                [10, 20, 30, 40],
                [10, 20, 30, 40],
                [10, 20, 30, 40]
            ]
        );

        let res = a.strassen_mul(b);
        assert_eq!(
            res.data.borrow().as_ref(),
            &[
                [10, 20, 30, 40],
                [10, 20, 30, 40],
                [10, 20, 30, 40],
                [10, 20, 30, 40]
            ]
        );
    }

    #[test]
    fn check_view_ops() {
        let a = Matrix::new([[1, 2], [3, 4]]);
        let mut view1 = MatrixView::from_matrix(&a, 0..1, 0..1);
        let view2 = MatrixView::from_matrix(&a, 1..2, 1..2);

        assert_eq!(view1.inner_cloned(), [[1]]);
        assert_eq!(view2.inner_cloned(), [[4]]);

        let view = view1.clone().add(view2.clone());
        assert_eq!(view.view().inner_cloned(), [[5, 0], [0, 0]]);
        assert_eq!(a.inner_cloned(), [[1, 2], [3, 4]]);

        let view = view1.clone().sub(view2.clone());
        assert_eq!(view.view().inner_cloned(), [[-3, 0], [0, 0]]);

        view1.set_self_matrix(&view2);
        assert_eq!(view1.inner_cloned(), [[4]]);

        let mut view = MatrixView::from_matrix(&a, 0..2, 0..2);
        view.set_matrix(1..2, 0..1, &view1);
        assert_eq!(view.inner_cloned(), [[4, 2], [4, 4]]);

        let a = Matrix::new([[1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4]]);
        let view = a.view();

        let (v1, v2, v3, v4) = view.split_evenly();
        assert_eq!(v1.inner_cloned(), [[1, 2], [1, 2]]);
        assert_eq!(v2.inner_cloned(), [[3, 4], [3, 4]]);
        assert_eq!(v3.inner_cloned(), [[1, 2], [1, 2]]);
        assert_eq!(v4.inner_cloned(), [[3, 4], [3, 4]]);

        let (v11, v12, v13, v14) = v1.split_evenly();
        assert_eq!(v11.inner_cloned(), [[1]]);
        assert_eq!(v12.inner_cloned(), [[2]]);
        assert_eq!(v13.inner_cloned(), [[1]]);
        assert_eq!(v14.inner_cloned(), [[2]]);

        let (v41, v42, v43, v44) = v4.split_evenly();
        assert_eq!(v41.inner_cloned(), [[3]]);
        assert_eq!(v42.inner_cloned(), [[4]]);
        assert_eq!(v43.inner_cloned(), [[3]]);
        assert_eq!(v44.inner_cloned(), [[4]]);
    }
}
