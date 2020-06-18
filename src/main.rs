mod bf;
use fasthash::murmur3;
use extprim::u128::u128;

fn main() {
    println!("{}", u128::from_built_in(murmur3::hash128(b"Hello, World!")));
    println!("{}", u128::from_built_in(murmur3::hash128(b"Hello, World")));

    println!("Hello, world!");

    let mut bf = bf::BloomFilter::new(1024, 3);

    bf.add(b"Hello, World!");
    bf.add(b"Hello, Again!");
    println!("{}", bf.test(b"Hello, World!"));
    println!("{}", bf.test(b"Hello, Again!"));
    println!("{}", bf.test(b"Good bye!"));
    println!("{}", bf.test(b"Hi there!"));

}
