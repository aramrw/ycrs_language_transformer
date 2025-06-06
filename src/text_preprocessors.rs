use crate::{
    japanese::{
        collapse_emphatic_sequences, convert_alphanumeric_to_fullwidth,
        convert_fullwidth_alphanumeric_to_normal, convert_halfwidth_kana_to_fullwidth,
        convert_hiragana_to_katakana, convert_katakana_to_hiragana,
        normalize_cjk_compatibility_characters, normalize_combining_characters,
    },
    language_d::{
        BidirectionalConversionPreProcessor, BidirectionalPreProcessorOptions, TextProcessor,
    },
    text_processors::BASIC_TEXT_PROCESSOR_OPTIONS,
    wanakana::convert_alphabetic_to_kana,
};

use kanji_processor::convert_variants;

fn convert_half_width_characters_helper(text: &str, setting: bool) -> String {
    if setting {
        return convert_halfwidth_kana_to_fullwidth(text);
    }
    text.to_owned()
}

pub const CONVERT_HALF_WIDTH_CHARACTERS: TextProcessor<bool, bool> = TextProcessor {
    name: "Convert Half Width Characters to Full Width",
    description: "ﾖﾐﾁｬﾝ → ヨミチャン",
    options: &BASIC_TEXT_PROCESSOR_OPTIONS,
    process: convert_half_width_characters_helper,
};

pub fn alphabetic_to_hiragana_helper(text: &str, setting: bool) -> String {
    if setting {
        return convert_alphabetic_to_kana(text);
    }
    text.to_owned()
}

pub const ALPHABETIC_TO_HIRAGANA: TextProcessor<bool, bool> = TextProcessor {
    name: "Convert Alphabetic Characters to Hiragana",
    description: "yomichan → よみちゃん",
    options: &BASIC_TEXT_PROCESSOR_OPTIONS,
    process: alphabetic_to_hiragana_helper,
};

fn process_alphanumeric_width_variants(
    str: &str,
    setting: BidirectionalPreProcessorOptions,
) -> String {
    match setting {
        BidirectionalPreProcessorOptions::Off => str.to_string(),
        BidirectionalPreProcessorOptions::Direct => convert_fullwidth_alphanumeric_to_normal(str),
        BidirectionalPreProcessorOptions::Inverse => convert_alphanumeric_to_fullwidth(str),
    }
}

pub const ALPHANUMERIC_WIDTH_VARIANTS: BidirectionalConversionPreProcessor =
    BidirectionalConversionPreProcessor {
        name: "Convert Between Alphabetic Width Variants",
        description: "ｙｏｍｉｔａｎ → yomitan and vice versa",
        options: &[
            BidirectionalPreProcessorOptions::Off,
            BidirectionalPreProcessorOptions::Direct,
            BidirectionalPreProcessorOptions::Inverse,
        ],
        process: process_alphanumeric_width_variants,
    };

fn process_hiragana_to_katakana(str: &str, setting: BidirectionalPreProcessorOptions) -> String {
    match setting {
        BidirectionalPreProcessorOptions::Off => str.to_string(),
        BidirectionalPreProcessorOptions::Direct => convert_hiragana_to_katakana(str),
        BidirectionalPreProcessorOptions::Inverse => convert_katakana_to_hiragana(str, false),
    }
}

pub const CONVERT_HIRAGANA_TO_KATAKANA: BidirectionalConversionPreProcessor =
    BidirectionalConversionPreProcessor {
        name: "Convert Hiragana to Katakana",
        description: "よみちゃん → ヨミチャン and vice versa",
        options: &[
            BidirectionalPreProcessorOptions::Off,
            BidirectionalPreProcessorOptions::Direct,
            BidirectionalPreProcessorOptions::Inverse,
        ],
        process: process_hiragana_to_katakana,
    };

fn collapse_emphatic_sequences_helper(text: &str, setting: [bool; 2]) -> String {
    let text = text.to_owned();
    let [collapse_emphatic, collapse_emphatic_full] = setting;
    if collapse_emphatic {
        collapse_emphatic_sequences(&text, collapse_emphatic_full)
    } else {
        text
    }
}

pub const COLLAPSE_EMPHATIC_SEQUENCES: TextProcessor<[bool; 2], [bool; 2]> = TextProcessor {
    name: "Collapse Emphatic Character Sequences",
    description: "すっっごーーい → すっごーい / すごい",
    options: &[[false, false], [true, false], [true, true]],
    process: collapse_emphatic_sequences_helper,
};

fn normalize_combining_characters_helper(text: &str, setting: bool) -> String {
    if setting {
        return normalize_combining_characters(text);
    }
    text.to_owned()
}

pub const NORMALIZE_COMBINING_CHARACTERS: TextProcessor<bool, bool> = TextProcessor {
    name: "Normalize Combining Characters",
    description: "ド → ド (U+30C8 U+3099 → U+30C9)",
    options: &BASIC_TEXT_PROCESSOR_OPTIONS,
    process: normalize_combining_characters_helper,
};

fn normalize_cjk_compatibility_characters_helper(text: &str, setting: bool) -> String {
    if setting {
        // Assuming you have this function in your japanese.rs now
        return normalize_cjk_compatibility_characters(text);
    }
    text.to_owned()
}

pub const NORMALIZE_CJK_COMPATIBILITY_CHARACTERS: TextProcessor<bool, bool> = TextProcessor {
    name: "Normalize CJK Compatibility Characters",
    description: "㌀ → アパート",
    options: &BASIC_TEXT_PROCESSOR_OPTIONS,
    process: normalize_cjk_compatibility_characters_helper,
};

fn standardize_kanji_helper(text: &str, setting: bool) -> String {
    if setting {
        return convert_variants(text);
    }
    text.to_owned()
}

pub const STANDARDIZE_KANJI: TextProcessor<bool, bool> = TextProcessor {
    name: "Convert kanji variants to their modern standard form",
    description: "萬 → 万",
    options: &BASIC_TEXT_PROCESSOR_OPTIONS,
    process: standardize_kanji_helper,
};

// You might also need NORMALIZE_RADICAL_CHARACTERS if you intend to keep it,
// but it's not in the JS provided. If you want strict JS parity, remove it
// from descriptors.rs. If you need it, you'll have to define it here.
