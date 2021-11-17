defmodule QRCodeRustTest do
  use ExUnit.Case

  doctest QRCodeRust

  test "greets the world" do
    assert QRCodeRust.hello() == :world
  end
end
