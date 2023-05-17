defmodule RustCFnTest do
  use ExUnit.Case
  doctest RustCFn

  test "greets the world" do
    assert RustCFn.hello() == :world
  end
end
