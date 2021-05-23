use lazy_static::lazy_static;
use std::collections::HashSet;
use std::error::Error;
use std::sync::RwLock;

lazy_static! {
    // staticな変数`DOGS`を導入する。この変数はプログラム全体で共有される
    // refは束縛モードと呼ばれ、不変の参照を意味する
    // lazy_staticではDOGSが初めて参照外しされたときに以下の初期化コードが実行される
    pub static ref DOGS: RwLock<HashSet<&'static str>> = {
        // HashSetを可変にするためにRwLockで包む
        let dogs = ["brushwood", "toy-poodle"].iter().cloned().collect();
        RwLock::new(dogs)
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    {
        let dogs = DOGS.read()?; // readロックを取得する
        assert!(dogs.contains("brushwood"));
        assert!(dogs.contains("toy-poodle"));
    }  // `dogs`がスコープを抜け、readロックが解除される

    fn stringify(x: impl ToString) -> String { 
        x.to_string() 
    }

    // writeロックを取得してHashSetに要素を追加する
    DOGS.write()?.insert("bull-terrier");

    std::thread::spawn(||
        // 別のスレッドでwriteロックを取得しHashSetに要素を追加する
        DOGS.write().map(|mut ds| ds.insert("corgi")).map_err(stringify)
    ).join().expect("Thread error")?;   // スレッドの終了を待つ

    // このスレッドと別スレッドの両方で追加した要素が見える
    assert!(DOGS.read()?.contains("bull-terrier"));
    assert!(DOGS.read()?.contains("corgi"));
    Ok(())
}
