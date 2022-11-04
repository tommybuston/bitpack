use std::convert::TryInto;


/// Returns true iff the signed value `n` fits into `width` signed bits.
/// 
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {

    let n_unsigned = match n.try_into(){
        Ok(n) => n,
        Err(error) => panic!("{:?}" , error),
    };
    
    n_unsigned_shifted = (n_unsigned<<(64 - width))>>(64-width);

    if n_unsigned == n_unsigned_shifted { return true;}

    false
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
/// 
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
        
    n_shifted = (n<(64 - width))>>(64-width);

    if n == n_shifted { return true;}

    false
}

/// Retrieve a signed value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {
    //get extracted word
    let word_unsigned = match word.try_into(){
        Ok(n) => n,
        Err(error) => panic!("{:?}" , error),
    };

    let extracted_word : u64;
    
    extracted_word = (word_unsigned<<64-width-lsb)>>(64 - width);   
    //invariant, ask about the difference between the two
    
    extracted_word;
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
    
    let extracted_word: u64;

    extracted_word = (word<<64-with-lsb)>>(64 - width);   
    //invariant


    extracted_word;
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    if !fitsu(word, width) {panic!()}
    //check that the word will not hang off the end!!!

    let right: u64;
    let left: u64;

    right = (word<<(64-lsb))>>(64-lsb);   //invariant
    left = (word>>(width+lsb))<<(width+lsb);

    let newu : Option<u64> = Some(left|word<<lsb|right);
    
    newu
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    if !fitss(word, width) {panic!()}
    Some(0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
