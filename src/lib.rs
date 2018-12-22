
use add_one;

pub type RunResult = Result<(), &'static str>;

const ERR_MSG:&'static str = "Got: 'None' result";

pub fn run(n:f64) -> RunResult {

    let a =5;
    let z = (1..a).collect::<Vec<i32>>() ;

    let w = Some(n)
                .and_then(add_one::sqrt)
                .and_then(add_one::log)
                .map(|x|{x as f32})         // conversion to <f32> so the add_one::square(v:f32) can use it
                .map(add_one::square)
                .map(|x|{x as f32})         // conversion to <f32> so the add_one::square(v:f32) can use it
                .map(add_one::double)
                .map(add_one::neg);

    println!("result: {:?} {:?}", w, z);            

    let v1:Vec<u32> = vec![2, 4, 8, 10];
    let v2:Vec<u32> = vec![3, 5, 9, 11];

    let r = v1.iter()
        .zip(v2)
        .map(|(a,b)| a*b)
        .fold(0, |acc, a| acc +a);
        //.collect::<Vec<u32>>();

    println!("r = {:?}",r);

    match w {
        Some(_) => Ok( () ),
        None    => Err( ERR_MSG )
    }                

}


pub struct Matrix {
    nrow: u32,
    ncol: u32,
    data: Vec<u32>,
}

use std::fmt::Display;
use std::fmt;

impl Display for Matrix{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cl = |(i,idx):(usize,&u32)| {if idx % self.ncol == 0 {i.to_string() + " |\n"}else{i.to_string()+", "}}; 
        let w:String = self.data.iter()
                        .enumerate()
                        .map( cl )
                        .collect::<Vec<String>>()
                        .concat();
        write!(f, "\n{}", w)
    }
}

impl Matrix {
        pub fn new(r:u32, c:u32, v: Vec<u32>)->Result<Matrix,&'static str>{
            
            if r*c != v.len() as u32 {
                return Err("Not enough data inside vector")
            }

            Ok(Matrix{
                nrow: r,
                ncol: c,
                data: v,
            })
        }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_run_correctly() {
        assert_eq!(Ok( () ), run(3.0));
    }

    #[test]
    fn should_run_with_error() {
        assert_eq!( Err( ERR_MSG ), run(-3.0))
    }
}
