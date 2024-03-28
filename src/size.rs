use std::fmt;

#[derive(PartialEq, PartialOrd, Eq, Ord, Copy, Clone, Debug)]
pub struct Size(u64);

impl Size {
    pub fn new(bytes: u64) -> Self {
        Self(bytes)
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // permet d'afficher selon un format
        if (self.0 as f64) < 1024.0 {
            write!(f, "{} B", self.0)
        } else if (self.0 as f64) < 1024.0*1024.0 {
            write!(f, "{:.1} KB", (self.0 as f64 / 1024.0))
        } else if (self.0 as f64) < 1024.0*1024.0*1024.0 {
            write!(f, "{:.1} MB", (self.0 as f64 / 1024.0/1024.0))
        } else {
            write!(f, "{:.1} GB", (self.0 as f64 / 1024.0/1024.0/1024.0))
        }
    }
}

impl std::ops::Add for Size {
    // permet de faire les opérations d'addition avec des Sizes
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Size::new(self.0 + other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_add() {
        let a: Size = Size(136);
        let b: Size = Size(164);
        assert_eq!(a+b, Size(300));
    }

    #[test]
    fn sould_display() {
        let a = Size(680);
        assert_eq!(format!("{a}"), "680 B");
        let a = Size(1027);
        assert_eq!(format!("{a}"), "1.0 KB");
        let a = Size(5879);
        assert_eq!(format!("{a}"), "5.7 KB");
        let a = Size(4444444);
        assert_eq!(format!("{a}"), "4.2 MB");
        let a = Size(5007009000);
        assert_eq!(format!("{a}"), "4.7 GB");
        let a = Size(2411724);
        assert_eq!(format!("{a}"), "2.3 MB")
    }
}