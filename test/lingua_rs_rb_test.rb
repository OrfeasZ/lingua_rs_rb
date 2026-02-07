# frozen_string_literal: true

require_relative "test_helper"

class LinguaRsRbTest < Minitest::Test
  def test_languages_list_exposes_english
    languages = LinguaRsRb.languages
    assert_kind_of Array, languages
    refute_empty languages
    assert_includes languages, "English"
  end

  def test_builder_detects_english
    detector = LinguaRsRb::LanguageDetectorBuilder
               .from_all_languages
               .with_minimum_relative_distance(0.0)
               .build

    result = detector.detect_language("This is a test sentence written in English.")
    assert_equal "English", result
  end

  def test_builder_detects_spanish_and_greek
    detector = LinguaRsRb::LanguageDetectorBuilder
               .from_all_languages
               .with_minimum_relative_distance(0.0)
               .build

    spanish = detector.detect_language("Hola, ¿cómo estás? Este texto está en español.")
    greek = detector.detect_language("Γειά σου! Αυτό το κείμενο είναι στα ελληνικά.")

    assert_equal "Spanish", spanish
    assert_equal "Greek", greek
  end

  def test_confidence_values_return_pairs
    detector = LinguaRsRb::LanguageDetectorBuilder
               .from_all_languages
               .build

    values = detector.compute_language_confidence_values("Bonjour tout le monde")
    assert_kind_of Array, values
    refute_empty values
    assert_kind_of Array, values.first
    assert_equal 2, values.first.size
  end

  def test_confidence_values_in_parallel_return_pairs
    detector = LinguaRsRb::LanguageDetectorBuilder
               .from_all_languages
               .build

    texts = ["Bonjour tout le monde", "Hola, ¿qué tal?"]
    values = detector.compute_language_confidence_values_in_parallel(texts)

    assert_kind_of Array, values
    assert_equal texts.size, values.size
    refute_empty values.first
    assert_kind_of Array, values.first.first
    assert_equal 2, values.first.first.size
  end

  def test_confidence_for_language_and_parallel
    detector = LinguaRsRb::LanguageDetectorBuilder
               .from_all_languages
               .with_minimum_relative_distance(0.0)
               .build

    text = "This is a short English sentence."
    confidence = detector.compute_language_confidence(text, "English")
    assert_kind_of Float, confidence
    assert confidence >= 0.0

    texts = [text, "Hola, ¿cómo estás?"]
    confidences = detector.compute_language_confidence_in_parallel(texts, "English")
    assert_kind_of Array, confidences
    assert_equal texts.size, confidences.size
    confidences.each { |value| assert_kind_of Float, value }
  end

  def test_builder_distance_validation
    builder = LinguaRsRb::LanguageDetectorBuilder.from_all_languages
    assert_raises(ArgumentError) do
      builder.with_minimum_relative_distance(1.1)
    end
  end
end
