/// `Level` represent the compression level this value is include between 1 to 9.
/// 1 optimize the compression time,
/// 9 optimize the size of the output.
///
/// For bzip2:
///  - `Zero`/`One` are convert to `bzip2::Compression::Fastest`,
///  - `Nine` in `bzip2::Compression::Best`
///     and other value is convert to `bzip2::Compression::Default.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Level {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
}

impl From<Level> for u32 {
    fn from(level: Level) -> Self {
        match level {
            Level::Zero => 0,
            Level::One => 1,
            Level::Two => 2,
            Level::Three => 3,
            Level::Four => 4,
            Level::Five => 5,
            Level::Six => 6,
            Level::Seven => 7,
            Level::Eight => 8,
            Level::Nine => 9,
            Level::Ten => 10,
            Level::Eleven => 11,
            Level::Twelve => 12,
            Level::Thirteen => 13,
            Level::Fourteen => 14,
            Level::Fifteen => 15,
            Level::Sixteen => 16,
            Level::Seventeen => 17,
            Level::Eighteen => 18,
            Level::Nineteen => 19,
            Level::Twenty => 20,
            Level::TwentyOne => 21,
        }
    }
}

impl From<Level> for i32 {
    fn from(level: Level) -> Self {
        match level {
            Level::Zero => 0,
            Level::One => 1,
            Level::Two => 2,
            Level::Three => 3,
            Level::Four => 4,
            Level::Five => 5,
            Level::Six => 6,
            Level::Seven => 7,
            Level::Eight => 8,
            Level::Nine => 9,
            Level::Ten => 10,
            Level::Eleven => 11,
            Level::Twelve => 12,
            Level::Thirteen => 13,
            Level::Fourteen => 14,
            Level::Fifteen => 15,
            Level::Sixteen => 16,
            Level::Seventeen => 17,
            Level::Eighteen => 18,
            Level::Nineteen => 19,
            Level::Twenty => 20,
            Level::TwentyOne => 21,
        }
    }
}

#[cfg(feature = "gz")]
impl From<Level> for flate2::Compression {
    fn from(level: Level) -> Self {
        match level {
            Level::Zero => flate2::Compression::new(0),
            Level::One => flate2::Compression::new(1),
            Level::Two => flate2::Compression::new(2),
            Level::Three => flate2::Compression::new(3),
            Level::Four => flate2::Compression::new(4),
            Level::Five => flate2::Compression::new(5),
            Level::Six => flate2::Compression::new(6),
            Level::Seven => flate2::Compression::new(7),
            Level::Eight => flate2::Compression::new(8),
            Level::Nine => flate2::Compression::new(9),
            _ => flate2::Compression::new(9),
        }
    }
}

#[cfg(feature = "bz2")]
impl From<Level> for bzip2::Compression {
    fn from(level: Level) -> Self {
        match level {
            Level::Zero => bzip2::Compression::new(1),
            Level::One => bzip2::Compression::new(1),
            Level::Two => bzip2::Compression::new(2),
            Level::Three => bzip2::Compression::new(3),
            Level::Four => bzip2::Compression::new(4),
            Level::Five => bzip2::Compression::new(5),
            Level::Six => bzip2::Compression::new(6),
            Level::Seven => bzip2::Compression::new(7),
            Level::Eight => bzip2::Compression::new(8),
            Level::Nine => bzip2::Compression::new(9),
            _ => bzip2::Compression::new(9),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level2u32() {
        let mut tmp: u32 = Level::One.into();
        assert_eq!(tmp, 1);

        tmp = Level::Two.into();
        assert_eq!(tmp, 2);

        tmp = Level::Three.into();
        assert_eq!(tmp, 3);

        tmp = Level::Four.into();
        assert_eq!(tmp, 4);

        tmp = Level::Five.into();
        assert_eq!(tmp, 5);

        tmp = Level::Six.into();
        assert_eq!(tmp, 6);

        tmp = Level::Seven.into();
        assert_eq!(tmp, 7);

        tmp = Level::Eight.into();
        assert_eq!(tmp, 8);

        tmp = Level::Nine.into();
        assert_eq!(tmp, 9);

        tmp = Level::Ten.into();
        assert_eq!(tmp, 10);

        tmp = Level::Eleven.into();
        assert_eq!(tmp, 11);

        tmp = Level::Twelve.into();
        assert_eq!(tmp, 12);

        tmp = Level::Thirteen.into();
        assert_eq!(tmp, 13);

        tmp = Level::Fourteen.into();
        assert_eq!(tmp, 14);

        tmp = Level::Fifteen.into();
        assert_eq!(tmp, 15);

        tmp = Level::Sixteen.into();
        assert_eq!(tmp, 16);

        tmp = Level::Seventeen.into();
        assert_eq!(tmp, 17);

        tmp = Level::Eighteen.into();
        assert_eq!(tmp, 18);

        tmp = Level::Nineteen.into();
        assert_eq!(tmp, 19);

        tmp = Level::Twenty.into();
        assert_eq!(tmp, 20);

        tmp = Level::TwentyOne.into();
        assert_eq!(tmp, 21);
    }

    #[test]
    fn level2i32() {
        let mut tmp: i32 = Level::One.into();
        assert_eq!(tmp, 1);

        tmp = Level::Two.into();
        assert_eq!(tmp, 2);

        tmp = Level::Three.into();
        assert_eq!(tmp, 3);

        tmp = Level::Four.into();
        assert_eq!(tmp, 4);

        tmp = Level::Five.into();
        assert_eq!(tmp, 5);

        tmp = Level::Six.into();
        assert_eq!(tmp, 6);

        tmp = Level::Seven.into();
        assert_eq!(tmp, 7);

        tmp = Level::Eight.into();
        assert_eq!(tmp, 8);

        tmp = Level::Nine.into();
        assert_eq!(tmp, 9);

        tmp = Level::Ten.into();
        assert_eq!(tmp, 10);

        tmp = Level::Eleven.into();
        assert_eq!(tmp, 11);

        tmp = Level::Twelve.into();
        assert_eq!(tmp, 12);

        tmp = Level::Thirteen.into();
        assert_eq!(tmp, 13);

        tmp = Level::Fourteen.into();
        assert_eq!(tmp, 14);

        tmp = Level::Fifteen.into();
        assert_eq!(tmp, 15);

        tmp = Level::Sixteen.into();
        assert_eq!(tmp, 16);

        tmp = Level::Seventeen.into();
        assert_eq!(tmp, 17);

        tmp = Level::Eighteen.into();
        assert_eq!(tmp, 18);

        tmp = Level::Nineteen.into();
        assert_eq!(tmp, 19);

        tmp = Level::Twenty.into();
        assert_eq!(tmp, 20);

        tmp = Level::TwentyOne.into();
        assert_eq!(tmp, 21);
    }

    #[cfg(feature = "gz")]
    #[test]
    fn level2flate2() {
        let mut tmp: flate2::Compression = Level::Zero.into();
        assert_eq!(tmp, flate2::Compression::new(0));

        tmp = Level::One.into();
        assert_eq!(tmp, flate2::Compression::new(1));

        tmp = Level::Two.into();
        assert_eq!(tmp, flate2::Compression::new(2));

        tmp = Level::Three.into();
        assert_eq!(tmp, flate2::Compression::new(3));

        tmp = Level::Four.into();
        assert_eq!(tmp, flate2::Compression::new(4));

        tmp = Level::Five.into();
        assert_eq!(tmp, flate2::Compression::new(5));

        tmp = Level::Six.into();
        assert_eq!(tmp, flate2::Compression::new(6));

        tmp = Level::Seven.into();
        assert_eq!(tmp, flate2::Compression::new(7));

        tmp = Level::Eight.into();
        assert_eq!(tmp, flate2::Compression::new(8));

        tmp = Level::Nine.into();
        assert_eq!(tmp, flate2::Compression::new(9));

        tmp = Level::Ten.into();
        assert_eq!(tmp, flate2::Compression::new(9));

        tmp = Level::Eleven.into();
        assert_eq!(tmp, flate2::Compression::new(9));

        tmp = Level::Twelve.into();
        assert_eq!(tmp, flate2::Compression::new(9));

        tmp = Level::Thirteen.into();
        assert_eq!(tmp, flate2::Compression::new(9));

        tmp = Level::Fourteen.into();
        assert_eq!(tmp, flate2::Compression::new(9));

        tmp = Level::Fifteen.into();
        assert_eq!(tmp, flate2::Compression::new(9));

        tmp = Level::Sixteen.into();
        assert_eq!(tmp, flate2::Compression::new(9));

        tmp = Level::Seventeen.into();
        assert_eq!(tmp, flate2::Compression::new(9));

        tmp = Level::Eighteen.into();
        assert_eq!(tmp, flate2::Compression::new(9));

        tmp = Level::Nineteen.into();
        assert_eq!(tmp, flate2::Compression::new(9));

        tmp = Level::Twenty.into();
        assert_eq!(tmp, flate2::Compression::new(9));

        tmp = Level::TwentyOne.into();
        assert_eq!(tmp, flate2::Compression::new(9));
    }

    #[test]
    #[cfg(feature = "bz2")]
    fn level2bzip2() {
        let tmp: bzip2::Compression = Level::Zero.into();
        assert_eq!(tmp.level(), bzip2::Compression::new(1).level());

        let tmp: bzip2::Compression = Level::One.into();
        assert_eq!(tmp.level(), bzip2::Compression::new(1).level());

        let tmp: bzip2::Compression = Level::Two.into();
        assert_eq!(tmp.level(), bzip2::Compression::new(2).level());

        let tmp: bzip2::Compression = Level::Three.into();
        assert_eq!(tmp.level(), bzip2::Compression::new(3).level());

        let tmp: bzip2::Compression = Level::Four.into();
        assert_eq!(tmp.level(), bzip2::Compression::new(4).level());

        let tmp: bzip2::Compression = Level::Five.into();
        assert_eq!(tmp.level(), bzip2::Compression::new(5).level());

        let tmp: bzip2::Compression = Level::Six.into();
        assert_eq!(tmp.level(), bzip2::Compression::new(6).level());

        let tmp: bzip2::Compression = Level::Seven.into();
        assert_eq!(tmp.level(), bzip2::Compression::new(7).level());

        let tmp: bzip2::Compression = Level::Eight.into();
        assert_eq!(tmp.level(), bzip2::Compression::new(8).level());

        let tmp: bzip2::Compression = Level::Nine.into();
        assert_eq!(tmp.level(), bzip2::Compression::new(9).level());

        let tmp: bzip2::Compression = Level::Ten.into();
        assert_eq!(tmp.level(), bzip2::Compression::new(9).level());

        let tmp: bzip2::Compression = Level::Eleven.into();
        assert_eq!(tmp.level(), bzip2::Compression::new(9).level());

        let tmp: bzip2::Compression = Level::Twelve.into();
        assert_eq!(tmp.level(), bzip2::Compression::new(9).level());

        let tmp: bzip2::Compression = Level::Thirteen.into();
        assert_eq!(tmp.level(), bzip2::Compression::new(9).level());

        let tmp: bzip2::Compression = Level::Fourteen.into();
        assert_eq!(tmp.level(), bzip2::Compression::new(9).level());

        let tmp: bzip2::Compression = Level::Fifteen.into();
        assert_eq!(tmp.level(), bzip2::Compression::new(9).level());

        let tmp: bzip2::Compression = Level::Sixteen.into();
        assert_eq!(tmp.level(), bzip2::Compression::new(9).level());

        let tmp: bzip2::Compression = Level::Seventeen.into();
        assert_eq!(tmp.level(), bzip2::Compression::new(9).level());

        let tmp: bzip2::Compression = Level::Eighteen.into();
        assert_eq!(tmp.level(), bzip2::Compression::new(9).level());

        let tmp: bzip2::Compression = Level::Nineteen.into();
        assert_eq!(tmp.level(), bzip2::Compression::new(9).level());

        let tmp: bzip2::Compression = Level::Twenty.into();
        assert_eq!(tmp.level(), bzip2::Compression::new(9).level());

        let tmp: bzip2::Compression = Level::TwentyOne.into();
        assert_eq!(tmp.level(), bzip2::Compression::new(9).level());
    }
}
