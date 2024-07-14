# Arra
## プロジェクトの概要
自作言語（arra）の処理系（テスト用に自作言語のソースコードもあり）。  
Rustで開発している。ただしstringやintなどの標準オブジェクトはarraで書くことも検討中。  
## 使い方
```sh
arra build your_script.arr
arra run your_script.arr
```
## 制約を示すプログラムの例
### 最終行にはカンマが必須
```Arra
standard,=,module,"standard",
attr,standard,print,
call,ref,tmp,1,"Hello, world!",
```
※test_and_examples/hello_world.arr
### インデントの最初にカンマはつけない
```Arra
standard,=,module,"standard",
my_func,=,function,
    attr,standard,print,
    call,tmp,0,"this is \"my_func!\"",
end,
return_null_func,=,function,
    return,null,
end,
```
※test_and_examples/define_function.arr