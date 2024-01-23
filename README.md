# notoize

A crate that tells you what Noto font stack you need.

## Per-font options (preferences)


```rust
let config = NotoizeConfig {
    prefer_math: true,
    adlam: vec![AdlamNkoCfg::Unjoined],
    ..NotoizeConfig::new_sans()
};
```

### bools

- `prefer_math`: over Noto Sans Symbols, for the 222 characters in both

### Script-specific

You supply lists to `NotoizeConfig` for `notoize()`.

There are no options for
- Music
- **Sans only:** Anatolian Hieroglyphs, Avestan, Bamum, Bassa Vah, Batak, Bhaisuki, Brahmi, Buginese, Buhid, Canadian Aboriginal, Carian, Caucasian Albanian, Chakma, Cham, Cherokee, Chorasmian, Coptic, Cuneiform, Cypriot, Cypro-Minoan, Deseret, Duployan, Egyptian Hieroglyphs, Elbasan, Elymaic, Glagolitic, Gothic, Gunjala Gondi, Hanifi Rohingya, Hanunoo, Hatran, Imperial Aramaic, Indic Siyaq Numbers, Inscriptional Pahlavi, Inscriptional Parthian, Javanese, Kaithi, Kawi, Kayah Li, Kharoshthi, Khudawadi, Lepcha, Limbu, Linear A, Linear B, Lisu, Lycian, Lydian, Mahajani, Mandaic, Manichaean, Masaram Gondi, Math, Mayan Numerals, Medefaidrin, Meetei Mayek, Mende Kikakui, Meroitic, Miao, Modi, Mongolian, Mro, Multani, Nabataean, Nag Mundari, Nandinagari, Newa, New Tai Lue, Ogham, Ol Chiki, Old Hungarian, Old Italic, Old North Arabian, Old Permic, Old Persian, Old Sogdian, Old South Arabian, Old Turkic, Osage, Osmanya, Pahawh Hmong, Palmyrene, Pau Cin Hau, Phags Pa, Phoenician, Psalter Pahlavi, Rejang, Samaritan, Saurashtra, Sharada, Shavian, Siddham, SignWriting, Sogdian, Sora Sopeng, Soyombo, Sunandese, Syloti Nagri, Symbols, Symbols 2, Tagalog, Tagbanwa, Tai Le, Tai Tham, Tai Viet, Takri, Tamil Supplement, Tangsa, Thaana, Tifinagh, Tirhuta, Ugaritic, Vai, Wancho, Warang Citi, Yi, Zananbazar Square
- **Serif only**: Ahom, Dives Akuru, Dogra, Makasar, Nyiakeng Puachue Hmong, Old Uyghur, Ottoman Siyaq, Tibetan, Toto, Yezidi

Noto Sans Mono and Noto Sans/Serif Display are not supplied.

| script | default sans | default serif | other |
|:-|:-|:-|:-|
| LGC ('regular') | `Sans` | `Serif` | - |
| Adlam, N'ko | `Sans` | - | `Unjoined` |
| Arabic | `Sans` | `Naskh` | `Kufi`, `NaskhUi`, `Nastaliq` |
| Hebrew | `Sans` | `Serif` | `Rashi` |
| Khitan | - | `Serif` | `Rotated`, `Vertical` |
| Nushu | `Sans` | - | `Traditional` |
| Syriac | `Sans` | - | `Eastern`, `Western` |
| Thai, Lao | `SansUnlooped` | `Serif` | `SansLooped` |

Everything else (Armenian, Balinese, Bengali, Devanagari, Ethiopic, Georgian, Grantha, Gujarati, Gurmukhi, Kannada, Khmer, Khojki, Malayalam, Myanmar, Oriya, Sinhala, Tamil, Telugu, Vithkuqi) is only either `Sans` or `Serif`, accessible via the `Serifness` enum.

There are all-sans and all*-serif presets as `new_sans()` / `prefer_serif()`.