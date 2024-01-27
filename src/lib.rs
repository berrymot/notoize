use serde::Deserialize;
use itertools::Itertools;
use std::{any::Any, collections::{HashMap, HashSet}, fs};

pub struct NotoizeConfig {
    pub lgc: Vec<Serifness>,
    pub armenian: Vec<Serifness>,
    pub balinese: Vec<Serifness>,
    pub bengali: Vec<Serifness>,
    pub devanagari: Vec<Serifness>,
    pub ethiopic: Vec<Serifness>,
    pub georgian: Vec<Serifness>,
    pub grantha: Vec<Serifness>,
    pub gujarati: Vec<Serifness>,
    pub gurmukhi: Vec<Serifness>,
    pub kannada: Vec<Serifness>,
    pub khmer: Vec<Serifness>,
    pub khojki: Vec<Serifness>,
    pub malayalam: Vec<Serifness>,
    pub myanmar: Vec<Serifness>,
    pub oriya: Vec<Serifness>,
    pub sinhala: Vec<Serifness>,
    pub tamil: Vec<Serifness>,
    pub telugu: Vec<Serifness>,
    pub vithkuqi: Vec<Serifness>,
    // not just sans/serif
    pub adlam: Vec<AdlamNkoCfg>,
    pub nko: Vec<AdlamNkoCfg>,
    pub arabic: Vec<ArabicCfg>,
    pub hebrew: Vec<HebrewCfg>,
    pub khitan: Vec<KhitanCfg>,
    pub nushu: Vec<NushuCfg>,
    pub syriac: Vec<SyriacCfg>,
    pub thai: Vec<ThaiLaoCfg>,
    pub lao: Vec<ThaiLaoCfg>,
    // cjk
    pub cjk: Vec<(Serifness, CjkVariant)>
}
impl NotoizeConfig {
    pub fn new_sans() -> Self {
        Self {
            lgc: vec![Serifness::Sans],
            armenian: vec![Serifness::Sans],
            balinese: vec![Serifness::Sans],
            bengali: vec![Serifness::Sans],
            devanagari: vec![Serifness::Sans],
            ethiopic: vec![Serifness::Sans],
            georgian: vec![Serifness::Sans],
            grantha: vec![Serifness::Sans],
            gujarati: vec![Serifness::Sans],
            gurmukhi: vec![Serifness::Sans],
            kannada: vec![Serifness::Sans],
            khmer: vec![Serifness::Sans],
            khojki: vec![Serifness::Sans],
            malayalam: vec![Serifness::Sans],
            myanmar: vec![Serifness::Sans],
            oriya: vec![Serifness::Sans],
            sinhala: vec![Serifness::Sans],
            tamil: vec![Serifness::Sans],
            telugu: vec![Serifness::Sans],
            vithkuqi: vec![Serifness::Sans],
            //
            adlam: vec![AdlamNkoCfg::Sans],
            nko: vec![AdlamNkoCfg::Sans],
            arabic: vec![ArabicCfg::Sans],
            hebrew: vec![HebrewCfg::Sans],
            khitan: vec![KhitanCfg::Serif],
            nushu: vec![NushuCfg::Sans],
            syriac: vec![SyriacCfg::Sans],
            thai: vec![ThaiLaoCfg::SansUnlooped],
            lao: vec![ThaiLaoCfg::SansUnlooped],
            //
            cjk: vec![(Serifness::Sans, CjkVariant::Sc)]
        }
    }
    pub fn prefer_serif() -> Self {
        Self {
            lgc: vec![Serifness::Serif],
            armenian: vec![Serifness::Serif],
            balinese: vec![Serifness::Serif],
            bengali: vec![Serifness::Serif],
            devanagari: vec![Serifness::Serif],
            ethiopic: vec![Serifness::Serif],
            georgian: vec![Serifness::Serif],
            grantha: vec![Serifness::Serif],
            gujarati: vec![Serifness::Serif],
            gurmukhi: vec![Serifness::Serif],
            kannada: vec![Serifness::Serif],
            khmer: vec![Serifness::Serif],
            khojki: vec![Serifness::Serif],
            malayalam: vec![Serifness::Serif],
            myanmar: vec![Serifness::Serif],
            oriya: vec![Serifness::Serif],
            sinhala: vec![Serifness::Serif],
            tamil: vec![Serifness::Serif],
            telugu: vec![Serifness::Serif],
            vithkuqi: vec![Serifness::Serif],
            //
            arabic: vec![ArabicCfg::Naskh],
            hebrew: vec![HebrewCfg::Serif],
            khitan: vec![KhitanCfg::Serif],
            thai: vec![ThaiLaoCfg::Serif],
            lao: vec![ThaiLaoCfg::Serif],
            //
            cjk: vec![(Serifness::Serif, CjkVariant::Sc)],
            ..Self::new_sans()
        }
    }
}

pub enum Serifness {Sans, Serif}
pub enum AdlamNkoCfg {Sans, Unjoined}
pub enum ArabicCfg {Sans, Kufi, Naskh, NaskhUi, Nastaliq}
pub enum HebrewCfg {Sans, Serif, Rashi}
pub enum KhitanCfg {Serif, Vertical, Rotated}
pub enum NushuCfg {Sans, Traditional}
pub enum SyriacCfg {Sans, Western, Eastern}
pub enum ThaiLaoCfg {SansLooped, SansUnlooped, Serif}
pub enum CjkVariant {Sc, Tc, Hk, Jp, Kr}

pub struct FontStack(pub Vec<String>);

#[derive(Debug)]
pub struct Font {
    pub filename: String,
    pub fontname: String,
    pub bytes: Vec<u8>
}

impl FontStack {
    pub fn files(& self) -> Vec<Font> {
        self.0.clone().iter().map(|x| Font {
            filename: "NotoSans-Regular.otf".to_string(),
            fontname: x.to_string(),
            bytes: fs::read("notofonts.github.io/fonts/NotoSans/full/otf/NotoSans-Regular.otf").unwrap()
        }).collect()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct BlockData {
    name: String,
    cps: HashMap<String, CodepointFontSupport>,
    fonts: Option<Vec<String>>
}

#[derive(Deserialize, Debug, Clone)]
pub struct CodepointFontSupport {
    fonts: Option<Vec<String>>,
}

/// Returns a minimal font stack for rendering `text`
pub fn notoize(text: &str, config: NotoizeConfig) -> Vec<String> {
    let font_support = (0..=323).map(
        |i| serde_json::from_str::<BlockData>(
            &fs::read_to_string(format!("overview/blocks/block-{i:03}.json")).unwrap()
        ).unwrap()
    ).enumerate().flat_map(
        |(i, mut e)| {
            // the entries will all parse as u32 except this one.
            // this way we still return an iterator & pass the index of the block
            e.cps.insert(format!("i={i}"), CodepointFontSupport {fonts: None}); e.cps
        }
    ).collect_vec();
    let font_support = font_support.iter().map(
        |(k, v)| if k.clone().parse::<u32>().is_ok() {
            (k.clone().parse::<i32>().unwrap(), match v.clone().fonts {
                Some(vf) => vf,
                // FIXME: this doesn't actually work
                None => font_support.iter().filter(|(k, _)| k.parse::<u32>().is_err()).collect_vec()[0].1.fonts.clone().unwrap_or(vec![])
            })
        } else {
            (-1, vec![])
        }
    ).filter(|e| e.0 != -1).map(|e| (e.0 as u32, e.1)).sorted_by_key(|&(k, _)| k).collect_vec();
    let fonts = HashSet::new();
    for c in text.chars() {
        let codepoint = c as u32;
        let hex = format!("{codepoint:04x}");
        let f = font_support.iter().find(|(n, _)| n == &codepoint).cloned().unwrap_or((codepoint, vec![])).1;
        println!("{hex} {f:?}");
    }
    fonts.into_iter().collect()
}