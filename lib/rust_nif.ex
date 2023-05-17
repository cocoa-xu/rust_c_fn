defmodule RustNIF do
  use Rustler, otp_app: :rust_c_fn, crate: :rustnif

  def pointer_to_my_add_rs(), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
