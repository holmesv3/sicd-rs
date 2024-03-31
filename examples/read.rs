use clap::Parser;
use ndarray::s;
use sicd_rs::{read_sicd, ToNative};
/// Example of reading and working with a SICD file
#[derive(Parser)]
struct Args {
    /// Input file
    input: std::path::PathBuf,
}
fn main() {
    let args = Args::parse();
    let sicd = read_sicd(&args.input).unwrap();
    let meta = sicd.meta.get_v1_meta().unwrap();
    println!("Collection Info: ");
    println!("{:?}", meta.collection_info);
    let arr = sicd.image_data[0].array.slice(s![0..2, 0..2]);
    arr.indexed_iter()
        .for_each(|((row, col), z)| println!("[{row}, {col}] = {} + {}i", z.re, z.im));
    
    // Convert to 'native' Complex32
    let useful = arr.to_native();
}
