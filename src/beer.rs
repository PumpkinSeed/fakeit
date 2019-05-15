use crate::data::beer;
use crate::misc;
use ::std::string::String;

pub fn name() -> String {
    misc::random_data_str(beer::NAME).to_string()
}

pub fn style() -> String {
    misc::random_data_str(beer::STYLE).to_string()
}

pub fn hop() -> String {
    misc::random_data_str(beer::HOP).to_string()
}

pub fn yeast() -> String {
    misc::random_data_str(beer::YEAST).to_string()
}

pub fn malt() -> String {
    misc::random_data_str(beer::MALT).to_string()
}

pub fn ibu() -> String {
    format!("{} IBU", misc::random_int(10, 100))
}

pub fn alcohol() -> String {
    format!("{} %", misc::random_float(2.0, 10.0))
}

pub fn blg() -> String {
    format!("{}°Blg", misc::random_float(5.0, 20.0))
}

#[cfg(test)]
mod tests {
    use crate::beer;

    #[test]
    fn beer_name() {
        let name1 = beer::name();
        let name2 = beer::name();
        assert_ne!(name1, name2);
    }
    
    #[test]
    fn beer_style() {
        let style1 = beer::style();
        let style2 = beer::style();
        assert_ne!(style1, style2);
    }

    #[test]
    fn beer_hop() {
        let hop1 = beer::hop();
        let hop2 = beer::hop();
        assert_ne!(hop1, hop2);
    }
}
