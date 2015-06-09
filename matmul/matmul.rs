use std::env;
use std::ops::Index;
use std::str::FromStr;
use std::slice::{Chunks, ChunksMut};

struct Mat {
  rows: usize,
  columns: usize,
  data: Vec<f64>,
}

impl Mat {
  pub fn new(rows: usize, columns: usize) -> Mat {
     Mat {
       rows: rows,
       columns: columns,
       data: vec![0.0; rows * columns],
     }
  }

  pub fn rows(&self) -> usize { self.rows }
  pub fn columns(&self) -> usize { self.columns }

  pub fn iter(&self) -> Chunks<f64> { self.data.chunks(self.columns) }
  pub fn iter_mut(&mut self) -> ChunksMut<f64> { self.data.chunks_mut(self.columns) }
}

impl Index<usize> for Mat {
  type Output = [f64];
  fn index(&self, row: usize) -> &[f64] {
    let start = row * self.columns;
    &self.data[start..(start + self.columns)]
  }
}

fn matgen(n: usize) -> Mat {
  let mut a = Mat::new(n, n);
  let tmp = 1_f64 / (n as f64) / (n as f64);
  for (i, row) in a.iter_mut().enumerate() {
    for (j, element) in row.iter_mut().enumerate() {
      *element = tmp * (i as f64 - j as f64) * (i as f64 + j as f64);
    }
  }
  a
}

fn matmul(a: &Mat, b: &Mat) -> Mat {
  let m = a.rows();
  let n = a.columns();
  let p = b.columns();

  let mut b2 = Mat::new(n, p);
  for (i, row) in b2.iter_mut().enumerate() {
    for (j, element) in row.iter_mut().enumerate() {
      *element = b[j][i];
    }
  }

  let mut c = Mat::new(m, p);
  for (c_row, a_row) in c.iter_mut().zip(a.iter()) {
    for (element, b2_row) in c_row.iter_mut().zip(b.iter()) {
      *element = a_row.iter().zip(b2_row.iter())
          .fold(0.0f64, |sum, (&x, &y)| sum + x * y);
    }
  }

  c
}

fn main() {
  let mut n = 100;
  if env::args().len() > 1 {
    let arg1 = env::args().nth(1).unwrap();
    n = FromStr::from_str(&arg1).unwrap();
  }
  n = n / 2 * 2;

  let a = matgen(n);
  let b = matgen(n);
  let c = matmul(&a, &b);
  print!("{}\n", c[n / 2][n / 2]);
}
