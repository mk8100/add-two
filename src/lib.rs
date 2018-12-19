
use add_one;

pub type RunResult = Result<(), &'static str>;

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

    match w {
        Some(_) => Ok( () ),
        None    => Err( "Got: 'None' result" )
    }                

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
