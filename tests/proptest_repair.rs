use proptest::prelude::*;
use patch_ts::ast::{RustLanguage, Language};
use patch_ts::repair::quick_balance;

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 500,
        .. ProptestConfig::default()
    })]

    #[test]
    fn quick_balance_produces_valid_ast(
        base_source in "fn main\\(\\) \\{ [a-zA-Z0-9_(); ]* \\}",
        extra_delimiters in prop::collection::vec(prop::char::range('(', ')'), 0..5)
    ) {
        // Build a source by inserting extra delimiters after the valid base
        let mut source = base_source.clone();
        for ch in extra_delimiters {
            source.push(ch);  // ch is now a char
        }
        let mut lang = RustLanguage::new();
        if let Some(fixed) = quick_balance(&source, &mut lang) {
            let parse = lang.parse(&fixed);
            prop_assert!(lang.is_valid(&parse), "Repair produced invalid AST: {}", fixed);
        }
    }
}
