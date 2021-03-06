
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

//////////////////////////
// tests
/////////////////////////
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
