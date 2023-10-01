struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn compare_area(&self, other: &Rectangle) -> bool {
        self.width * self.height > other.width * other.height
    }
}

fn double_value(x: i32) -> i32 {
    x * 2
}

fn greeting(name: &str) -> String {
    format!("Hello {} san", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_value() {
        assert_eq!(double_value(2), 4);
        assert_eq!(double_value(3), 6);
        assert_eq!(double_value(4), 8);
    }

    #[test]
    fn test_greeting() {
        assert_eq!(greeting("Taro"), "Hello Taro san");
        assert_eq!(greeting("Jiro"), "Hello Jiro san");
        assert_eq!(greeting("Saburo"), "Hello Saburo san");
    }

    #[test]
    fn test_compare_area() {
        let rect1 = Rectangle {
            width: 10,
            height: 20,
        };
        let rect2 = Rectangle {
            width: 5,
            height: 10,
        };
        let rect3 = Rectangle {
            width: 20,
            height: 40,
        };
        assert!(rect1.compare_area(&rect2));
        assert!(!rect2.compare_area(&rect3));
    }
}
