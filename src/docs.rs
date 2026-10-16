//! # 문서 모음
//!
//! 조사·조직화·보존·공통의 중심 안내 문서에서 상세 문서로 이어 읽는다.
//! Markdown 원본은 `docs/`에 있으며 GitHub와 rustdoc에서 같은 본문을 사용한다.

#[doc = concat!(
    "[formats]: crate::docs::survey::formats\n",
    "[commercial]: crate::docs::survey::formats::commercial\n",
    "[units]: crate::docs::survey::formats::chart_units\n",
    "[grammar]: crate::docs::survey::bms_grammar\n",
    "[parsers]: crate::docs::survey::bms_parsers\n",
    "[references]: crate::docs::common::references\n",
    "[checks]: crate::docs::survey::formats::format_checks\n",
    "\n",
    concat!("[project-guide]: crate\n\n", include_str!("../docs/survey/index.md"))
)]
pub mod survey {
    #[doc = concat!(
        "[nav-1]: crate::docs::cataloging::schema\n",
        "[nav-2]: crate::docs::survey::bms_grammar\n",
        "[nav-3]: crate::docs::survey::bms_parsers\n",
        "[nav-4]: crate::docs::survey::formats::commercial\n",
        "[nav-5]: crate::docs::survey::bms_grammar\n",
        "[nav-6]: crate::docs::survey::formats::chart_units\n",
        "[nav-7]: crate::docs::survey::formats::commercial\n",
        "[nav-8]: crate::docs::survey::formats::format_checks\n",
        "\n",
        concat!("[project-guide]: crate\n\n", include_str!("../docs/survey/formats.md"))
    )]
    pub mod formats {
        #[doc = concat!(
            "[nav-1]: crate::docs::survey::bms_grammar\n",
            "[nav-2]: crate::docs::cataloging::schema\n",
            "[nav-3]: crate::docs::survey::formats\n",
            "[checks]: crate::docs::survey::formats::format_checks\n",
            "\n",
            concat!("[project-guide]: crate\n\n", include_str!("../docs/survey/formats/chart-units.md"))
        )]
        pub mod chart_units {}

        #[doc = concat!(
            "[nav-1]: crate::docs::survey::formats\n",
            "[units]: crate::docs::survey::formats::chart_units\n",
            "[checks]: crate::docs::survey::formats::format_checks\n",
            "\n",
            concat!("[project-guide]: crate\n\n", include_str!("../docs/survey/formats/commercial.md"))
        )]
        pub mod commercial {}

        #[doc = concat!("[units]: crate::docs::survey::formats::chart_units\n[project-guide]: crate\n\n", include_str!("../docs/survey/formats/format-checks.md"))]
        pub mod format_checks {}
    }

    #[doc = concat!(
        "[nav-1]: crate::docs::survey::formats\n",
        "[nav-2]: crate::docs::cataloging::schema\n",
        "[nav-3]: crate::docs::common::references::bms\n",
        "[parsers]: crate::docs::survey::bms_parsers\n",
        "[abnf-source]: https://github.com/N-ISOGE/chicken_manu/blob/dev/docs/survey/bms.abnf\n",
        "[ebnf-source]: https://github.com/N-ISOGE/chicken_manu/blob/dev/docs/survey/bms.ebnf\n",
        "[grammar-check]: https://github.com/N-ISOGE/chicken_manu/blob/dev/tools/check_bms_grammars.py\n",
        "\n",
        concat!("[project-guide]: crate\n\n", include_str!("../docs/survey/bms-grammar.md"))
    )]
    #[doc = concat!("## ABNF 원문\n\n```abnf\n", include_str!("../docs/survey/bms.abnf"), "\n```") ]
    pub mod bms_grammar {}

    #[doc = concat!("[grammar]: crate::docs::survey::bms_grammar\n[project-guide]: crate\n\n", include_str!("../docs/survey/bms-parsers.md"))]
    pub mod bms_parsers {}
}

#[doc = concat!(
    "[guide]: crate#4-매체와-관련-자료를-어떻게-기록하고-활용하는가\n",
    "[units]: crate::docs::survey::formats::chart_units\n",
    "[schema]: crate::docs::cataloging::schema\n",
    "\n",
    concat!("[project-guide]: crate\n\n", include_str!("../docs/cataloging/index.md"))
)]
pub mod cataloging {
    #[doc = concat!(
        "[nav-1]: crate::docs::survey::formats::chart_units\n",
        "[nav-2]: crate::docs::survey::bms_grammar\n",
        "[nav-3]: crate::docs::preservation::adoption\n",
        "[nav-4]: crate::docs::common::references::preservation\n",
        "\n",
        concat!("[project-guide]: crate\n\n", include_str!("../docs/cataloging/schema.md"))
    )]
    pub mod schema {}
}

#[doc = concat!(
    "[schema]: crate::docs::cataloging::schema\n",
    "[adoption]: crate::docs::preservation::adoption\n",
    "[references]: crate::docs::common::references::preservation\n",
    "\n",
    concat!("[project-guide]: crate\n\n", include_str!("../docs/preservation/index.md"))
)]
pub mod preservation {
    #[doc = concat!(
        "[nav-1]: crate::docs::common::references\n",
        "[nav-2]: crate::docs::common::references::preservation\n",
        "[nav-3]: crate::docs::common::references::preservation\n",
        "[nav-4]: crate::docs::cataloging::schema\n",
        "\n",
        concat!("[project-guide]: crate\n\n", include_str!("../docs/preservation/adoption.md"))
    )]
    pub mod adoption {}
}

#[doc = concat!(
    "[guide]: crate\n",
    "[references]: crate::docs::common::references\n",
    "[notes]: crate::docs::common::notes\n",
    "\n",
    concat!("[project-guide]: crate\n\n", include_str!("../docs/common/index.md"))
)]
pub mod common {
    #[doc = concat!(
        "[nav-1]: crate::docs::common#문서-읽는-법\n",
        "[nav-2]: crate::docs::preservation::adoption\n",
        "[nav-3]: crate::docs::common::references::bms\n",
        "[nav-4]: crate::docs::common::references::standards\n",
        "[nav-5]: crate::docs::common::references::charts\n",
        "[nav-6]: crate::docs::common::references::preservation\n",
        "\n",
        concat!("[project-guide]: crate\n\n", include_str!("../docs/common/references.md"))
    )]
    pub mod references {
        #[doc = concat!(
            "[nav-1]: crate::docs::common#문서-읽는-법\n",
            "\n",
            concat!("[project-guide]: crate\n\n", include_str!("../docs/common/references/bms.md"))
        )]
        pub mod bms {}

        #[doc = concat!(
            "[nav-1]: crate::docs::common#문서-읽는-법\n",
            "[nav-2]: crate::docs::preservation::adoption\n",
            "\n",
            concat!("[project-guide]: crate\n\n", include_str!("../docs/common/references/standards.md"))
        )]
        pub mod standards {}

        #[doc = concat!("[project-guide]: crate\n\n", include_str!("../docs/common/references/charts.md"))]
        pub mod charts {}

        #[doc = concat!("[project-guide]: crate\n\n", include_str!("../docs/common/references/preservation.md"))]
        pub mod preservation {}
    }

    #[doc = concat!(
        "[nav-1]: crate::docs::common::changelog\n",
        "[nav-2]: crate::docs::common::references\n",
        "[nav-3]: crate::docs::survey::bms_grammar\n",
        "[nav-5]: crate::docs::cataloging::schema\n",
        "\n",
        concat!("[project-guide]: crate\n\n", include_str!("../docs/common/notes.md"))
    )]
    pub mod notes {}

    #[doc = include_str!("../change_log.md")]
    pub mod changelog {}
}
