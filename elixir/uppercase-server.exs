defmodule Uppercase do
  def upper(arg) do
    String.upcase(arg)
  end
  def bad(arg) do
    bad(arg+1)
  end
  def passing do
    receive do
      {:upper, arg, client} ->
	send(client, String.upcase(arg))
	passing()
      {:quit, client} ->
	send(client, :closing)
    end
  end
end


defmodule Runtime do
  def find_reductions(lst) do
    if lst == [] do
      :what
    else
      case hd(lst) do
	{:reductions, res} -> res
	_ -> find_reductions(tl(lst))
      end
    end
  end
  def top do
    processes = Process.list()
    reductions = Enum.map(processes, fn elem -> find_reductions(Process.info(elem)) end)
    Enum.take(Enum.sort(Enum.zip(processes, reductions), fn {_, a}, {_, b} -> b <= a end), 10)
  end
end

