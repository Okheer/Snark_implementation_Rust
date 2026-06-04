// use ark_ff:: Field;
use ark_bn254:: Fr;

pub fn matrix_mul(matrix: [[Fr;3];2], witness:[Fr;3]) -> [Fr;2]{ // (2* 3) * (3*1)
     let mut out = [0u64.into(),0u64.into()];
     for i in 0..2{
        for j in 0..3{
            out[i] += matrix[i][j] * witness[j] ;
        }
     }
     out

}

pub fn add_2(a: [Fr;2], b: [Fr;2]) -> [Fr;2]{
    [a[0]+ b[0], a[1]+b[1]]
}

pub fn hadamard_product(a: &[Fr;2], b: &[Fr;2]) -> [Fr;2]{
    [a[0]*b[0], a[1]*b[1]]
}