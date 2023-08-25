use std::str::FromStr;

use proc_macro2::TokenStream as TkStream;
use syn::{ExprLit,Expr};

use quote::quote;

fn valide_prime(elements:&[usize], qnum:usize) -> bool
{
    for &el in elements
    {
        if qnum > el {
            let rem = qnum % el;
            if rem == 0 {return false;}
        }
    }
    true
}

fn generate_primes(tope:usize) -> Vec<usize>
{
    let mut tabla_primos:Vec<usize> = vec![2usize];
    for num in 3..tope
    {
        let valid = valide_prime(&tabla_primos[..],num);
        if valid == true {
            tabla_primos.push(num);
        }
    }
    tabla_primos
}

fn generate_primes_syntax(primes_values: &[usize]) -> TkStream
{
    let numbers_lit = primes_values.iter().map(|&num| -> TkStream {
        TkStream::from_str(num.to_string().as_str()).unwrap()
    });

    let size_lit = TkStream::from_str(primes_values.len().to_string().as_str()).unwrap();

    
    quote!(
        const TABLE_PRIMES:[usize; #size_lit] = [#(#numbers_lit),*];
    )
}

fn primes_constant_expression(inum:syn::LitInt) -> TkStream
{
    let value = inum.base10_parse::<usize>();
    match value {
        Ok(usnum) => {
            let vecprimes =  generate_primes(usnum);
            generate_primes_syntax(&vecprimes[..])                       
        },
        Err(er) => {
            syn::Error::new(inum.span(), "Not a Number:".to_string() + er.to_string().as_str() ).into_compile_error()
        }
    }
}


pub fn primes_table_gen(input: TkStream) -> TkStream
{
    let litnum_res = syn::parse2::<ExprLit>(input);
    let tokens = match litnum_res {
        syn::Result::Ok(litnum) => {
            match litnum.lit {
                syn::Lit::Int(inum) => {
                    primes_constant_expression(inum)
                },
                _ => {
                    syn::Error::new(litnum.lit.span(), "Not a Number".to_string() ).into_compile_error()
                }

            }
        },
        syn::Result::Err(err) => {
            syn::Error::new(err.span(), err.to_string() ).into_compile_error()
        }
    };

    tokens
}