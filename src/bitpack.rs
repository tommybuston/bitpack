use std::convert::TryInto;


/// Returns true iff the signed value `n` fits into `width` signed bits.
/// 
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {

    if width == 0 {return false;}

    let n_shifted : i64 = (n<<(64 - width))>>(64-width);

    if n == n_shifted { return true;}

    false
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
/// 
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    if width == 0 {return false;}
        
    let n_shifted = (n<<(64 - width))>>(64-width);

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
    //checked error
    if width > 64 || width + lsb > 64 {panic!();}
    //get extracted word
    let extracted_word = (word<<(64-width-lsb))>>(64 - width);

    let mut extracted_word_signed : i64 = extracted_word.try_into().unwrap();  

    extracted_word_signed = (extracted_word_signed<<(64-width))>>(64-width);
    //invariant, ask about the difference between the two
    
    extracted_word_signed
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
    //checked error
    if width > 64 || width + lsb > 64 {panic!();}
    
    let extracted_word: u64;

    extracted_word = (word<<64-width-lsb)>>(64 - width);   
    //invariant

    extracted_word
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
    //checked error
    if width > 64 || width + lsb > 64 {panic!();}

    if !fitsu(value, width) {return None}
    //check that the word will not hang off the end!!!

    let right: u64;
    let left: u64;
    if lsb == 0 { right = 0; } else { right = (word<<(64-lsb))>>(64-lsb); } //invariant
    if width+lsb == 64 { left = 0; } else { left = (word>>(width+lsb))<<(width+lsb);}

    let newu : Option<u64> = Some(left|value<<lsb|right);
    
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
    //checked error
    if width > 64 || width + lsb > 64 {panic!();}

    if !fitss(value, width) {return None}
    
    let right: u64;
    let left: u64;
    let mut middle: u64;

    if lsb == 0 { right = 0; } else { right = (word<<(64-lsb))>>(64-lsb); }
    if width+lsb == 64 { left = 0; } else { left = (word>>(width+lsb))<<(width+lsb);}

    if value < 0 {
        middle = (!(value<<(64-width))).try_into().unwrap();
        middle = (!middle)>>(64-width-lsb);

    }
    else {
        middle = (value<<lsb).try_into().unwrap();
    }

    let news : Option<u64> = Some(left|middle|right);

    news
}



#[cfg(test)]
mod tests {
    use crate::bitpack::{fitsu,fitss,getu,gets,newu,news};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn fitu_zero() {
        assert_eq!(fitsu(17, 0), false);
    }
    #[test]
    fn fitu_23() {
        assert_eq!(fitsu(23, 5), true);
        assert_eq!(fitsu(23, 4), false);
    }
    #[test]
    fn fitss_23() {
        assert_eq!(fitss(23, 6), true);
        assert_eq!(fitss(23, 5), false);
    }
    #[test]
    fn getu_23() {
        assert_eq!(getu(23, 3, 2), 5 as u64);
        assert_eq!(getu(23, 3, 3), 2 as u64);
    }
    #[test]
    fn gets_23() {
        assert_eq!(gets(23, 3, 2), -3 as i64);
        assert_eq!(gets(23, 3, 3), 2 as i64);

    }
    #[test]
    fn newu_23() {
        assert_eq!(newu(23, 3, 3, 7).unwrap(), 63 as u64);
        assert_eq!(newu(23, 3, 3, 6).unwrap(), 55 as u64);
    }
    #[test]
    fn news_23() {
        assert_eq!(news(23, 3, 3, 0).unwrap(), 7 as u64);
        assert_eq!(news(23, 3, 2, 3).unwrap(), 15 as u64);
    }
    #[test]
    fn news_neg3() {
        assert_eq!(news(23, 4, 2, -3).unwrap(), 55 as u64);
    }


}
