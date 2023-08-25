

fn main() {

    primes_table::primes_table!(20);
    
    println!("Todos Los Primos!");

    TABLE_PRIMES.iter().enumerate().for_each(|tup|
        {
            println!("Indice:{} => {}", tup.0, tup.1);    
        }
    );
}
