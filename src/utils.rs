// use ark_ff:: Field;
use ark_bn254:: Fr;

pub fn matrix_mul(matrix: [[Fr;3];2] ,witness:[Fr;3]) -> [Fr;2]{ // (2* 3) * (3*1)
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

pub fn scalar_mul(matrix: &[Fr;2], scalar:Fr) -> [Fr;2]{
    [scalar*matrix[0],scalar*matrix[1]]
}

pub fn interpolate(matrix: &[[Fr;3];2], column: usize) -> [Fr;2]{
    //We have 2 rows thus max 1 degree of poly can bas through it
    // a0x+a1=y
    let xs: [Fr;2]= [1u64.into(), 2u64.into()];
    let ys: [Fr;2]= [matrix[0][column],matrix[1][column]];
    
    /*ao+a1=ys[0]
      2ao+a1= ys[1]
      
     */
     [ys[1]-ys[0],Fr::from(2u64)*ys[0]-ys[1]]

}

pub fn interpolate_matrix(matrix: [[Fr;3];2]) -> [[Fr;2];3]{
    [interpolate(&matrix,0),interpolate(&matrix,1),interpolate(&matrix,2)]
}