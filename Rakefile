require "bundler/gem_tasks"
require "rake/testtask"
require "helix_runtime/build_task"

HelixRuntime::BuildTask.new("iron_rb")

task default_task: :build

Rake::TestTask.new(:test) do |t|
  t.libs << "test"
  t.libs << "lib"
  t.test_files = FileList["test/**/*_test.rb"]
end

task :default => :test
