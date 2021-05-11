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
}