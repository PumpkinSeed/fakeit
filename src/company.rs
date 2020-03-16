use crate::data::company;
use crate::misc;
use crate::name;
use ::std::string::String;

pub fn company() -> String {
    match misc::random::<i64>(1, 3) {
        1 => return format!("{}, {} and {}", name::last(), name::last(), name::last()),
        2 => return format!("{}-{}", name::last(), name::last()),
        3 => return format!("{} {}", name::last(), company_suffix()),
        _ => format!("impossible"),
    }
}

pub fn company_suffix() -> String {
    misc::random_data(company::SUFFIX).to_string()
}

pub fn buzzword() -> String {
    misc::random_data(company::BUZZWORDS).to_string()
}

pub fn bs() -> String {
    misc::random_data(company::BS).to_string()
}

#[cfg(test)]
mod tests {
    use crate::company;

    #[test]
    fn company() {
        let data1 = company::company();
        let data2 = company::company();
        assert_ne!(data1, data2);
    }

    #[test]
    fn company_suffix() {
        let data1 = company::company_suffix();
        let data2 = company::company_suffix();
        assert_ne!(data1, data2);
    }

    #[test]
    fn buzzword() {
        let data1 = company::buzzword();
        let data2 = company::buzzword();
        assert_ne!(data1, data2);
    }

    #[test]
    fn bs() {
        let data1 = company::bs();
        let data2 = company::bs();
        assert_ne!(data1, data2);
    }
}