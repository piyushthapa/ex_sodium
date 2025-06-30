defmodule ExSodium.LibSodium do
  @moduledoc """
    LibSodium Nif
  """
  use Rustler,
    otp_app: :ex_sodium

  # Function stubs - these will be replaced by the NIFs
  def scalarmult_ed25519_base_noclamp(_scalar), do: :erlang.nif_error(:nif_not_loaded)
  def hash_sha512(_input), do: :erlang.nif_error(:nif_not_loaded)
  def scalar_reduce(_scalar), do: :erlang.nif_error(:nif_not_loaded)
  def scalar_mul(_scalar_a, _scalar_b), do: :erlang.nif_error(:nif_not_loaded)
  def scalar_add(_scalar_a, _scalar_b), do: :erlang.nif_error(:nif_not_loaded)
end
