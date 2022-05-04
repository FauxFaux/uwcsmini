use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Word(u64);

impl fmt::Debug for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut w = self.0;
        while w != 0 {
            let masked = (w & 31) as u8;
            write!(f, "{}", (masked + b'a' - 1) as char)?;
            w >>= 5;
        }
        Ok(())
    }
}

impl Word {
    fn new(from: &str) -> Self {
        let mut w: u64 = 0;
        for (idx, c) in from.chars().enumerate() {
            w |= (((c as u8) - b'a' + 1) as u64) << (idx * 5);
        }
        Word(w)
    }

    fn dupl_first(&self) -> Option<Self> {
        let mut w = self.0;
        if w == 0 {
            return None;
        }
        if w & 0b00_11111_00000_00000_00000_00000_00000 != 0 {
            return None;
        }
        let s = w & 31;
        w <<= 5;
        w |= s;
        Some(Word(w))
    }

    fn pop(&self) -> Option<Self> {
        let mut w = self.0;
        w >>= 5;
        if w == 0 {
            return None;
        }
        Some(Word(w))
    }

    fn shifts(&self) -> [Option<Self>; 12] {
        let mut ret: [Option<Self>; 12] = Default::default();
        for i in 0..6 {
            let shift = i * 5;
            let mask = 31 << shift;
            let c = ((self.0 & mask) >> shift) as u8;
            if c == 0 {
                break;
            }
            let w = self.0 & !mask;
            let mut up = c + 1;
            let mut down = c - 1;
            if up == 27 {
                up = 1;
            }

            if down == 0 {
                down = 26;
            }

            ret[i] = Some(Word(w | u64::from(up) << shift));
            ret[i + 6] = Some(Word(w | u64::from(down) << shift));
        }
        ret
    }
}

fn main() {
    println!("{:?}", Word::new("hello"));
}

#[test]
fn strs() {
    assert_eq!("a", format!("{:?}", Word::new("a")));
    assert_eq!("ab", format!("{:?}", Word::new("ab")));
    assert_eq!("abcde", format!("{:?}", Word::new("abcde")));
    assert_eq!("abcdefghi", format!("{:?}", Word::new("abcdefghi")));
}

#[test]
fn dupl() {
    assert_eq!(Some(Word::new("aa")), Word::new("a").dupl_first());
    assert_eq!(Some(Word::new("aab")), Word::new("ab").dupl_first());
    assert_eq!(Some(Word::new("aabcde")), Word::new("abcde").dupl_first());
    assert_eq!(None, Word::new("abcdef").dupl_first());
}

#[test]
fn poppity() {
    assert_eq!(Some(Word::new("bcde")), Word::new("abcde").pop());
    assert_eq!(Some(Word::new("b")), Word::new("ab").pop());
    assert_eq!(None, Word::new("a").pop());
    assert_eq!(None, Word::new("").pop());
}

#[test]
fn shifty_edge() {
    assert_eq!(
        [
            Some(Word::new("b")),
            None,
            None,
            None,
            None,
            None,
            Some(Word::new("z")),
            None,
            None,
            None,
            None,
            None
        ],
        Word::new("a").shifts()
    );
    assert_eq!(
        [
            Some(Word::new("a")),
            None,
            None,
            None,
            None,
            None,
            Some(Word::new("y")),
            None,
            None,
            None,
            None,
            None
        ],
        Word::new("z").shifts()
    );
}

#[test]
fn shifty_multiple() {
    assert_eq!(
        [
            Some(Word::new("cc")),
            Some(Word::new("bd")),
            None,
            None,
            None,
            None,
            Some(Word::new("ac")),
            Some(Word::new("bb")),
            None,
            None,
            None,
            None
        ],
        Word::new("bc").shifts()
    );
}

#[test]
fn shifty_long() {
    assert_eq!(
        [
            Some(Word::new("pooooo")),
            Some(Word::new("opoooo")),
            Some(Word::new("oopooo")),
            Some(Word::new("ooopoo")),
            Some(Word::new("oooopo")),
            Some(Word::new("ooooop")),
            Some(Word::new("nooooo")),
            Some(Word::new("onoooo")),
            Some(Word::new("oonooo")),
            Some(Word::new("ooonoo")),
            Some(Word::new("oooono")),
            Some(Word::new("ooooon")),
        ],
        Word::new("oooooo").shifts()
    );
}
