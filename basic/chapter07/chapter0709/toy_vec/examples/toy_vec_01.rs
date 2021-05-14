use toy_vec::ToyVec;

fn main() {
    let mut v = ToyVec::new();
    // 所有権が`push`メソッドにムーブする
    v.push("Java Finch".to_string());
    // 所有権が`push`メソッドにムーブする
    v.push("Budgerigar".to_string());
    // 要素数未満なので、`Some(&"Budgerigar".to_string())`を返す
    // 1. `get`メソッドの呼び出し元である`v`が所有する`ToyVec`構造体について、その参照`&self`が引数として渡される
    // 2. 戻り値は、`&self`が指す`ToyVec`構造体の`elements(Box<[T]>)`が所有する値への参照となる
    // - つまり、`get`の戻り値が有効な間は借用中である
    // - 借用規則によると、戻り値のライフタイム(`e`のスコープ)が`self`のライフタイム(`v`のスコープ)より短くなければならない
    let e = v.get(1);
    assert_eq!(e, Some(&"Budgerigar".to_string()));
    // 要素数は`2`
    assert_eq!(v.len(), 2);
}