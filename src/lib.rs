pub use array_utils::chunk_vec;
pub use array_utils::remove_duplicates;
pub use array_utils::find_pairs_with_sum;
pub use statistics::mean;
pub use statistics::median;
pub use statistics::mode;
pub mod array_utils{
    use std::{collections::HashMap, hash::Hash, ops::Sub};
///Splits a vector into chunks of specified size.
/// # Examples
/// ```
/// use array_functions::chunk_vec;
/// let num = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
/// let chunked_vec = chunk_vec(&num, 3);
/// assert_eq!(chunked_vec, Ok(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]));
/// ```
pub fn chunk_vec<T:Clone>(vec:&[T],chunk_size:usize)->Result<Vec<Vec<T>>,&'static str>{
   if(chunk_size==0){
         Err("Chunk size cannot be zero")
   }
   else{
        let mut chunked_vec = Vec::new();
        let mut result=Vec::new();
       for item in vec{
        chunked_vec.push(item.clone());
        if(chunked_vec.len()==chunk_size){
            result.push(chunked_vec.clone());
            chunked_vec.clear();
        }
       }
       if !chunked_vec.is_empty(){
           result.push(chunked_vec.clone());
           }
        Ok(result)
   }
   
}
///Removes Duplicates from an array.
/// # Examples
/// ```
/// use array_functions::remove_duplicates;
/// let num = vec![1, 2, 3, 4, 5, 6, 2, 8, 9,9];
/// let arr = remove_duplicates(&num);
/// assert_eq!(chunked_vec, vec![1, 2, 3, 4, 5, 6, 8, 9]);
/// ```
pub fn remove_duplicates<T:Clone+PartialEq>(vec:&[T])->Vec<T>{
    let mut result=Vec::new();
    for item in vec{
        if !result.contains(item){
            result.push(item.clone());
        }
    }
    result
}
/// Finds all pairs that sum to a target value
/// 
/// # Examples
/// ```
/// use array_functions::find_pairs_with_sum;
/// let arr = vec![1, 5, 7, 1, 5, 3, 2];
/// let pairs = find_pairs_with_sum(&arr, 6);
/// assert_eq!(pairs, vec![(1, 5), (1, 5), (3, 3)]);
/// ```
pub fn find_pairs_with_sum<T: Copy + Hash+ Eq + Sub<Output = T>>(
    arr: &[T], 
    target: T
) -> Vec<(T, T)> {
    
    let mut count_map: HashMap<T, usize> = HashMap::new();
    let mut pairs = Vec::new();
    
    // Count occurrences
    for &num in arr {
        *count_map.entry(num).or_insert(0) += 1;
    }
    
    // Find pairs
    for &num in arr {
        if let Some(&count) = count_map.get(&(target - num)) {
            if num == target - num {
                // Handle case where both numbers are the same
                let pair_count = count / 2;
                for _ in 0..pair_count {
                    pairs.push((num, num));
                }
            } else {
                for _ in 0..count {
                    pairs.push((num, target - num));
                }
            }
        }
    }
    
    pairs
}

}
pub mod statistics{
    ///Calculates the mean of a vector of numbers.
    /// # Examples
    /// ```
    /// use array_functions::statistics::mean;
    /// let num = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let avg = mean(&num);
    /// assert_eq!(avg, 5.0);
    /// ```
    pub fn mean(vec:&[f64])->f64{
        let sum:f64=vec.iter().sum();
        sum/(vec.len() as f64)
    }
    ///Calculates the median of a vector of numbers.
    /// # Examples
    /// ```
    /// use array_functions::statistics::median;
    /// let num = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let med = median(&num);
    /// assert_eq!(med, 5.0);
    /// ```
    pub fn median(vec:&[f64])->f64{
        let mut vec=vec.to_vec();
        vec.sort_by(|a,b|a.partial_cmp(b).unwrap());
        let mid=vec.len()/2;
        if vec.len()%2==0{
            (vec[mid-1]+vec[mid])/2.0
        }
        else{
            vec[mid]
        }
    }
    ///Calculates the mode of a vector of numbers.
    /// # Examples
    /// ```
    /// use array_functions::statistics::mode;
    /// let num = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8];
    /// let mod = mode(&num);
    /// assert_eq!(mod, 1);
    /// ```
    pub fn mode(vec:&[i32])->i32{
        let mut count_map=std::collections::HashMap::new();
        for &num in vec{
            *count_map.entry(num).or_insert(0)+=1;
        }
        let max_val=count_map.values().max().unwrap();
        *count_map.iter().find(|&(_,v)|v==max_val).unwrap().0
    }
}