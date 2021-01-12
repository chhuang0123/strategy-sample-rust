#[derive(Debug)]
struct Cart {
    shipper: Shipper,
    length: usize,
    width: usize,
    height: usize,
    weight: usize,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Shipper {
    BlackCat,
    HsinChu,
    PostOffice,
}

impl Cart {
    pub fn new(
        shipper: Shipper,
        length: usize,
        width: usize,
        height: usize,
        weight: usize,
    ) -> Self {
        Self {
            shipper,
            length,
            width,
            height,
            weight,
        }
    }

    fn shipping_fee(&self) -> f64 {
        match self.shipper {
            Shipper::BlackCat => {
                if self.weight > 20 {
                    return 500.0;
                }

                100.0 + self.weight as f64 * 10.0
            }
            Shipper::HsinChu => {
                let size = (self.length * self.width * self.height) as f64;

                if self.length > 100 || self.width > 100 || self.height > 100 {
                    return size * 0.00002 * 1100.0 + 500.0;
                }

                size * 0.00002 * 1200.0
            }
            Shipper::PostOffice => {
                let fee_by_weight = 80.0 + self.weight as f64 * 10.0;
                let size = (self.length * self.width * self.height) as f64;
                let fee_by_size = size * 0.00002 * 1100.0;

                if fee_by_weight < fee_by_size {
                    return fee_by_weight;
                }

                fee_by_size
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest(
        cart,
        expected,
        case(Cart::new(Shipper::BlackCat, 30, 20, 10, 5), 150.0),
        case(Cart::new(Shipper::BlackCat, 30, 20, 10, 50), 500.0),
        case(Cart::new(Shipper::HsinChu, 30, 20, 10, 50), 144.0),
        case(Cart::new(Shipper::HsinChu, 100, 20, 10, 50), 480.0),
        case(Cart::new(Shipper::PostOffice, 100, 20, 10, 3), 110.0),
        case(Cart::new(Shipper::PostOffice, 100, 20, 10, 300), 440.0)
    )]
    fn test_cart_shipping_fee_success(cart: Cart, expected: f64) {
        assert_eq!(cart.shipping_fee(), expected);
    }
}

fn main() {
    let cart = Cart::new(Shipper::PostOffice, 100, 20, 10, 300);
    println!("{:?} shipping fee: -> {}", cart, &cart.shipping_fee());
}
