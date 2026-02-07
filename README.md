# LinguaRsRb

Ruby bindings for the [lingua](https://github.com/pemistahl/lingua-rs) language detection library.

## Installation

Add the gem to your Gemfile:

```bash
bundle add lingua_rs_rb
```

Or install directly:

```bash
gem install lingua_rs_rb
```

## Usage

```ruby
require "lingua_rs_rb"

detector = LinguaRsRb::LanguageDetectorBuilder
	.from_all_languages
	.with_minimum_relative_distance(0.0)
	.build

detector.detect_language("This is an English sentence")
# => "English"

detector.compute_language_confidence("Bonjour tout le monde", "French")
# => 0.9...
```

Limit the detector to specific languages:

```ruby
detector = LinguaRsRb::LanguageDetectorBuilder
	.from_languages(["English", "French"])
	.build
```

## Development

This project uses a Rust native extension. Make sure you have Rust installed.

Install dependencies and run tests:

```bash
bundle install
bundle exec rake test
```

To build the gem locally:

```bash
gem build lingua_rs_rb.gemspec
```

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/OrfeasZ/lingua_rs_rb.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
