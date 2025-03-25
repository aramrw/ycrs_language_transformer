use std::sync::Arc;
use fancy_regex::Regex;
use crate::transformer::{DeinflectFnType, Rule, RuleType, SuffixRule};

pub fn inflection(
    inflected: &str,
    deinflected: &'static str,
    conditions_in: &'static [&'static str],
    conditions_out: &'static [&'static str],
    rule_type: RuleType,
) -> Rule {
    let regx = match rule_type {
        RuleType::Prefix => format!("^{inflected}"),
        RuleType::Suffix => format!("{inflected}$"),
        RuleType::WholeWord => format!("^{inflected}$"),
        _ => panic!(
            "{rule_type:?} is invalid, only RuleType Suffix, Prefix && WholeWord work with this fn"
        ),
    };
    let deinflect_fn = match rule_type {
        RuleType::Suffix => DeinflectFnType::GenericSuffix,
        RuleType::Prefix => DeinflectFnType::GenericPrefix,
        RuleType::WholeWord => DeinflectFnType::GenericWholeWord,
        _ => panic!(
            "{rule_type:?} is invalid, only RuleType Suffix, Prefix && WholeWord work with this fn"
        ),
    };
    let is_inflected = Regex::new(&regx).unwrap();
    Rule {
        rule_type,
        is_inflected,
        deinflected: Some(deinflected),
        deinflect_fn,
        conditions_in,
        conditions_out,
    }
}
