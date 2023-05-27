defmodule RustNIF do
  use Rustler, otp_app: :rust_c_fn, crate: :rustnif

  def rust_adbc_statement_execute_query(_func, _stmt, _error, _ref),
    do: :erlang.nif_error(:nif_not_loaded)
end
