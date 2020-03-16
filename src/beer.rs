use crate::data::beer;
use crate::misc;

pub fn name() -> String {
    misc::random_data(beer::NAME).to_string()
}

pub fn style() -> String {
    misc::random_data(beer::STYLE).to_string()
}

pub fn hop() -> String {
    misc::random_data(beer::HOP).to_string()
}

pub fn yeast() -> String {
    misc::random_data(beer::YEAST).to_string()
}

pub fn malt() -> String {
    misc::random_data(beer::MALT).to_string()
}

pub fn ibu() -> String {
    format!("{} IBU", misc::random::<i64>(10, 100))
}

pub fn alcohol() -> String {
    format!("{} %", misc::random::<f32>(2.0, 10.0))
}

pub fn blg() -> String {
    format!("{}°Blg", misc::random::<f32>(5.0, 20.0))
}

#[cfg(test)]
mod tests {
    use crate::beer;
    use crate::test_helper;

    #[test]
    fn name() {
        let data1 = beer::name();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn style() {
        let data1 = beer::style();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn hop() {
        let data1 = beer::hop();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn yeast() {
        let data1 = beer::yeast();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn malt() {
        let data1 = beer::malt();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn ibu() {
        let data1 = beer::ibu();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn alcohol() {
        let data1 = beer::alcohol();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn blg() {
        let data1 = beer::blg();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }
}
