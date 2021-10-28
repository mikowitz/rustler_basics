defmodule Basics do
  use Rustler, otp_app: :basics, crate: "basics"

  def make_int, do: error()
  def make_float, do: error()
  def make_char, do: error()
  def make_atom, do: error()
  def make_charlist, do: error()
  def make_tuple, do: error()
  def make_string, do: error()
  def make_list, do: error()
  def make_map, do: error()

  def add1(_), do: error()
  def exclaim(_), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
