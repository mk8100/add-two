
use add_one;

pub type RunResult = Result<(), &'static str>;

pub fn run(n:f64) -> RunResult {

    let w = Some(n)
                .and_then(add_one::sqrt)
                .and_then(add_one::log);

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
