use array_stat_functions::chunk_vec;
use array_stat_functions::remove_duplicates;
use array_stat_functions::find_pairs_with_sum;
use array_stat_functions::mean;
use array_stat_functions::median;
use array_stat_functions::mode;
fn main() {
    let v=vec![1,2,3,4,5,6,7,8,9];
    let chunked_vec=chunk_vec(&v,3);
    match chunked_vec {
        Ok(result)=>println!("{:?}",result),
        Err(e)=>println!("{}",e),
    }
}
