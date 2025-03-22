use bitflags::bitflags;

/// An enum to help the parser decide how to interpret a markdown document
pub enum MarkdownSpecification {
    /// The core markdown specification, with a boolean value to enable extended syntax
    Core(bool),

    /// The [CommonMark](https://commonmark.org/) specification
    CommonMark,

    /// The [GitHub](https://github.github.com/gfm/) specification
    GitHub,

    /// The [MarkdownExtra](https://michelf.ca/projects/php-markdown/extra/) specification
    MarkdownExtra,

    /// The [MultiMarkdown](https://fletcherpenney.net/multimarkdown/) specification
    MultiMarkdown,

    /// The [R Markdown](https://rmarkdown.rstudio.com/) specification
    RMarkdown,
}

bitflags! {
    pub struct MarkdownFeatureSet: u64{
        const SPEC = 0b00001111_11111111;
    }
}

bitflags! {
    pub struct MarkdownFeatures: u64{
        // CORE FEATURES
        const HEADINGS         = 0b00000000_00000001; //  1
        const PARAGRAPH        = 0b00000000_00000010; //  2
        const LINE_BREAKS      = 0b00000000_00000100; //  4
        const BOLD             = 0b00000000_00001000; //  8
        const ITALIC           = 0b00000000_00010000; // 16
        const BLOCK_QUOTES     = 0b00000000_00100000; // 32
        const LISTS            = 0b00000000_01000000; // 64
        const CODE             = 0b00000000_10000000; // 128
        const IMAGE            = 0b00000001_00000000; // 256
        const HORIZONTAL_RULES = 0b00000010_00000000; // 512
        const LINKS            = 0b00000100_00000000; // 1024
        const HTML             = 0b00001000_00000000; // 2048 - currently unsupported

    }
}

mod core;