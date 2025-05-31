def hello_from_bin() -> str:
    ...

def test_lib_code_fn() -> str:
    ...

def test_add_two_numbers(a: int, b: int) -> None:
    ...

#  2つの整数を受け取り、その和を返す関数
def test_lib_code_fn_2(a: int, b: int) -> str:
    ...

#  基本的な数値型を使用した関数
def test_numeric_types(int_val: int, float_val: float, unsigned_val: int) -> tuple[int, float, int]:
    ...

#  文字列と文字列スライスを使用した関数
def test_string_types(text: str) -> str:
    ...

#  配列とベクターを使用した関数
def test_collection_types(numbers: list[int], text_list: list[str]) -> tuple[list[int], list[str]]:
    ...

#  タプルを使用した関数
def test_tuple_types(tuple: tuple[int, str, float]) -> tuple[int, str, float]:
    ...

#  ハッシュマップを使用した関数
def test_hashmap_types(map: dict[str, int]) -> dict[str, int]:
    ...

#  オプション型を使用した関数
def test_option_types(maybe_number: int | None, maybe_text: str | None) -> tuple[int | None, str | None]:
    ...

def test_custom_struct(struct_instance: TestStruct) -> TestStruct:
    ...

def test_lib_code_b_fn() -> str:
    ...

def test_lib_code_b_fn_2() -> str:
    ...

