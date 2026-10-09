//! # 문서 모음
//!
//! `docs/` 디렉터리의 마크다운 문서를 rustdoc에 싣는 모듈이다.
//! 문서는 `cargo doc --no-deps --open`으로 볼 수 있다.
//!
//! 마크다운 원본은 `docs/`에 있고, 이 파일은 `include_str!`로 가져오기만 한다.
//! 코드 블록에는 항상 언어(`text`, `abnf`, `sql` 등)를 적는다.
//! 언어를 생략하면 rustdoc이 Rust 코드로 보고 시험하려 한다.

#[doc = include_str!("../change_log.md")]
pub mod changelog {}

#[doc = include_str!("../docs/notes.md")]
pub mod notes {}

#[doc = include_str!("../docs/references.md")]
pub mod references {
    #[doc = include_str!("../docs/references/bms.md")]
    pub mod bms {}

    #[doc = include_str!("../docs/references/standards.md")]
    pub mod standards {}

    #[doc = include_str!("../docs/references/charts.md")]
    pub mod charts {}

    #[doc = include_str!("../docs/references/preservation.md")]
    pub mod preservation {}
}

#[doc = include_str!("../docs/adoption.md")]
pub mod adoption {}

#[doc = include_str!("../docs/schema.md")]
pub mod schema {}

#[doc = include_str!("../docs/formats.md")]
pub mod formats {
    #[doc = include_str!("../docs/formats/chart-units.md")]
    pub mod chart_units {}

    #[doc = include_str!("../docs/formats/commercial.md")]
    pub mod commercial {}

    #[doc = include_str!("../docs/formats/format-checks.md")]
    pub mod format_checks {}
}

#[doc = include_str!("../docs/bms-grammar.md")]
#[doc = concat!("```abnf\n", include_str!("../docs/bms.abnf"), "\n```")]
pub mod bms_grammar {}
