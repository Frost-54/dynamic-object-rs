load("//bazel/crates:syn.bzl", "syn")
load("//bazel/crates:uuid.bzl", "uuid")
load("//bazel/crates:quote.bzl", "quote")

def dynamic_dependencies():
      quote()
      syn()
      uuid()
