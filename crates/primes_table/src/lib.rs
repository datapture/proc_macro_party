use primes_table_gen::primes_table_gen;
use proc_macro::TokenStream;

#[proc_macro]
pub fn primes_table(input: TokenStream /* Un entero constante*/ ) -> TokenStream {
    let tokens = primes_table_gen::primes_table_gen(input.into());
    tokens.into()
}

