use itertools::Itertools;

use crate::NasinNanpaVariation;

/// An encoding position (either a number, or `None` which prints `-1`)
#[derive(Clone)]
pub enum EncPos {
    Pos(usize),
    None,
}

impl EncPos {
    fn inc(&mut self) {
        *self = match self {
            EncPos::Pos(p) => EncPos::Pos(*p + 1),
            EncPos::None => EncPos::None,
        };
    }

    fn gen(&self) -> String {
        match self {
            EncPos::Pos(p) => p.to_string(),
            EncPos::None => "-1".to_string(),
        }
    }
}

/// An encoding, consisting of a fontforge position and an encoding position
#[derive(Clone)]
pub struct Encoding {
    pub ff_pos: usize,
    pub enc_pos: EncPos,
}

impl Encoding {
    pub fn new(ff_pos: usize, enc_pos: EncPos) -> Self {
        Self { ff_pos, enc_pos }
    }

    pub fn gen(&self) -> String {
        format!(
            "Encoding: {ff_pos} {enc_pos} {ff_pos}",
            ff_pos = self.ff_pos,
            enc_pos = self.enc_pos.gen(),
        )
    }

    pub fn gen_ref(&self, position: String) -> String {
        let Encoding { ff_pos, enc_pos } = self;
        format!(
            "Refer: {ff_pos} {enc_pos} {position}",
            enc_pos = enc_pos.gen(),
            position = position,
        )
    }
}

/// A glyph reference (with positional data)
#[derive(Clone)]
pub struct Ref {
    ref_glyph: Encoding,
    position: String,
}

impl Ref {
    pub fn new(ref_glyph: Encoding, position: impl Into<String>) -> Self {
        Self {
            ref_glyph,
            position: position.into(),
        }
    }

    pub fn gen(&self) -> String {
        self.ref_glyph.gen_ref(self.position.clone())
    }
}

/// A glyph representation, consisting of a spline set and references
#[derive(Default, Clone)]
pub struct Rep {
    spline_set: String,
    references: Vec<Ref>,
}

impl Rep {
    pub fn new(spline_set: impl Into<String>, references: Vec<Ref>) -> Self {
        Self {
            spline_set: spline_set.into(),
            references,
        }
    }

    pub fn gen(&self) -> String {
        let f = if !self.spline_set.is_empty() || !self.references.is_empty() {
            "Fore\n"
        } else {
            ""
        };

        let r = self
            .references
            .clone()
            .into_iter()
            .map(|r| r.gen())
            .join("\n");

        let nl = if !self.references.is_empty() {
            "\n"
        } else {
            ""
        };

        let s = if !self.spline_set.is_empty() {
            format!("SplineSet{s}\nEndSplineSet\n", s = self.spline_set)
        } else {
            String::new()
        };

        format!("{f}{r}{nl}{s}")
    }
}

/// An anchor class, either stack or scale
#[derive(Clone)]
pub enum AnchorClass {
    Stack,
    Scale,
}

/// An anchor type, either base (for lower/outer) or mark (for upper/inner)
#[derive(Clone, Copy)]
pub enum AnchorType {
    Base,
    Mark,
}

/// An anchor, consisting of a class, type, and position
#[derive(Clone)]
pub struct Anchor {
    class: AnchorClass,
    ty: AnchorType,
    pos: (isize, isize),
}

impl Anchor {
    pub const fn new_stack(ty: AnchorType) -> Self {
        Self {
            class: AnchorClass::Stack,
            ty,
            pos: (
                match ty {
                    AnchorType::Base => 500,
                    AnchorType::Mark => -500,
                },
                400,
            ),
        }
    }

    pub const fn new_scale(ty: AnchorType, pos: (isize, isize)) -> Self {
        Self {
            class: AnchorClass::Scale,
            ty,
            pos,
        }
    }

    fn gen(&self) -> String {
        let class = match self.class {
            AnchorClass::Stack => "stack",
            AnchorClass::Scale => "scale",
        };
        let x = self.pos.0;
        let y = self.pos.1;
        let ty = match self.ty {
            AnchorType::Base => "basechar",
            AnchorType::Mark => "mark",
        };
        format!("AnchorPoint: \"{class}\" {x} {y} {ty} 0\n")
    }
}


/// This is the smallest building block of a glyph, containing the name, width, representation, and optional anchor
#[derive(Clone)]
pub struct GlyphBasic {
    pub name: String,
    pub width: usize,
    pub rep: Rep,
    pub anchor: Option<Anchor>,
}

impl GlyphBasic {
    pub fn new(name: impl Into<String>, width: usize, rep: Rep, anchor: Option<Anchor>) -> Self {
        Self {
            name: name.into(),
            width,
            rep,
            anchor,
        }
    }
}

/// This is a `GlyphBasic` that has been assigned an `EncPos`
pub struct GlyphEnc {
    glyph: GlyphBasic,
    enc: EncPos,
}

#[allow(unused)]
impl GlyphEnc {
    pub fn new_from_basic(glyph: GlyphBasic, enc: EncPos) -> Self {
        Self { glyph, enc }
    }

    pub fn new_from_parts(enc: EncPos, name: impl Into<String>, width: usize, rep: Rep) -> Self {
        Self {
            glyph: GlyphBasic::new(name, width, rep, None),
            enc,
        }
    }
}

/// 
pub enum LookupsMode {
    WordLigFromLetters,
    WordLigManual(Vec<String>),
    StartLongGlyph,
    Alt,
    ComboFirst,
    ComboLast,
    None,
}

#[derive(Clone)]
pub enum Lookups {
    WordLigFromLetters,
    WordLigManual(String),
    StartLongGlyph,
    EndLongGlyph,
    Alt,
    ComboFirst,
    ComboLast,
    None,
}

impl Lookups {
    fn new_from_mode(mode: &LookupsMode, idx: usize) -> Self {
        match mode {
            LookupsMode::WordLigFromLetters => Lookups::WordLigFromLetters,
            LookupsMode::WordLigManual(vec) => {
                let s = &vec[idx];
                if s.len() > 0 {
                    Lookups::WordLigManual(vec[idx].clone())
                } else {
                    Lookups::None
                }
            }
            LookupsMode::StartLongGlyph => Lookups::StartLongGlyph,
            LookupsMode::Alt => Lookups::Alt,
            LookupsMode::ComboFirst => Lookups::ComboFirst,
            LookupsMode::ComboLast => Lookups::ComboLast,
            LookupsMode::None => Lookups::None,
        }
    }

    fn gen(&self, name: String, full_name: String, variation: NasinNanpaVariation) -> String {

        let latin_ligs = match &self {
            
            // Used in tok_block and tok_ext_block when NasinNanpaVariation == Main
            Lookups::WordLigFromLetters => {
                let lig = name.chars().join(" ");
                let ali = if full_name.eq("aleTok") {
                    "Ligature2: \"'liga' WORD PLUS SPACE\" a l i space\nLigature2: \"'liga' WORD\" a l i\n"
                } else {
                    ""
                };
                format!("Ligature2: \"'liga' WORD PLUS SPACE\" {lig} space\nLigature2: \"'liga' WORD\" {lig}\n{ali}")
            }

            // Used in ctrl_block, tok_ctrl_block, and tok_no_combo_block
            Lookups::WordLigManual(word) => {

                let mut do_it = true;
                let always = if word.contains("middleDotTok") {
                    do_it = false;
                    format!("Ligature2: \"'liga' VAR\" {word}\n")
                } else if word.contains("CartAlt") {
                    format!(
                        "Ligature2: \"'liga' VAR\" {which}Tok VAR01\n",
                        which = if word.contains("start") { "startCart" } else { "endCart" }
                    )
                } else if name.eq("ZWJ") {
                    "Substitution2: \"'ss02' BECOME STACK\" joinStackTok\nSubstitution2: \"'ss01' BECOME SCALE\" joinScaleTok\n".to_string()
                } else if word.eq("i t a n") {
                    "Ligature2: \"'liga' VAR\" ijoTok ZWJ tanTok ZWJ anpaTok ZWJ nanpaTok\n".to_string()
                } else if word.eq("l e p e k a") {
                    "Ligature2: \"'liga' VAR\" meliTok ZWJ kuleTok ZWJ kuleTok\n".to_string()
                } else {
                    String::new()
                };

                let latin = if variation == NasinNanpaVariation::Main && do_it {
                    if word.eq("space space") {
                        format!("Ligature2: \"'liga' SPACE\" {word}\nLigature2: \"'liga' SPACE\" z z space\nLigature2: \"'liga' SPACE\" z z\n")
                    } else if word.eq("arrow") {
                        let convert = |c: char| match c {
                            'W' => "less",
                            'N' => "asciicircum",
                            'E' => "greater",
                            'S' => "v",
                            _ => panic!(),
                        };

                        let dir1 = convert(name.chars().nth(5).unwrap());
                        if let Some(dir2) = name.chars().nth(6) {
                            let dir2 = convert(dir2);
                            format!("Ligature2: \"'liga' WORD PLUS SPACE\" {dir1} {dir2} space\nLigature2: \"'liga' WORD PLUS SPACE\" {dir2} {dir1} space\nLigature2: \"'liga' WORD\" {dir1} {dir2}\nLigature2: \"'liga' WORD\" {dir2} {dir1}\n")
                        } else {
                            format!("Ligature2: \"'liga' WORD PLUS SPACE\" {dir1} space\nLigature2: \"'liga' WORD\" {dir1}\n")
                        }
                    } else if word.eq("bar") {
                        format!("Ligature2: \"'liga' WORD PLUS SPACE\" bar space\nLigature2: \"'liga' WORD\" bar\n")
                    } else if word.contains("CartAlt") {
                            format!(
                                "Ligature2: \"'liga' VAR PLUS SPACE\" {which}Tok VAR01 space\nLigature2: \"'liga' VAR PLUS SPACE\" {which}Tok one space\nLigature2: \"'liga' VAR\" {which}Tok VAR01\nLigature2: \"'liga' VAR\" {which}Tok one\n",
                                which = if word.contains("start") { "startCart" } else { "endCart" }
                            )
                    } else {
                        format!("Ligature2: \"'liga' WORD PLUS SPACE\" {word} space\nLigature2: \"'liga' WORD\" {word}\n")
                    }
                } else {
                    String::new()
                };

                format!("{always}{latin}")
            } // Lookups::WordLigManual

            // Used in start_long_glyph_block
            Lookups::StartLongGlyph => {
                let (glyph, joiner) = full_name.rsplit_once("_").unwrap();
                format!("Ligature2: \"'liga' START CONTAINER\" {glyph} {joiner}\n")
            }

            // Used in start_long_glyph_block for laTok
            Lookups::EndLongGlyph => {
                let (glyph, _) = full_name.split_once("_").unwrap();
                format!("Ligature2: \"'liga' START CONTAINER\" endRevLongGlyphTok {glyph}\n")
            }

            // Used in tok_alt_block
            Lookups::Alt => {
                let parts: Vec<&str> = full_name.split("_").collect();
                let glyph = parts[0];
                let sel = parts[1];

                let a = if full_name.eq("aTok_VAR01") {
                    "Ligature2: \"'liga' VAR\" semeTok ZWJ aTok\nLigature2: \"'liga' VAR\" aTok ZWJ semeTok\n"
                } else if full_name.eq("aTok_VAR02") {
                    "Ligature2: \"'liga' VAR\" aTok aTok\n"
                } else if full_name.eq("aTok_VAR03") {
                    "Ligature2: \"'liga' VAR\" aTok aTok aTok\n"
                } else if full_name.eq("aTok_VAR04") && variation == NasinNanpaVariation::Main {
                    "Ligature2: \"'liga' VAR PLUS SPACE\" exclam question space\nLigature2: \"'liga' VAR PLUS SPACE\" question exclam space\nLigature2: \"'liga' VAR\" exclam question\nLigature2: \"'liga' VAR\" question exclam\n"
                } else {
                    ""
                };

                let arrow_lig = if full_name.contains("niTok_arrow") {
                    format!("Ligature2: \"'liga' VAR\" {glyph} ZWJ {sel}\n")
                } else {
                    String::new()
                };

                let num_lig = if variation == NasinNanpaVariation::Main {
                    format!(
                        "Ligature2: \"'liga' VAR PLUS SPACE\" {glyph} {sel} space\nLigature2: \"'liga' VAR\" {glyph} {sel}\n",
                        sel = match sel {
                            "VAR01" | "arrowW" => "one",
                            "VAR02" | "arrowN" => "two",
                            "VAR03" | "arrowE" => "three",
                            "VAR04" | "arrowS" => "four",
                            "VAR05" | "arrowNW" => "five",
                            "VAR06" | "arrowNE" => "six",
                            "VAR07" | "arrowSE" => "seven",
                            "VAR08" | "arrowSW" => "eight",
                            _ => panic!(),
                        }
                    )
                } else {
                    String::new()
                };

                let rerand = {
                    let sel_word = match sel {
                        "VAR01" | "arrowW" => "one",
                        "VAR02" | "arrowN" => "two",
                        "VAR03" | "arrowE" => "three",
                        "VAR04" | "arrowS" => "four",
                        "VAR05" | "arrowNW" => "five",
                        "VAR06" | "arrowNE" => "six",
                        "VAR07" | "arrowSE" => "seven",
                        "VAR08" | "arrowSW" => "eight",
                        _ => panic!(),
                    };
                    let sel = sel.chars().last().unwrap().to_string();
                    if full_name.starts_with("jakiTok") {
                        if variation == NasinNanpaVariation::Main {
                            (1..9).map(|n| format!("Ligature2: \"'liga' VAR PLUS SPACE\" jakiTok_VAR0{n} VAR0{sel} space\nLigature2: \"'liga' VAR PLUS SPACE\" jakiTok_VAR0{n} {sel_word} space\nLigature2: \"'liga' VAR\" jakiTok_VAR0{n} VAR0{sel}\nLigature2: \"'liga' VAR\" jakiTok_VAR0{n} {sel_word}\n")).collect::<String>()
                        } else {
                            (1..9).map(|n| format!("Ligature2: \"'liga' VAR\" jakiTok_VAR0{n} VAR0{sel}\n")).collect::<String>()
                        }
                    } else if full_name.starts_with("koTok") {
                        if variation == NasinNanpaVariation::Main {
                            (1..9).map(|n| format!("Ligature2: \"'liga' VAR PLUS SPACE\" koTok_VAR0{n} VAR0{sel} space\nLigature2: \"'liga' VAR PLUS SPACE\" koTok_VAR0{n} {sel_word} space\nLigature2: \"'liga' VAR\" koTok_VAR0{n} VAR0{sel}\nLigature2: \"'liga' VAR\" koTok_VAR0{n} {sel_word}\n")).collect::<String>()
                        } else {
                            (1..9).map(|n| format!("Ligature2: \"'liga' VAR\" koTok_VAR0{n} VAR0{sel}\n")).collect::<String>()
                        }
                    } else {
                        String::new()
                    }
                };

                let if_main = if variation == NasinNanpaVariation::Main {
                    format!("Ligature2: \"'liga' VAR PLUS SPACE\" {glyph} {sel} space\n")
                } else {
                    String::new()
                };
                format!("{a}{if_main}Ligature2: \"'liga' VAR\" {glyph} {sel}\n{arrow_lig}{num_lig}{rerand}")
            }

            // Used in tok_outer_block, tok_ext_outer_block, tok_alt_outer_block,
            // tok_lower_block, tok_ext_lower_block, and tok_alt_lower_block.
            Lookups::ComboFirst => {
                let (glyph, joiner) = full_name.rsplit_once('_').unwrap();
                format!("Ligature2: \"'liga' GLYPH THEN JOINER\" {glyph} {joiner}\nMultipleSubs2: \"'ccmp' RESPAWN JOINER\" {full_name} {joiner}\n")
            }

            // Used in tok_inner_block, tok_ext_inner_block, tok_alt_inner_block,
            // tok_upper_block, tok_ext_upper_block, and tok_alt_upper_block.
            Lookups::ComboLast => {
                let (joiner, glyph) = full_name.split_once("_").unwrap();
                format!("Ligature2: \"'liga' JOINER THEN GLYPH\" {joiner} {glyph}\nLigature2: \"'liga' CC CLEANUP\" combCartExtHalfTok {full_name}\nLigature2: \"'liga' CC CLEANUP\" combLongGlyphExtHalfTok {full_name}\nLigature2: \"'liga' CC CLEANUP\" combCartExtTok {full_name}\nLigature2: \"'liga' CC CLEANUP\" combLongGlyphExtTok {full_name}\n")
            }
            Lookups::None => String::new(),
        };

        let rand = if full_name.eq("jakiTok") {
            format!(
                "{rerand}AlternateSubs2: \"'rand' RAND VARIATIONS\" jakiTok_VAR01 jakiTok_VAR02 jakiTok_VAR03 jakiTok_VAR04 jakiTok_VAR05 jakiTok_VAR06 jakiTok_VAR07 jakiTok_VAR08\n",
                rerand = if variation == NasinNanpaVariation::Main {
                    (1..9).map(|n| format!("Ligature2: \"'liga' VAR PLUS SPACE\" jakiTok_VAR0{n} VAR09 space\nLigature2: \"'liga' VAR PLUS SPACE\" jakiTok_VAR0{n} nine space\nLigature2: \"'liga' VAR\" jakiTok_VAR0{n} VAR09\nLigature2: \"'liga' VAR\" jakiTok_VAR0{n} nine\n")).collect::<String>()
                } else { 
                    (1..9).map(|n| format!("Ligature2: \"'liga' VAR\" jakiTok_VAR0{n} VAR09\n")).collect::<String>()
                }
            )
        } else if full_name.eq("koTok") {
            format!(
                "{rerand}AlternateSubs2: \"'rand' RAND VARIATIONS\" koTok_VAR01 koTok_VAR02 koTok_VAR03 koTok_VAR04 koTok_VAR05 koTok_VAR06 koTok_VAR07 koTok_VAR08\n",
                rerand = if variation == NasinNanpaVariation::Main { 
                    (1..9).map(|n| format!("Ligature2: \"'liga' VAR PLUS SPACE\" koTok_VAR0{n} VAR09 space\nLigature2: \"'liga' VAR PLUS SPACE\" koTok_VAR0{n} nine space\nLigature2: \"'liga' VAR\" koTok_VAR0{n} VAR09\nLigature2: \"'liga' VAR\" koTok_VAR0{n} nine\n")).collect::<String>()
                } else {
                    (1..9).map(|n| format!("Ligature2: \"'liga' VAR\" koTok_VAR0{n} VAR09\n")).collect::<String>()
                }
            )
        } else {
            String::new()
        };

        format!("{latin_ligs}{rand}")
    }
}

#[derive(Clone)]
pub enum Cc {
    Full,
    Half,
    Participant,
    None,
}

#[derive(Clone)]
pub struct GlyphFull {
    pub glyph: GlyphBasic,
    pub encoding: Encoding,
    pub lookups: Lookups,
    pub cc_subs: Cc,
}

impl GlyphFull {
    pub fn new_from_basic(
        glyph: GlyphBasic,
        encoding: Encoding,
        lookups: Lookups,
        cc_subs: Cc,
    ) -> Self {
        Self {
            glyph,
            encoding,
            lookups,
            cc_subs,
        }
    }

    pub fn new_from_enc(glyph: GlyphEnc, ff_pos: usize, lookups: Lookups, cc_subs: Cc) -> Self {
        Self {
            glyph: glyph.glyph,
            encoding: Encoding::new(ff_pos, glyph.enc),
            lookups,
            cc_subs,
        }
    }

    pub fn new_from_parts(
        name: impl Into<String>,
        width: usize,
        rep: Rep,
        anchor: Option<Anchor>,
        encoding: Encoding,
        lookups: Lookups,
        cc_subs: Cc,
    ) -> Self {
        Self {
            glyph: GlyphBasic::new(name, width, rep, anchor),
            encoding,
            lookups,
            cc_subs,
        }
    }

    pub fn gen(
        &self,
        prefix: String,
        suffix: String,
        color: String,
        variation: NasinNanpaVariation,
    ) -> String {
        let name = &self.glyph.name;
        let encoding = self.encoding.gen();
        let color = format!("Colour: {color}");
        if name.contains("empty") {
            return format!(
                "\nStartChar: {name}\n{encoding}\nWidth: 0\nLayerCount: 2\n{color}\nEndChar\n"
            );
        }
        let full_name = format!("{}{}{}", prefix, name, suffix);
        let width = self.glyph.width;
        let representation = self.glyph.rep.gen();
        let lookups = self
            .lookups
            .gen(name.to_string(), full_name.clone(), variation);
        let cc_subs = match self.cc_subs {
            Cc::Full => format!("MultipleSubs2: \"'cc01' CART\" {full_name} combCartExtTok\nMultipleSubs2: \"'cc02' CONT\" {full_name} combLongGlyphExtTok\n"),
            Cc::Half => format!("MultipleSubs2: \"'cc01' CART\" {full_name} combCartExtHalfTok\nMultipleSubs2: \"'cc02' CONT\" {full_name} combLongGlyphExtHalfTok\n"),
            Cc::Participant => format!("MultipleSubs2: \"'cc01' CART\" {full_name} combCartExtNoneTok\nMultipleSubs2: \"'cc02' CONT\" {full_name} combCartExtNoneTok\n"),
            Cc::None => String::new(),
        };
        let flags = if full_name.eq("ZWJ")
            || full_name.eq("ZWNJ")
            || full_name.starts_with("VAR")
            || full_name.starts_with("arrow")
            || full_name.eq("joinStackTok")
            || full_name.eq("joinScaleTok")
            || full_name.contains("space")
            || full_name.eq("combCartExtNoneTok")
        {
            "Flags: W\n"
        } else {
            ""
        };
        let anchor = if let Some(anchor) = &self.glyph.anchor {
            anchor.gen()
        } else {
            String::new()
        };
        format!("\nStartChar: {full_name}\n{encoding}\nWidth: {width}\n{flags}{anchor}LayerCount: 2\n{representation}{lookups}{cc_subs}{color}\nEndChar\n")
    }
}

pub struct GlyphDescriptor {
    pub name: &'static str,
    pub spline_set: &'static str,
    pub width: Option<usize>,
    pub anchor: Option<Anchor>,
}

impl GlyphDescriptor {
    pub const fn new(name: &'static str, spline_set: &'static str) -> Self {
        Self {
            name,
            spline_set,
            width: None,
            anchor: None,
        }
    }

    pub const fn new_with_width(
        name: &'static str,
        width: usize,
        spline_set: &'static str,
    ) -> Self {
        Self {
            name,
            spline_set,
            width: Some(width),
            anchor: None,
        }
    }

    pub const fn new_with_anchor(
        name: &'static str,
        anchor: Anchor,
        spline_set: &'static str,
    ) -> Self {
        Self {
            name,
            spline_set,
            width: None,
            anchor: Some(anchor),
        }
    }
}

pub struct GlyphBlock {
    pub glyphs: Vec<GlyphFull>,
    pub prefix: String,
    pub suffix: String,
    pub color: String,
}

impl GlyphBlock {
    pub fn new_from_enc_glyphs(
        ff_pos: &mut usize,
        glyphs: Vec<GlyphEnc>,
        lookups: LookupsMode,
        cc_subs: Cc,
        prefix: impl Into<String>,
        suffix: impl Into<String>,
        color: impl Into<String>,
    ) -> Self {
        let mut glyphs: Vec<GlyphFull> = glyphs
            .into_iter()
            .enumerate()
            .map(|(idx, glyph)| {
                let g = GlyphFull::new_from_enc(
                    glyph,
                    *ff_pos,
                    Lookups::new_from_mode(&lookups, idx),
                    cc_subs.clone(),
                );
                *ff_pos += 1;
                g
            })
            .collect();

        let mut padding = Self::new_empty(ff_pos, 15 - ((glyphs.len() + 15) % 16), 0).glyphs;
        glyphs.append(&mut padding);

        Self {
            glyphs,
            prefix: prefix.into(),
            suffix: suffix.into(),
            color: color.into(),
        }
    }

    pub fn new_from_basic_glyphs(
        ff_pos: &mut usize,
        glyphs: Vec<GlyphBasic>,
        lookups: LookupsMode,
        cc_subs: Cc,
        prefix: impl Into<String>,
        suffix: impl Into<String>,
        color: impl Into<String>,
        mut enc_pos: EncPos,
    ) -> Self {
        let mut glyphs: Vec<GlyphFull> = glyphs
            .into_iter()
            .enumerate()
            .map(|(idx, glyph)| {
                let g = GlyphFull::new_from_basic(
                    glyph,
                    Encoding::new(*ff_pos, enc_pos.clone()),
                    Lookups::new_from_mode(&lookups, idx),
                    cc_subs.clone(),
                );
                *ff_pos += 1;
                enc_pos.inc();
                g
            })
            .collect();

        let mut padding = Self::new_empty(ff_pos, 15 - ((glyphs.len() + 15) % 16), 0).glyphs;
        glyphs.append(&mut padding);

        Self {
            glyphs,
            prefix: prefix.into(),
            suffix: suffix.into(),
            color: color.into(),
        }
    }

    pub fn new_from_constants(
        ff_pos: &mut usize,
        glyphs: &'static [GlyphDescriptor],
        lookups: LookupsMode,
        cc_subs: Cc,
        prefix: impl Into<String>,
        suffix: impl Into<String>,
        color: impl Into<String>,
        enc_pos: EncPos,
        fallback_width: usize,
    ) -> Self {
        let glyphs: Vec<GlyphBasic> = glyphs
            .into_iter()
            .map(
                |GlyphDescriptor {
                     name,
                     spline_set,
                     width,
                     anchor,
                 }| {
                    GlyphBasic::new(
                        name.to_string(),
                        width.unwrap_or(fallback_width),
                        Rep::new(spline_set.to_string(), vec![]),
                        anchor.clone(),
                    )
                },
            )
            .collect();

        Self::new_from_basic_glyphs(
            ff_pos, glyphs, lookups, cc_subs, prefix, suffix, color, enc_pos,
        )
    }

    /// Generates a `GlyphBlock` whose glyphs are all references this block's glyphs, all with the same `rel_pos`
    pub fn new_from_refs(
        &self,
        ff_pos: &mut usize,
        rel_pos: String,
        lookups: LookupsMode,
        cc_subs: Cc,
        use_full_names: bool,
        prefix: impl Into<String>,
        suffix: impl Into<String>,
        color: impl Into<String>,
        width: Option<usize>,
        anchor: Option<Anchor>,
    ) -> Self {
        let glyphs: Vec<GlyphBasic> = self
            .glyphs
            .clone()
            .into_iter()
            .map(
                |GlyphFull {
                     glyph, encoding, ..
                 }| {
                    let refs = vec![
                        Some(Ref::new(encoding.clone(), rel_pos.clone())),
                        None,
                    ]
                    .into_iter()
                    .flatten()
                    .collect();
                    let name = if use_full_names {
                        format!(
                            "{pre}{name}{post}",
                            pre = self.prefix,
                            name = glyph.name,
                            post = self.suffix
                        )
                    } else {
                        glyph.name
                    };
                    let g = GlyphBasic::new(
                        name,
                        match width {
                            Some(width) => width,
                            None => glyph.width,
                        },
                        Rep::new(String::default(), refs),
                        match &anchor {
                            Some(anchor) => Some(anchor.clone()),
                            None => glyph.anchor,
                        },
                    );
                    g
                },
            )
            .collect();

        Self::new_from_basic_glyphs(
            ff_pos,
            glyphs,
            lookups,
            cc_subs,
            prefix,
            suffix,
            color,
            EncPos::None,
        )
    }

    /// Generates a `GlyphBlock` with a given `count` of empty glyphs
    pub fn new_empty(ff_pos: &mut usize, count: usize, width: usize) -> Self {
        let end = *ff_pos + count;
        let mut glyphs = vec![];

        while *ff_pos < end {
            glyphs.push(GlyphFull::new_from_parts(
                format!("empty{i:04}", i = *ff_pos),
                width,
                Rep::default(),
                None,
                Encoding::new(*ff_pos, EncPos::None),
                Lookups::None,
                Cc::None,
            ));
            *ff_pos += 1;
        }

        Self {
            glyphs,
            prefix: String::default(),
            suffix: String::default(),
            color: "dddddd".to_string(),
        }
    }

    /// Generates a `GlyphBlock`
    pub fn gen(&self, variation: NasinNanpaVariation) -> String {
        let mut s = String::new();
        for g in &self.glyphs {
            s += &g.gen(
                self.prefix.clone(),
                self.suffix.clone(),
                self.color.clone(),
                variation,
            )
        }
        s
    }
}
