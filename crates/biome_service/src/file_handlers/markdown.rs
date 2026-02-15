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
        _languages: &crate::settings::LanguageListSettings,
    ) -> &crate::settings::LanguageSettings<Self> {
        todo!()
    }

    fn resolve_environment(
        _settings: &crate::settings::Settings,
    ) -> Option<&Self::EnvironmentSettings> {
        todo!()
    }

    fn resolve_parse_options(
        _overrides: &crate::settings::OverrideSettings,
        _language: &Self::ParserSettings,
        _path: &biome_fs::BiomePath,
        _file_source: &super::DocumentFileSource,
    ) -> Self::ParserOptions {
        todo!()
    }

    fn resolve_format_options(
        _global: &crate::settings::FormatSettings,
        _overrides: &crate::settings::OverrideSettings,
        _language: &Self::FormatterSettings,
        _path: &biome_fs::BiomePath,
        _file_source: &super::DocumentFileSource,
    ) -> Self::FormatOptions {
        todo!()
    }

    fn resolve_analyzer_options(
        _global: &crate::settings::Settings,
        _language: &Self::LinterSettings,
        _environment: Option<&Self::EnvironmentSettings>,
        _path: &biome_fs::BiomePath,
        _file_source: &super::DocumentFileSource,
        _suppression_reason: Option<&str>,
    ) -> biome_analyze::AnalyzerOptions {
        todo!()
    }

    fn linter_enabled_for_file_path(
        _settings: &crate::settings::Settings,
        _path: &camino::Utf8Path,
    ) -> bool {
        todo!()
    }

    fn formatter_enabled_for_file_path(
        _settings: &crate::settings::Settings,
        _path: &camino::Utf8Path,
    ) -> bool {
        todo!()
    }

    fn assist_enabled_for_file_path(
        _settings: &crate::settings::Settings,
        _path: &camino::Utf8Path,
    ) -> bool {
        todo!()
    }
}
