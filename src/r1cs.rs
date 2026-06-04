// use ark_ff::Field;
use ark_bn254::Fr;
use std::sync::LazyLock;

// pub type Var = usize;

// pub struct LinearTerm<F: Field>{
//     pub var: Var,
//     pub coeff: F
// }

// pub struct LinearCombination<F: Field>{
//     (pub Vec<LinearTerm<F>>);
// }

// pub struct Constraint<F: Field> {
//     pub a:LinearCombination<F>,
//     pub b:LinearCombination<F>,
//     pub c:LinearCombination<F>
// }

/* x^2-6x+8
   v= x*x 
   0= v-6x+8
*/

macro_rules! fr_matrix {
   ($([$($x: expr),*]),*$(,)?) => {
      [$([$(Fr::from($x as i64)),*]),*]
   };
}


pub static LEFT_MATRIX: LazyLock<[[Fr;3];2]> = LazyLock::new(|| fr_matrix![
                                [0,0,1],
                                [8,1,-6]
]);  //[1,v,x]

pub static RIGHT_MATRIX: LazyLock<[[Fr;3];2]> = LazyLock::new(||fr_matrix![
                                  [0,0,1],
                                  [1,0,0]
]);

pub static RESULT_MATRIX: LazyLock<[[Fr;3];2]> = LazyLock::new(|| fr_matrix![
                                        [0,1,0],
                                        [0,0,0]
]);
