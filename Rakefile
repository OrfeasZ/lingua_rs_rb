# frozen_string_literal: true

require "bundler/gem_tasks"
require "rubocop/rake_task"
require "rake/testtask"

RuboCop::RakeTask.new

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("lingua_rs_rb.gemspec")

RbSys::ExtensionTask.new("lingua_rs_rb", GEMSPEC) do |ext|
  ext.lib_dir = "lib/lingua_rs_rb"
end

Rake::TestTask.new do |t|
  t.libs << "test"
  t.pattern = "test/**/*_test.rb"
end

task test: :compile

task default: %i[compile test rubocop]
