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

    #[test]
    fn name() {
        let name1 = beer::name();
        let name2 = beer::name();
        assert_ne!(name1, name2);
    }

    #[test]
    fn style() {
        let style1 = beer::style();
        let style2 = beer::style();
        assert_ne!(style1, style2);
    }

    #[test]
    fn hop() {
        let hop1 = beer::hop();
        let hop2 = beer::hop();
        assert_ne!(hop1, hop2);
    }

    #[test]
    fn yeast() {
        let yeast1 = beer::yeast();
        let yeast2 = beer::yeast();
        assert_ne!(yeast1, yeast2);
    }

    #[test]
    fn malt() {
        let malt1 = beer::malt();
        let malt2 = beer::malt();
        assert_ne!(malt1, malt2);
    }

    #[test]
    fn ibu() {
        let ibu1 = beer::ibu();
        let ibu2 = beer::ibu();
        assert_ne!(ibu1, ibu2);
    }

    #[test]
    fn alcohol() {
        let alcohol1 = beer::alcohol();
        let alcohol2 = beer::alcohol();
        assert_ne!(alcohol1, alcohol2);
    }

    #[test]
    fn blg() {
        let blg1 = beer::blg();
        let blg2 = beer::blg();
        assert_ne!(blg1, blg2);
    }
}
