use magnus::{function, method, prelude::*, Error, Ruby, Value};
use std::cell::RefCell;
use std::str::FromStr;

use lingua::{
    DetectionResult, IsoCode639_1, IsoCode639_3, Language, LanguageDetector,
    LanguageDetectorBuilder,
};

#[magnus::wrap(class = "LinguaRsRb::LanguageDetector")]
struct LanguageDetectorWrapper(LanguageDetector);

#[magnus::wrap(class = "LinguaRsRb::LanguageDetectorBuilder")]
struct LanguageDetectorBuilderWrapper(RefCell<Option<LanguageDetectorBuilder>>);

impl LanguageDetectorBuilderWrapper {
    fn new(builder: LanguageDetectorBuilder) -> Self {
        Self(RefCell::new(Some(builder)))
    }

    fn with_minimum_relative_distance(
        ruby: &Ruby,
        rb_self: &Self,
        distance: f64,
    ) -> Result<Value, Error> {
        if !(0.0..=1.0).contains(&distance) {
            return Err(Error::new(
                ruby.exception_arg_error(),
                "minimum relative distance must be between 0.0 and 1.0",
            ));
        }

        let mut builder = take_builder(ruby, rb_self)?;
        builder.with_minimum_relative_distance(distance);
        *rb_self.0.borrow_mut() = Some(builder);
        magnus::current_receiver::<Value>()
    }

    fn with_preloaded_language_models(
        ruby: &Ruby,
        rb_self: &Self,
    ) -> Result<Value, Error> {
        let mut builder = take_builder(ruby, rb_self)?;
        builder.with_preloaded_language_models();
        *rb_self.0.borrow_mut() = Some(builder);
        magnus::current_receiver::<Value>()
    }

    fn with_low_accuracy_mode(ruby: &Ruby, rb_self: &Self) -> Result<Value, Error> {
        let mut builder = take_builder(ruby, rb_self)?;
        builder.with_low_accuracy_mode();
        *rb_self.0.borrow_mut() = Some(builder);
        magnus::current_receiver::<Value>()
    }

    fn build(ruby: &Ruby, rb_self: &Self) -> Result<LanguageDetectorWrapper, Error> {
        let mut builder = take_builder(ruby, rb_self)?;
        Ok(LanguageDetectorWrapper(builder.build()))
    }
}

impl LanguageDetectorWrapper {
    fn unload_language_models(&self) {
        self.0.unload_language_models();
    }

    fn detect_language(&self, text: String) -> Option<String> {
        self.0.detect_language_of(text).map(|lang| lang.to_string())
    }

    fn detect_languages_in_parallel(&self, texts: Vec<String>) -> Vec<Option<String>> {
        self.0
            .detect_languages_in_parallel_of(&texts)
            .into_iter()
            .map(|lang| lang.map(|l| l.to_string()))
            .collect()
    }

    fn detect_multiple_languages(&self, text: String) -> Vec<(String, usize, usize)> {
        self.0
            .detect_multiple_languages_of(text)
            .into_iter()
            .map(detection_result_to_tuple)
            .collect()
    }

    fn detect_multiple_languages_in_parallel(
        &self,
        texts: Vec<String>,
    ) -> Vec<Vec<(String, usize, usize)>> {
        self.0
            .detect_multiple_languages_in_parallel_of(&texts)
            .into_iter()
            .map(|results| results.into_iter().map(detection_result_to_tuple).collect())
            .collect()
    }

    fn compute_language_confidence_values(&self, text: String) -> Vec<(String, f64)> {
        self.0
            .compute_language_confidence_values(text)
            .into_iter()
            .map(|(language, confidence)| (language.to_string(), confidence))
            .collect()
    }

    fn compute_language_confidence_values_in_parallel(
        &self,
        texts: Vec<String>,
    ) -> Vec<Vec<(String, f64)>> {
        self.0
            .compute_language_confidence_values_in_parallel(&texts)
            .into_iter()
            .map(|values| {
                values
                    .into_iter()
                    .map(|(language, confidence)| (language.to_string(), confidence))
                    .collect()
            })
            .collect()
    }

    fn compute_language_confidence(
        ruby: &Ruby,
        rb_self: &Self,
        text: String,
        language_value: Value,
    ) -> Result<f64, Error> {
        let language = parse_language_value(ruby, language_value)?;
        Ok(rb_self.0.compute_language_confidence(text, language))
    }

    fn compute_language_confidence_in_parallel(
        ruby: &Ruby,
        rb_self: &Self,
        texts: Vec<String>,
        language_value: Value,
    ) -> Result<Vec<f64>, Error> {
        let language = parse_language_value(ruby, language_value)?;
        Ok(rb_self
            .0
            .compute_language_confidence_in_parallel(&texts, language))
    }
}

fn detection_result_to_tuple(result: DetectionResult) -> (String, usize, usize) {
    (result.language().to_string(), result.start_index(), result.end_index())
}

fn take_builder(
    ruby: &Ruby,
    wrapper: &LanguageDetectorBuilderWrapper,
) -> Result<LanguageDetectorBuilder, Error> {
    wrapper.0.borrow_mut().take().ok_or_else(|| {
        Error::new(
            ruby.exception_runtime_error(),
            "language detector builder has already been consumed",
        )
    })
}

fn parse_language_value(ruby: &Ruby, value: Value) -> Result<Language, Error> {
    let name: String = value.funcall("to_s", ())?;
    Language::from_str(&name).map_err(|_| {
        Error::new(
            ruby.exception_arg_error(),
            format!("unknown language: {name}"),
        )
    })
}

fn parse_languages(ruby: &Ruby, values: Vec<String>) -> Result<Vec<Language>, Error> {
    if values.is_empty() {
        return Err(Error::new(
            ruby.exception_arg_error(),
            "languages list must not be empty",
        ));
    }

    values
        .into_iter()
        .map(|value| {
            Language::from_str(&value).map_err(|_| {
                Error::new(
                    ruby.exception_arg_error(),
                    format!("unknown language: {value}"),
                )
            })
        })
        .collect()
}

fn parse_iso_codes_639_1(
    ruby: &Ruby,
    values: Vec<String>,
) -> Result<Vec<IsoCode639_1>, Error> {
    if values.is_empty() {
        return Err(Error::new(
            ruby.exception_arg_error(),
            "ISO 639-1 codes list must not be empty",
        ));
    }

    values
        .into_iter()
        .map(|value| {
            IsoCode639_1::from_str(&value).map_err(|_| {
                Error::new(
                    ruby.exception_arg_error(),
                    format!("unknown ISO 639-1 code: {value}"),
                )
            })
        })
        .collect()
}

fn parse_iso_codes_639_3(
    ruby: &Ruby,
    values: Vec<String>,
) -> Result<Vec<IsoCode639_3>, Error> {
    if values.is_empty() {
        return Err(Error::new(
            ruby.exception_arg_error(),
            "ISO 639-3 codes list must not be empty",
        ));
    }

    values
        .into_iter()
        .map(|value| {
            IsoCode639_3::from_str(&value).map_err(|_| {
                Error::new(
                    ruby.exception_arg_error(),
                    format!("unknown ISO 639-3 code: {value}"),
                )
            })
        })
        .collect()
}

fn builder_from_all_languages() -> LanguageDetectorBuilderWrapper {
    LanguageDetectorBuilderWrapper::new(LanguageDetectorBuilder::from_all_languages())
}

fn builder_from_all_spoken_languages() -> LanguageDetectorBuilderWrapper {
    LanguageDetectorBuilderWrapper::new(LanguageDetectorBuilder::from_all_spoken_languages())
}

fn builder_from_all_languages_with_arabic_script() -> LanguageDetectorBuilderWrapper {
    LanguageDetectorBuilderWrapper::new(
        LanguageDetectorBuilder::from_all_languages_with_arabic_script(),
    )
}

fn builder_from_all_languages_with_cyrillic_script() -> LanguageDetectorBuilderWrapper {
    LanguageDetectorBuilderWrapper::new(
        LanguageDetectorBuilder::from_all_languages_with_cyrillic_script(),
    )
}

fn builder_from_all_languages_with_devanagari_script() -> LanguageDetectorBuilderWrapper {
    LanguageDetectorBuilderWrapper::new(
        LanguageDetectorBuilder::from_all_languages_with_devanagari_script(),
    )
}

fn builder_from_all_languages_with_latin_script() -> LanguageDetectorBuilderWrapper {
    LanguageDetectorBuilderWrapper::new(
        LanguageDetectorBuilder::from_all_languages_with_latin_script(),
    )
}

fn builder_from_all_languages_with_single_unique_script() -> LanguageDetectorBuilderWrapper {
    let languages: Vec<Language> = Language::all_with_single_unique_script().into_iter().collect();
    LanguageDetectorBuilderWrapper::new(LanguageDetectorBuilder::from_languages(&languages))
}

fn builder_from_languages(
    ruby: &Ruby,
    languages: Vec<String>,
) -> Result<LanguageDetectorBuilderWrapper, Error> {
    let languages = parse_languages(ruby, languages)?;
    Ok(LanguageDetectorBuilderWrapper::new(
        LanguageDetectorBuilder::from_languages(&languages),
    ))
}

fn builder_from_all_languages_without(
    ruby: &Ruby,
    languages: Vec<String>,
) -> Result<LanguageDetectorBuilderWrapper, Error> {
    let languages = parse_languages(ruby, languages)?;
    Ok(LanguageDetectorBuilderWrapper::new(
        LanguageDetectorBuilder::from_all_languages_without(&languages),
    ))
}

fn builder_from_iso_codes_639_1(
    ruby: &Ruby,
    iso_codes: Vec<String>,
) -> Result<LanguageDetectorBuilderWrapper, Error> {
    let iso_codes = parse_iso_codes_639_1(ruby, iso_codes)?;
    Ok(LanguageDetectorBuilderWrapper::new(
        LanguageDetectorBuilder::from_iso_codes_639_1(&iso_codes),
    ))
}

fn builder_from_iso_codes_639_3(
    ruby: &Ruby,
    iso_codes: Vec<String>,
) -> Result<LanguageDetectorBuilderWrapper, Error> {
    let iso_codes = parse_iso_codes_639_3(ruby, iso_codes)?;
    Ok(LanguageDetectorBuilderWrapper::new(
        LanguageDetectorBuilder::from_iso_codes_639_3(&iso_codes),
    ))
}

fn languages() -> Vec<String> {
    let mut langs: Vec<String> = Language::all().into_iter().map(|l| l.to_string()).collect();
    langs.sort();
    langs
}

fn spoken_languages() -> Vec<String> {
    let mut langs: Vec<String> = Language::all_spoken_ones()
        .into_iter()
        .map(|l| l.to_string())
        .collect();
    langs.sort();
    langs
}

fn languages_with_arabic_script() -> Vec<String> {
    let mut langs: Vec<String> = Language::all_with_arabic_script()
        .into_iter()
        .map(|l| l.to_string())
        .collect();
    langs.sort();
    langs
}

fn languages_with_cyrillic_script() -> Vec<String> {
    let mut langs: Vec<String> = Language::all_with_cyrillic_script()
        .into_iter()
        .map(|l| l.to_string())
        .collect();
    langs.sort();
    langs
}

fn languages_with_devanagari_script() -> Vec<String> {
    let mut langs: Vec<String> = Language::all_with_devanagari_script()
        .into_iter()
        .map(|l| l.to_string())
        .collect();
    langs.sort();
    langs
}

fn languages_with_latin_script() -> Vec<String> {
    let mut langs: Vec<String> = Language::all_with_latin_script()
        .into_iter()
        .map(|l| l.to_string())
        .collect();
    langs.sort();
    langs
}

fn languages_with_single_unique_script() -> Vec<String> {
    let mut langs: Vec<String> = Language::all_with_single_unique_script()
        .into_iter()
        .map(|l| l.to_string())
        .collect();
    langs.sort();
    langs
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("LinguaRsRb")?;

    module.define_singleton_method("languages", function!(languages, 0))?;
    module.define_singleton_method("spoken_languages", function!(spoken_languages, 0))?;
    module.define_singleton_method(
        "languages_with_arabic_script",
        function!(languages_with_arabic_script, 0),
    )?;
    module.define_singleton_method(
        "languages_with_cyrillic_script",
        function!(languages_with_cyrillic_script, 0),
    )?;
    module.define_singleton_method(
        "languages_with_devanagari_script",
        function!(languages_with_devanagari_script, 0),
    )?;
    module.define_singleton_method(
        "languages_with_latin_script",
        function!(languages_with_latin_script, 0),
    )?;
    module.define_singleton_method(
        "languages_with_single_unique_script",
        function!(languages_with_single_unique_script, 0),
    )?;

    let builder_class = module.define_class("LanguageDetectorBuilder", ruby.class_object())?;
    builder_class.define_singleton_method("from_all_languages", function!(builder_from_all_languages, 0))?;
    builder_class.define_singleton_method(
        "from_all_spoken_languages",
        function!(builder_from_all_spoken_languages, 0),
    )?;
    builder_class.define_singleton_method(
        "from_all_languages_with_arabic_script",
        function!(builder_from_all_languages_with_arabic_script, 0),
    )?;
    builder_class.define_singleton_method(
        "from_all_languages_with_cyrillic_script",
        function!(builder_from_all_languages_with_cyrillic_script, 0),
    )?;
    builder_class.define_singleton_method(
        "from_all_languages_with_devanagari_script",
        function!(builder_from_all_languages_with_devanagari_script, 0),
    )?;
    builder_class.define_singleton_method(
        "from_all_languages_with_latin_script",
        function!(builder_from_all_languages_with_latin_script, 0),
    )?;
    builder_class.define_singleton_method(
        "from_all_languages_with_single_unique_script",
        function!(builder_from_all_languages_with_single_unique_script, 0),
    )?;
    builder_class.define_singleton_method("from_languages", function!(builder_from_languages, 1))?;
    builder_class.define_singleton_method(
        "from_all_languages_without",
        function!(builder_from_all_languages_without, 1),
    )?;
    builder_class.define_singleton_method(
        "from_iso_codes_639_1",
        function!(builder_from_iso_codes_639_1, 1),
    )?;
    builder_class.define_singleton_method(
        "from_iso_codes_639_3",
        function!(builder_from_iso_codes_639_3, 1),
    )?;
    builder_class.define_method(
        "with_minimum_relative_distance",
        method!(LanguageDetectorBuilderWrapper::with_minimum_relative_distance, 1),
    )?;
    builder_class.define_method(
        "with_preloaded_language_models",
        method!(LanguageDetectorBuilderWrapper::with_preloaded_language_models, 0),
    )?;
    builder_class.define_method(
        "with_low_accuracy_mode",
        method!(LanguageDetectorBuilderWrapper::with_low_accuracy_mode, 0),
    )?;
    builder_class.define_method("build", method!(LanguageDetectorBuilderWrapper::build, 0))?;

    let detector_class = module.define_class("LanguageDetector", ruby.class_object())?;
    detector_class.define_method(
        "unload_language_models",
        method!(LanguageDetectorWrapper::unload_language_models, 0),
    )?;
    detector_class.define_method(
        "detect_language",
        method!(LanguageDetectorWrapper::detect_language, 1),
    )?;
    detector_class.define_method(
        "detect_languages_in_parallel",
        method!(LanguageDetectorWrapper::detect_languages_in_parallel, 1),
    )?;
    detector_class.define_method(
        "detect_multiple_languages",
        method!(LanguageDetectorWrapper::detect_multiple_languages, 1),
    )?;
    detector_class.define_method(
        "detect_multiple_languages_in_parallel",
        method!(LanguageDetectorWrapper::detect_multiple_languages_in_parallel, 1),
    )?;
    detector_class.define_method(
        "compute_language_confidence_values",
        method!(LanguageDetectorWrapper::compute_language_confidence_values, 1),
    )?;
    detector_class.define_method(
        "compute_language_confidence_values_in_parallel",
        method!(LanguageDetectorWrapper::compute_language_confidence_values_in_parallel, 1),
    )?;
    detector_class.define_method(
        "compute_language_confidence",
        method!(LanguageDetectorWrapper::compute_language_confidence, 2),
    )?;
    detector_class.define_method(
        "compute_language_confidence_in_parallel",
        method!(LanguageDetectorWrapper::compute_language_confidence_in_parallel, 2),
    )?;

    Ok(())
}
