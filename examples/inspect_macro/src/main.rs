use std::str::FromStr;

use proc_macro2::TokenStream as TkStream;

const CODE_TEST1:&'static  str = "
    20
";

const CODE_TEST2: & 'static str = "
    YYYY
";

fn main() {
    let inspect1 = primes_table_gen::primes_table_gen(TkStream::from_str(CODE_TEST1).unwrap());
    println!("El código Generado es:\n {}", inspect1.to_string());
}
