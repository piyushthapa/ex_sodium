defmodule ExSodium.Ed25519 do
  @moduledoc """
    Handles Ed25519 related functions
  """

  alias ExSodium.LibSodium

  def scalarmult_base_no_clamp(scalar) when is_binary(scalar) do
    scalar
    |> :binary.bin_to_list()
    |> LibSodium.scalarmult_ed25519_base_noclamp()
    |> :binary.list_to_bin()
  end

  def hash_sha512(input) when is_binary(input) do
    input
    |> :binary.bin_to_list()
    |> LibSodium.hash_sha512()
    |> :binary.list_to_bin()
  end

  def scalar_reduce(scalar) when is_binary(scalar) do
    scalar
    |> :binary.bin_to_list()
    |> LibSodium.scalar_reduce()
    |> :binary.list_to_bin()
  end

  def scalar_mul(scalar_a, scalar_b) when is_binary(scalar_a) and is_binary(scalar_b) do
    scalar_a = :binary.bin_to_list(scalar_a)
    scalar_b = :binary.bin_to_list(scalar_b)

    LibSodium.scalar_mul(scalar_a, scalar_b)
    |> :binary.list_to_bin()
  end

  def scalar_add(scalar_a, scalar_b) when is_binary(scalar_a) and is_binary(scalar_b) do
    scalar_a = :binary.bin_to_list(scalar_a)
    scalar_b = :binary.bin_to_list(scalar_b)

    LibSodium.scalar_add(scalar_a, scalar_b)
    |> :binary.list_to_bin()
  end
end
