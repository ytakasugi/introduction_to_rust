pub struct ToyVec<T> {
    // `T`型の要素を格納する領域。各要素はヒープ領域に置かれる
    elements: Box<[T]>,
    // `ToyVec`の長さ(要素数)
    len: usize
}

impl<T: Default> ToyVec<T> {
    // `new`は、キャパシティが0の`ToyVec`を作る
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    // with_capacityは指定されたキャパシティを持つToyVecを作る
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    // T型の値がsize個格納できるBox<[T]>を返す
    pub fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            // T型のデフォルト値をsize個作り
            .take(size)            
            // Vec<T>に収集してから
            .collect::<Vec<_>>()   
            // Box<[T]>に変換する
            .into_boxed_slice()    
    }

    // ベクタの長さ(要素数)を返す
    pub fn len(&self) -> usize {
        self.len
    }

    // ベクタの現在のキャパシティを返す
    pub fn capacity(&self) -> usize {
        // `elements`の要素数が`ToyVec`のキャパシティになる
        self.elements.len()
    }

    // 第一引数が`&mut self`なので、`ToyVec`の内容を変更する
    // 第二引数が`T`なので、所有権がこのメソッドにムーブする
    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() {
            self.grow();
        }
        // 要素を格納する(所有権がムーブする)
        self.elements[self.len] = element;
        self.len += 1;
    }

    // elementsを拡張する（より大きなサイズで作り直す）
    // 第一引数が`&self`なので、`ToyVec`は変更されない
    // 第二引数が`usize`なので、値がコピーされる(`usize`にはコピートレイトが実装されている)
    pub fn get(&self, index: usize) -> Option<&T> {
        // もし、`index`が要素数未満であれば
        if index < self.len {
            // `Some`でラップされた不変参照を返す
            Some(&self.elements[index])
        } else {
            // 要素数以上であれば、`None`を返す
            None
        }
    }

    pub fn grow(&mut self) {
        // もし、現在の`elements`が空なら
        if self.capacity() == 0 {
            // 1要素分の領域を確保する
            self.elements = Self::allocate_in_heap(1);
        } else {
            // 現在の2倍の領域を確保する
            let new_elements = Self::allocate_in_heap(self.capacity() * 2);
            // `self.elements`を置き換える
            // `std::mem::replace`は、第二引数の要素で第一引数の要素を置き換える
            let old_elements = std::mem::replace(&mut self.elements, new_elements);
            // 既存の全要素を新しい領域へムーブする
            // `Vec<T>`の`into_iter(self)`なら要素の所有権が得られる
            for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = elem;
            }
        }
    }

}