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
    use crate::testify::exec_mes;

    #[test]
    fn company() {
        exec_mes("company::company", || {
            company::company()
        });
    }

    #[test]
    fn company_suffix() {
        exec_mes("company::company_suffix", || {
            company::company_suffix()
        });
    }

    #[test]
    fn buzzword() {
        exec_mes("company::buzzword", || {
            company::buzzword()
        });
    }

    #[test]
    fn bs() {
        exec_mes("company::bs", || {
            company::bs()
        });
    }
}