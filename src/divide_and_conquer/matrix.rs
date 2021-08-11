use std::cell::RefCell;

type InnerData<const N: usize> = [[i32; N]; N];

#[derive(Debug, Clone)]
pub struct Matrix<const N: usize> {
    data: RefCell<InnerData<N>>,
}

mod view {
    // use super::MatrixData;
    use super::{InnerData, Matrix};
    use std::{
        cell::RefCell,
        ops::{Add, Range, Sub},
    };

    #[derive(Clone, Debug)]
    pub struct MatrixView<'a, const N: usize> {
        // probably need smth like &[&[i32]], but i struggle to construct it
        data: &'a RefCell<InnerData<N>>,
        rows: Range<usize>,
        columns: Range<usize>,
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
            self.data.borrow_mut()[self.rows.start + i][self.columns.start + j] = dbg!(v);
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
            let c11 = MatrixView::from_view(&self, 0..n / 2, 0..n / 2);
            let c12 = MatrixView::from_view(&self, 0..n / 2, n / 2..n);
            let c21 = MatrixView::from_view(&self, n / 2..n, 0..n / 2);
            let c22 = MatrixView::from_view(&self, n / 2..n, n / 2..n);
            (c11, c12, c21, c22)
        }
    }

    impl<'a, const N: usize> Add for MatrixView<'a, N> {
        type Output = MatrixView<'a, N>;

        fn add(self, rhs: MatrixView<'a, N>) -> Self::Output {
            let mut m = self;
            let len = m.len();
            for i in 0..len {
                for j in 0..len {
                    let v = rhs.data(i, j);
                    m.set_data(i, j, dbg!(m.data(i, j)) + dbg!(v));
                }
            }

            m
        }
    }

    impl<'a, const N: usize> Sub for MatrixView<'a, N> {
        type Output = MatrixView<'a, N>;

        fn sub(self, rhs: MatrixView<'a, N>) -> Self::Output {
            let mut m = self;
            let len = m.len();
            for i in 0..len {
                for j in 0..len {
                    m.set_data(i, j, m.data(i, j) - rhs.data(i, j));
                }
            }

            m
        }
    }
}

use view::MatrixView;

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

    pub fn inner_cloned(&self) -> Vec<Vec<i32>> {
        self.data
            .borrow()
            .iter()
            .map(|e| e.iter().map(|e| *e).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    fn dnc_mul_impl<'a, 'b>(c: &mut MatrixView<'a, N>, m: MatrixView<'a, N>, rhs: MatrixView<'a, N>)
    where
        'a: 'b,
    {
        if c.len() == 1 {
            c.set_data(0, 0, m.data(0, 0) * rhs.data(0, 0));
        } else {
            let (a11, a12, a21, a22) = m.split_evenly();
            let (b11, b12, b21, b22) = rhs.split_evenly();
            let (mut c11, mut c12, mut c21, mut c22) = c.split_evenly();

            let tmp = Matrix::from_iter(std::iter::repeat(0));
            let tmp_view = MatrixView::from_matrix(&tmp, 0..tmp.len(), 0..tmp.len());
            let (mut view1, mut view2, _, _) = tmp_view.split_evenly();

            let _func = move |mut v1: &'b mut MatrixView<'a, N>,
                              mut v2: &'b mut MatrixView<'a, N>,
                              cc: &'a mut MatrixView<'a, N>,
                              a1,
                              b1,
                              a2,
                              b2| {
                Matrix::dnc_mul_impl(&mut v1, a1, b1);
                Matrix::dnc_mul_impl(&mut v2, a2, b2);
                cc.set_self_matrix(&(v1.clone() + v2.clone()));
            };

            Matrix::dnc_mul_impl(&mut view1, a11.clone(), b11.clone());
            Matrix::dnc_mul_impl(&mut view2, a12.clone(), b21.clone());
            c11.set_self_matrix(&(view1.clone() + view2.clone()));

            Matrix::dnc_mul_impl(&mut view1, a11, b12.clone());
            Matrix::dnc_mul_impl(&mut view2, a12, b22.clone());
            c12.set_self_matrix(&(view1.clone() + view2.clone()));

            Matrix::dnc_mul_impl(&mut view1, a21.clone(), b11);
            Matrix::dnc_mul_impl(&mut view2, a22.clone(), b21);
            c21.set_self_matrix(&(view1.clone() + view2.clone()));

            Matrix::dnc_mul_impl(&mut view1, a21, b12);
            Matrix::dnc_mul_impl(&mut view2, a22, b22);
            c22.set_self_matrix(&(view1 + view2));
        }
    }

    pub fn dnc_mul(&self, input: &Matrix<N>) -> Matrix<N> {
        let view1 = MatrixView::from_matrix(&self, 0..self.len(), 0..self.len());
        let view2 = MatrixView::from_matrix(input, 0..input.len(), 0..input.len());
        let c = Matrix::from_iter(std::iter::repeat(0));
        let mut c_view = MatrixView::from_matrix(&c, 0..c.len(), 0..c.len());
        Matrix::dnc_mul_impl(&mut c_view, view1, view2);
        c
    }

    pub fn len(&self) -> usize {
        self.data.borrow().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    }

    #[test]
    fn check_view_ops() {
        let a = Matrix::new([[1, 2], [3, 4]]);
        let mut view1 = MatrixView::from_matrix(&a, 0..1, 0..1);
        let view2 = MatrixView::from_matrix(&a, 1..2, 1..2);

        assert_eq!(view1.inner_cloned(), [[1]]);
        assert_eq!(view2.inner_cloned(), [[4]]);

        println!("{:?}", view1);
        let view = view1.clone() + view2.clone();
        println!("{:?}", view);
        assert_eq!(view.inner_cloned(), [[5]]);
        assert_eq!(a.inner_cloned(), [[5, 2], [3, 4]]);

        let view = view1.clone() - view2.clone();
        assert_eq!(view.inner_cloned(), [[1]]);

        view1.set_self_matrix(&view2);
        assert_eq!(view1.inner_cloned(), [[4]]);

        let mut view = MatrixView::from_matrix(&a, 0..2, 0..2);
        view.set_matrix(1..2, 0..1, &view1);
        assert_eq!(view.inner_cloned(), [[4, 2], [4, 4]]);
    }
}
