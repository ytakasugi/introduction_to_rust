use std::cell::RefCell;

struct A {
    #[allow(dead_code)]
    c: char,
    #[allow(dead_code)]
    s: String
}

struct B {
    #[allow(dead_code)]
    c: char,
    // `String`を`RefCell`で包む
    s: RefCell<String>,
}

fn main() {
    // `RefCell`を使わない場合
    let a = A { c: 'a', s: "alex".to_string()};
    #[allow(unused_variables)]
    // 不変参照を作る
    let r = &a;
    // 不変参照経由でフィールドを変更しようとするとエラーとなる
    // r.s.push('a');

    // `RefCell`を使った場合
    let b = B { c: 'a', s: RefCell::new("alex".to_string()) };
    let rb = &b;
    // フィールドsのデータに対する可変の参照をとる
    rb.s.borrow_mut().push('a');
    {
        // 不変の参照をとる
        let rbs = b.s.borrow();
        assert_eq!(&*rbs, "alexa");
        
        // RefCellでは他の参照が有効な間に可変の参照をとろうとすると実行時にパニックする
        // b.s.borrow_mut();  // この時点で不変の参照rbsがまだ有効
        // → thread 'main' panicked at 'already borrowed: BorrowMutError'

        // try_borrow_mutならパニックせずErrを返してくれる
        assert!(b.s.try_borrow_mut().is_err());  // Errが返る
    }   // rbsはここでスコープを抜ける
    assert!(b.s.try_borrow_mut().is_ok());       // Okが返る
}
