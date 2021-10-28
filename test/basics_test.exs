defmodule BasicsTest do
  use ExUnit.Case, async: true
  doctest Basics

  test "make_int/0" do
    assert Basics.make_int() == 42
  end

  test "make_float/0" do
    assert Basics.make_float() == 3.14159260123456789
  end

  test "make_char/0" do
    assert Basics.make_char() == ?a
  end

  test "make_charlist/0" do
    assert Basics.make_charlist() == 'lauren'
  end

  test "make_string/0" do
    assert Basics.make_string() == "Lauren"
  end

  test "make_tuple/0" do
    assert Basics.make_tuple() == {1, 2, 3}
  end

  test "make_list/0" do
    assert Basics.make_list() == [0, 1, 2, 3, 4]
  end

  test "make_map/0" do
    assert Basics.make_map() == %{
             a: 1,
             b: 2,
             c: 3
           }
  end

  test "make_atom/0" do
    assert Basics.make_atom() == :lauren
  end

  describe "add1/1" do
    test "for ints" do
      assert Basics.add1(3) == 4
    end

    test "for floats" do
      assert Basics.add1(7.5) == 8.5
    end

    test "for other types" do
      assert_raise ArgumentError, fn ->
        Basics.add1("3")
      end
    end
  end

  test "exclaim/1" do
    assert Basics.exclaim("hello") == "hello!"
  end
end
