use mewt::LanguageEngine;
use mewt::core::resolver::{LanguageResolver, ResolutionRequest};

use super::engine::TolkLanguageEngine;

pub struct TolkLanguageResolver {
    engine: TolkLanguageEngine,
}

impl TolkLanguageResolver {
    pub fn new() -> Self {
        Self {
            engine: TolkLanguageEngine::new(),
        }
    }

    fn is_language_name(raw: &str) -> bool {
        raw.eq_ignore_ascii_case("tolk")
    }
}

impl Default for TolkLanguageResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageResolver for TolkLanguageResolver {
    fn family(&self) -> &'static str {
        "tolk"
    }

    fn engines(&self) -> Vec<&dyn LanguageEngine> {
        vec![&self.engine]
    }

    fn resolve<'a>(
        &'a self,
        request: &ResolutionRequest<'_>,
    ) -> Option<Result<&'a dyn LanguageEngine, String>> {
        if let Some(explicit_language) = request.explicit_language {
            if !Self::is_language_name(explicit_language) {
                return None;
            }
            return Some(Ok(&self.engine));
        }

        request
            .path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case("tolk"))
            .then_some(Ok(&self.engine as &dyn LanguageEngine))
    }

    fn filter_labels(&self, query: &str) -> Option<Vec<String>> {
        Self::is_language_name(query).then(|| vec![self.engine.language().to_string()])
    }
}
