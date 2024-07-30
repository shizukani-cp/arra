standard,=,module,"standard",  #print関数はdefaultの中に入っている

print_fn,=,attr,standard,print,  #print_fn関数にstandardモジュールのprint関数を設定
call,print_fn,"Hello, world!",  #callでprint関数を引数付きで呼び出す