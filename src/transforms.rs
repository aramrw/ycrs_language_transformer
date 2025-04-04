use std::sync::Arc;

use regex::Regex;

use crate::transformer::{DeinflectFnType, Rule, RuleType, SuffixRule};

// Define the deinflect function
fn deinflect_fn(inflected_word: &str, inflected_suffix: &str, deinflected_suffix: &str) -> String {
    if inflected_word.ends_with(inflected_suffix) {
        let base = &inflected_word[..inflected_word.len() - inflected_suffix.len()];
        format!("{}{}", base, deinflected_suffix)
    } else {
        inflected_word.to_string()
    }
}

pub fn suffix_inflection(
    inflected_suffix: &str,
    deinflected_suffix: &'static str,
    conditions_in: &'static [&'static str],
    conditions_out: &'static [&'static str],
) -> SuffixRule {
    let reg = format!("{}$", inflected_suffix);
    let suffix_regex = Regex::new(&reg).unwrap();
    SuffixRule {
        deinflect_fn: DeinflectFnType::GenericSuffix,
        rule_type: RuleType::Suffix,
        is_inflected: suffix_regex,
        deinflected: deinflected_suffix,
        conditions_in,
        conditions_out,
    }
}

// pub fn inflection(
//     inflected: &str,
//     deinflected_prefix: &'static str,
//     conditions_in: &'static [&'static str],
//     conditions_out: &'static [&'static str],
//     rule_type: RuleType,
// ) -> Rule {
//     let regx = match rule_type {
//         RuleType::Prefix => format!("^{}", inflected),
//         RuleType::WholeWord => format!("^{}$", inflected),
//         _ => "".into(),
//     };
//     let is_inflected = Regex::new(&regx).unwrap();
//     Rule {
//         rule_type,
//         is_inflected,
//         deinflected: deinflected_prefix,
//         conditions_in,
//         conditions_out,
//     }
// }
