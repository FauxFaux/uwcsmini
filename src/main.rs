use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt;
use std::num::NonZeroU64;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Word(NonZeroU64);

impl fmt::Debug for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut w = self.0.get();
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
        assert!(!from.is_empty());
        assert!(from.is_ascii());
        let mut w: u64 = 0;
        for (idx, c) in from.chars().enumerate() {
            w |= (((c as u8) - b'a' + 1) as u64) << (idx * 5);
        }
        Word::u64(w)
    }

    fn u64(val: u64) -> Self {
        Word(NonZeroU64::new(val).unwrap())
    }

    fn dupl_first(&self) -> Option<Self> {
        let mut w = self.0.get();
        if w & 0b00_11111_00000_00000_00000_00000_00000 != 0 {
            return None;
        }
        let s = w & 31;
        w <<= 5;
        w |= s;
        Some(Word::u64(w))
    }

    fn pop(&self) -> Option<Self> {
        let mut w = self.0.get();
        w >>= 5;
        if w == 0 {
            return None;
        }
        Some(Word::u64(w))
    }

    fn rotate(&self) -> [Option<Self>; 2] {
        let w = self.0.get();
        let mask = 31;
        let is_len = |v: u8| w & (mask << v * 5) == 0;
        if is_len(1) {
            return [None, None];
        }

        // TODO: special case 2?

        for len in [2, 3, 4, 5, 6] {
            if !is_len(len) {
                continue;
            }

            let last = (len - 1) * 5;

            let start = w & mask;
            let end = (w & (mask << last)) >> last;

            let right = w >> 5;
            let left = (w << 5) & !(mask << len * 5);

            return [
                Some(Word::u64(right | (start << last))),
                Some(Word::u64(left | end)),
            ];
        }

        unreachable!()
    }

    fn shifts(&self) -> [Option<Self>; 12] {
        let us = self.0.get();
        let mut ret: [Option<Self>; 12] = Default::default();
        for i in 0..6 {
            let shift = i * 5;
            let mask = 31 << shift;
            let c = ((us & mask) >> shift) as u8;
            if c == 0 {
                break;
            }
            let w = us & !mask;
            let mut up = c + 1;
            let mut down = c - 1;
            if up == 27 {
                up = 1;
            }

            if down == 0 {
                down = 26;
            }

            ret[i] = Some(Word::u64(w | u64::from(up) << shift));
            ret[i + 6] = Some(Word::u64(w | u64::from(down) << shift));
        }
        ret
    }
}

fn main() {
    let mut m = HashMap::with_capacity(100_000);
    let starter = Word::new("sick");
    m.insert(starter, 0);

    let mut new_words: Vec<Word> = Vec::with_capacity(100);
    new_words.push(starter);
    for it in 1..32u8 {
        let old_words = new_words.clone();
        new_words.clear();
        for k in old_words {
            let mut appl = |op: Option<Word>| {
                if let Some(word) = op {
                    if let Entry::Vacant(v) = m.entry(word) {
                        v.insert(it);
                        new_words.push(word);
                    }
                }
            };
            appl(k.dupl_first());
            appl(k.pop());
            for op in k.shifts() {
                appl(op);
            }
            for op in k.rotate() {
                appl(op);
            }
        }

        // println!("{:?} {:?}", new_words, m);

        println!(
            "{}: {} {} true: {:?}",
            it,
            new_words.len(),
            m.len(),
            m.get(&Word::new("true"))
        );
    }
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

#[test]
fn rotter() {
    assert_eq!([None, None], Word::new("a").rotate());
    assert_eq!(
        [Some(Word::new("aa")), Some(Word::new("aa"))],
        Word::new("aa").rotate()
    );
    assert_eq!(
        [Some(Word::new("ba")), Some(Word::new("ba"))],
        Word::new("ab").rotate()
    );
    assert_eq!(
        [Some(Word::new("bca")), Some(Word::new("cab"))],
        Word::new("abc").rotate()
    );
}
