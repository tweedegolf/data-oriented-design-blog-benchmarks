use bumpalo::Bump;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        const INPUT: &str = include_str!("/home/folkertdev/rust/dod-benchmarks/data.txt");

        let arena = Bump::new();
        dbg!(standard::parser(&arena, INPUT));
    }
}

/// A span in the input file
#[derive(Debug, Clone, Eq, Copy, PartialEq, PartialOrd, Ord, Hash)]
pub struct Region {
    start_position: u32,
    end_position: u32,
}

#[derive(Debug, Clone, Eq, Copy, PartialEq, PartialOrd, Ord, Hash)]
pub struct Loc<T> {
    region: Region,
    value: T,
}

impl<T> Loc<T> {
    fn at(region: Region, value: T) -> Loc<T> {
        Loc { region, value }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentOrNewline<'a> {
    Newline,
    Comment(&'a [u8]),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TypeDef<'a> {
    _marker: std::marker::PhantomData<&'a ()>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValueDef<'a> {
    _marker: std::marker::PhantomData<&'a ()>,
}

pub mod standard {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub enum Def<'a> {
        Type(TypeDef<'a>),
        Value(ValueDef<'a>),

        // Blank Space (e.g. comments, spaces, newlines) before or after a def.
        // We preserve this for the formatter; canonicalization ignores it.
        SpaceBefore(&'a Def<'a>, &'a [CommentOrNewline<'a>]),
        SpaceAfter(&'a Def<'a>, &'a [CommentOrNewline<'a>]),
    }

    pub type Defs<'a> = Vec<Loc<Def<'a>>>;

    pub fn parser<'a>(arena: &'a Bump, input: &str) -> Defs<'a> {
        let mut defs = Vec::new();
        let mut spaces = Vec::new();

        let mut start_position = 0;

        for (i, c) in input.as_bytes().iter().enumerate() {
            let region = Region {
                start_position,
                end_position: i as u32,
            };

            match c {
                b'T' => {
                    let def = Def::Type(TypeDef::default());
                    let def = Def::SpaceBefore(arena.alloc(def), &*arena.alloc_slice_copy(&spaces));
                    defs.push(Loc::at(region, def));

                    start_position = i as u32 + 1;
                    spaces.clear();
                }
                b'V' => {
                    let def = Def::Type(TypeDef::default());
                    let def = Def::SpaceBefore(arena.alloc(def), &*arena.alloc_slice_copy(&spaces));
                    defs.push(Loc::at(region, def));

                    start_position = i as u32 + 1;
                    spaces.clear();
                }
                b'\n' => {
                    spaces.push(CommentOrNewline::Newline);
                }
                b'C' => {
                    spaces.push(CommentOrNewline::Comment(b"some comment"));
                }
                _ => unreachable!(),
            }
        }

        defs
    }
}

pub mod dod {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum DefTag {
        Type { type_defs_index: u32 },
        Value { value_defs_index: u32 },
    }

    #[derive(Debug, Clone, Default)]
    pub struct Defs<'a> {
        tags: Vec<DefTag>,
        regions: Vec<Region>,

        space_before: Vec<&'a [CommentOrNewline<'a>]>,
        space_after: Vec<&'a [CommentOrNewline<'a>]>,

        value_defs: Vec<ValueDef<'a>>,
        type_defs: Vec<TypeDef<'a>>,
    }

    pub fn parser<'a>(arena: &'a Bump, input: &str) -> Defs<'a> {
        let mut defs = Defs::default();
        let mut spaces = Vec::new();

        let mut start_position = 0;

        for (i, c) in input.as_bytes().iter().enumerate() {
            let region = Region {
                start_position,
                end_position: i as u32,
            };

            match c {
                b'T' => {
                    defs.space_before.push(&*arena.alloc_slice_copy(&spaces));
                    defs.space_after.push(&[] as &[_]);

                    defs.regions.push(region);

                    let tag = DefTag::Type {
                        type_defs_index: defs.type_defs.len() as u32,
                    };
                    defs.tags.push(tag);

                    defs.type_defs.push(TypeDef::default());

                    start_position = i as u32 + 1;
                    spaces.clear();
                }
                b'V' => {
                    defs.space_before.push(&*arena.alloc_slice_copy(&spaces));
                    defs.space_after.push(&[] as &[_]);

                    defs.regions.push(region);

                    let tag = DefTag::Value {
                        value_defs_index: defs.value_defs.len() as u32,
                    };
                    defs.tags.push(tag);

                    defs.value_defs.push(ValueDef::default());

                    start_position = i as u32 + 1;
                    spaces.clear();
                }
                b'\n' => {
                    spaces.push(CommentOrNewline::Newline);
                }
                b'C' => {
                    spaces.push(CommentOrNewline::Comment(b"some comment"));
                }
                _ => unreachable!(),
            }
        }

        defs
    }
}

pub mod boxed {
    use super::*;

    #[derive(Debug, Clone)]
    pub enum Def<'a> {
        Type(TypeDef<'a>),
        Value(ValueDef<'a>),

        // Blank Space (e.g. comments, spaces, newlines) before or after a def.
        // We preserve this for the formatter; canonicalization ignores it.
        SpaceBefore(Box<Def<'a>>, &'a [CommentOrNewline<'a>]),
        SpaceAfter(Box<Def<'a>>, &'a [CommentOrNewline<'a>]),
    }

    pub type Defs<'a> = Vec<Loc<Def<'a>>>;

    pub fn parser<'a>(arena: &'a Bump, input: &str) -> Defs<'a> {
        let mut defs = Vec::new();
        let mut spaces = Vec::new();

        let mut start_position = 0;

        for (i, c) in input.as_bytes().iter().enumerate() {
            let region = Region {
                start_position,
                end_position: i as u32,
            };

            match c {
                b'T' => {
                    let def = Def::Type(TypeDef::default());
                    let def = Def::SpaceBefore(Box::new(def), &*arena.alloc_slice_copy(&spaces));
                    defs.push(Loc::at(region, def));

                    start_position = i as u32 + 1;
                    spaces.clear();
                }
                b'V' => {
                    let def = Def::Type(TypeDef::default());
                    let def = Def::SpaceBefore(Box::new(def), &*arena.alloc_slice_copy(&spaces));
                    defs.push(Loc::at(region, def));

                    start_position = i as u32 + 1;
                    spaces.clear();
                }
                b'\n' => {
                    spaces.push(CommentOrNewline::Newline);
                }
                b'C' => {
                    spaces.push(CommentOrNewline::Comment(b"some comment"));
                }
                _ => unreachable!(),
            }
        }

        defs
    }
}
