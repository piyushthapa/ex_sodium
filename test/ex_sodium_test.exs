defmodule ExSodiumTest do
  use ExUnit.Case
  doctest ExSodium

  test "greets the world" do
    assert ExSodium.hello() == :world
  end
end
