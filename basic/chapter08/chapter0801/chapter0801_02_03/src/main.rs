// デカルト座標
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
struct CartesianCoord {
    x: f64,
    y: f64,
}

// 座標
struct PolarCoord {
    pub r: f64,
    pub theta: f64,
}

trait Coordinates {
    fn to_cartesian(self) -> CartesianCoord;
    // 関連関数
    fn from_cartesian(cart: CartesianCoord) -> Self;
}

// `CartesianCoord`構造体に`Coordinates`トレイトを実装する
impl Coordinates for CartesianCoord {
    fn to_cartesian(self) -> CartesianCoord {
        self
    }

    fn from_cartesian(cart: CartesianCoord) -> Self {
        cart
    }
}

// `PolarCoord`構造体に`Coordinates`トレイトを実装する
impl Coordinates for PolarCoord {
    fn to_cartesian(self) -> CartesianCoord {
        CartesianCoord {
            x: self.r * self.theta.cos(),
            y: self.r * self.theta.sin(),
        }
    }
    
    fn from_cartesian(cart: CartesianCoord) -> Self {
        PolarCoord {
            r: (cart.x.powf(2.) + cart.y.powf(2.)).sqrt(),
            theta: (cart.y / cart.x).atan(),
        }
    }
}

// タプルにもトレイトを実装できる
impl Coordinates for (f64, f64) {
    fn to_cartesian(self) -> CartesianCoord {
        CartesianCoord {
            x: self.0,
            y: self.1,
        }
    }
    fn from_cartesian(cart: CartesianCoord) -> Self {
        (cart.x, cart.y)
    }
}

fn print(point: impl Coordinates) {
    let p = point.to_cartesian();
    println!("({}, {})", p.x, p.y);
}

fn main() {
    print((0.0, 1.0));
    print(PolarCoord {
        r: 1.0,
        theta: std::f64::consts::PI / 2.0
    });
}
