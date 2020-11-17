require 'ffi'

module Todoist
  extend FFI::Library
  lib_name = "libtodoist_add.#{::FFI::Platform::LIBSUFFIX}"

  ffi_lib "#{File.expand_path(File.dirname(__FILE__))}/../target/release/#{lib_name}"

  attach_function :add, [:string, :string], :string
  attach_function :string_free, [:string], :void
end

result = Todoist.add(ENV["token"] || "", "New task added via Ruby")
puts result
Todoist.string_free(result)
