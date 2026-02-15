use std::fmt;

use biome_markdown_formatter::context::MarkdownFormatOptions;
use biome_markdown_syntax::MarkdownLanguage;

use crate::settings::ServiceLanguage;

#[derive(Default, Clone, Debug)]
pub struct MarkdownFormatterSettings;

#[derive(Default, Clone, Debug)]
pub struct MarkdownLinterSettings;

#[derive(Default, Clone, Debug)]
pub struct MarkdownAssistSettings;

// #[derive(Default, Clone)]
// pub struct MarkdownFormatOptions;

#[derive(Default, Clone, Debug)]
pub struct MarkdownParserSettings;

#[derive(Default)]
pub struct MarkdownParserOptions;

#[derive(Default, Clone, Debug)]
pub struct MarkdownEnvironmentSettings;

impl ServiceLanguage for MarkdownLanguage {
    type FormatterSettings = MarkdownFormatterSettings;

    type LinterSettings = MarkdownLinterSettings;

    type AssistSettings = MarkdownAssistSettings;

    type FormatOptions = MarkdownFormatOptions;

    type ParserSettings = MarkdownParserSettings;

    type ParserOptions = MarkdownParserOptions;

    type EnvironmentSettings = MarkdownEnvironmentSettings;

    fn lookup_settings(
        languages: &crate::settings::LanguageListSettings,
    ) -> &crate::settings::LanguageSettings<Self> {
        todo!()
    }

    fn resolve_environment(
        settings: &crate::settings::Settings,
    ) -> Option<&Self::EnvironmentSettings> {
        todo!()
    }

    fn resolve_parse_options(
        overrides: &crate::settings::OverrideSettings,
        language: &Self::ParserSettings,
        path: &biome_fs::BiomePath,
        file_source: &super::DocumentFileSource,
    ) -> Self::ParserOptions {
        todo!()
    }

    fn resolve_format_options(
        global: &crate::settings::FormatSettings,
        overrides: &crate::settings::OverrideSettings,
        language: &Self::FormatterSettings,
        path: &biome_fs::BiomePath,
        file_source: &super::DocumentFileSource,
    ) -> Self::FormatOptions {
        todo!()
    }

    fn resolve_analyzer_options(
        global: &crate::settings::Settings,
        language: &Self::LinterSettings,
        environment: Option<&Self::EnvironmentSettings>,
        path: &biome_fs::BiomePath,
        file_source: &super::DocumentFileSource,
        suppression_reason: Option<&str>,
    ) -> biome_analyze::AnalyzerOptions {
        todo!()
    }

    fn linter_enabled_for_file_path(
        settings: &crate::settings::Settings,
        path: &camino::Utf8Path,
    ) -> bool {
        todo!()
    }

    fn formatter_enabled_for_file_path(
        settings: &crate::settings::Settings,
        path: &camino::Utf8Path,
    ) -> bool {
        todo!()
    }

    fn assist_enabled_for_file_path(
        settings: &crate::settings::Settings,
        path: &camino::Utf8Path,
    ) -> bool {
        todo!()
    }
}
