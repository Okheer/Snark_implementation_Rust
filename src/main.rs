#![allow(unused)]

mod utils;
mod r1cs;
mod witness;

use r1cs::{LEFT_MATRIX, RIGHT_MATRIX,RESULT_MATRIX};
use ark_bn254::Fr;
use utils::{matrix_mul, hadamard_product};
use witness::WITNESS;

fn check_matrix_eq(a:[Fr;2], b:[Fr;2]) -> bool{
   let mut is_equal = true;
  
   for i in 0..2{
      if a[i]!=b[i]{
         is_equal= false;
      }
   }

   is_equal
}


fn main() {
   // let witness_vector: [Fr;3]= [Fr::from(1u64),Fr::from(16u64),Fr::from(4u64)];

   let l_w:[Fr;2] = matrix_mul(*LEFT_MATRIX, WITNESS());
   let r_w:[Fr;2] = matrix_mul(*RIGHT_MATRIX, WITNESS());
   let o_w:[Fr;2] = matrix_mul(*RESULT_MATRIX, WITNESS());

    
   if check_matrix_eq(hadamard_product(&l_w,&r_w),o_w){
      println!("Rejoice! Correct witness");
   }
   else{
      println!("witness wrong! :");
   }
}
