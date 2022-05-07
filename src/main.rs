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

    fn len(&self) -> u8 {
        let w = self.0.get();
        let first_bit_set = u64::BITS - w.leading_zeros() + (5 - 1);
        (first_bit_set / 5) as u8
    }

    fn dupl_first(&self, len_limit: u8) -> Option<Self> {
        if self.len() >= len_limit {
            return None;
        }
        let mut w = self.0.get();
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
        let mask = 31;

        let len = self.len();

        if 1 == len {
            return [None, None];
        }

        let w = self.0.get();

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
    let mut m = HashMap::with_capacity(10_000_000);
    let starter = Word::new("sick");
    let target = Word::new("true");
    m.insert(starter, (0, starter));
    let len_limit = 6;

    let mut new_words: Vec<Word> = Vec::with_capacity(100);
    new_words.push(starter);
    for it in 1..32u8 {
        let old_words = new_words.clone();
        new_words.clear();
        for k in old_words {
            let mut appl = |op: Option<Word>| {
                if let Some(word) = op {
                    if let Entry::Vacant(v) = m.entry(word) {
                        v.insert((it, k));
                        new_words.push(word);
                    }
                }
            };
            appl(k.dupl_first(len_limit));
            appl(k.pop());
            for op in k.shifts() {
                appl(op);
            }
            for op in k.rotate() {
                appl(op);
            }
        }

        // println!("{:?} {:?}", new_words, m);

        if m.contains_key(&target) {
            break;
        }

        println!("{}: {} {}", it, new_words.len(), m.len(),);
    }

    let mut curr = target;
    while let Some((it, word)) = m.get(&curr) {
        println!("{}: {:?}", it, word);
        if *word == starter {
            break;
        }
        curr = *word;
    }
}

#[test]
fn lens() {
    assert_eq!(1, Word::new("a").len());
    assert_eq!(1, Word::new("z").len());
    assert_eq!(2, Word::new("aa").len());
    assert_eq!(2, Word::new("zz").len());
    assert_eq!(7, Word::new("aaaaaaa").len());
    assert_eq!(7, Word::new("zzzzzzz").len());
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
    assert_eq!(Some(Word::new("aa")), Word::new("a").dupl_first(8));
    assert_eq!(Some(Word::new("aab")), Word::new("ab").dupl_first(8));
    assert_eq!(Some(Word::new("aabcde")), Word::new("abcde").dupl_first(8));
    assert_eq!(None, Word::new("abcdefgh").dupl_first(8));
}

#[test]
fn dupl_limit() {
    assert_eq!(None, Word::new("a").dupl_first(1));
    assert_eq!(None, Word::new("ab").dupl_first(2));
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
