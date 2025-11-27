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
    use crate::testify::exec_mes;

    #[test]
    fn name() {
        exec_mes("beer::name", beer::name);
    }

    #[test]
    fn style() {
        exec_mes("beer::style", beer::style);
    }

    #[test]
    fn hop() {
        exec_mes("beer::hop", beer::hop);
    }

    #[test]
    fn yeast() {
        exec_mes("beer::yeast", beer::yeast);
    }

    #[test]
    fn malt() {
        exec_mes("beer::malt", beer::malt);
    }

    #[test]
    fn ibu() {
        exec_mes("beer::ibu", beer::ibu);
    }

    #[test]
    fn alcohol() {
        exec_mes("beer::alcohol", beer::alcohol);
    }

    #[test]
    fn blg() {
        exec_mes("beer::blg", beer::blg);
    }
}
