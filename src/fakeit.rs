use std::cell::Cell;

use crate::address;
use crate::animal;
use crate::beer;
use crate::bool_rand;
use crate::color;
use crate::company;
use crate::contact;
use crate::currency;
use crate::datetime;
use crate::file;
use crate::generator;
use crate::hacker;
use crate::hipster;
use crate::internet;
use crate::job;
use crate::language;
use crate::log_level;
use crate::misc;
use crate::name;
use crate::password;
use crate::payment;
use crate::person;
use crate::status_code;
use crate::unique;
use crate::user_agent;
use crate::vehicle;
use crate::words;

pub struct FakeIt {
    rng: Cell<simplerand::Rng>,
}

impl FakeIt {
    pub fn new(seed: u128) -> Self {
        let mut rng = simplerand::Rng::new();
        rng.set_seed(seed);
        FakeIt {
            rng: Cell::new(rng),
        }
    }

    fn with_rng<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let rng = self.rng.get();
        let prev = misc::install_rng(rng);
        let result = f();
        let advanced = misc::take_rng().unwrap_or(rng);
        self.rng.set(advanced);
        misc::restore_rng(prev);
        result
    }

    // words
    pub fn word(&self) -> String {
        self.with_rng(|| words::word())
    }
    pub fn sentence(&self, word_count: i64) -> String {
        self.with_rng(|| words::sentence(word_count))
    }
    pub fn paragraph(
        &self,
        count: i64,
        sentence_count: i64,
        word_count: i64,
        separator: String,
    ) -> String {
        self.with_rng(|| words::paragraph(count, sentence_count, word_count, separator))
    }
    pub fn question(&self) -> String {
        self.with_rng(|| words::question())
    }
    pub fn quote(&self) -> String {
        self.with_rng(|| words::quote())
    }

    // name
    pub fn name_full(&self) -> String {
        self.with_rng(|| name::full())
    }
    pub fn name_first(&self) -> String {
        self.with_rng(|| name::first())
    }
    pub fn name_last(&self) -> String {
        self.with_rng(|| name::last())
    }
    pub fn name_prefix(&self) -> String {
        self.with_rng(|| name::prefix())
    }
    pub fn name_suffix(&self) -> String {
        self.with_rng(|| name::suffix())
    }

    // address
    pub fn address_info(&self) -> address::Info {
        self.with_rng(|| address::info())
    }
    pub fn address_street(&self) -> String {
        self.with_rng(|| address::street())
    }
    pub fn address_street_number(&self) -> String {
        self.with_rng(|| address::street_number())
    }
    pub fn address_street_prefix(&self) -> String {
        self.with_rng(|| address::street_prefix())
    }
    pub fn address_street_name(&self) -> String {
        self.with_rng(|| address::street_name())
    }
    pub fn address_street_suffix(&self) -> String {
        self.with_rng(|| address::street_suffix())
    }
    pub fn address_city(&self) -> String {
        self.with_rng(|| address::city())
    }
    pub fn address_state(&self) -> String {
        self.with_rng(|| address::state())
    }
    pub fn address_state_abr(&self) -> String {
        self.with_rng(|| address::state_abr())
    }
    pub fn address_zip(&self) -> String {
        self.with_rng(|| address::zip())
    }
    pub fn address_country(&self) -> String {
        self.with_rng(|| address::country())
    }
    pub fn address_country_abr(&self) -> String {
        self.with_rng(|| address::country_abr())
    }
    pub fn address_latitude(&self) -> f32 {
        self.with_rng(|| address::latitude())
    }
    pub fn address_latitude_in_range(&self, min: f32, max: f32) -> f32 {
        self.with_rng(|| address::latitude_in_range(min, max))
    }
    pub fn address_longitude(&self) -> f32 {
        self.with_rng(|| address::longitude())
    }
    pub fn address_longitude_in_range(&self, min: f32, max: f32) -> f32 {
        self.with_rng(|| address::longitude_in_range(min, max))
    }

    // animal
    pub fn animal_pet_name(&self) -> String {
        self.with_rng(|| animal::pet_name())
    }
    pub fn animal(&self) -> String {
        self.with_rng(|| animal::animal())
    }
    pub fn animal_type(&self) -> String {
        self.with_rng(|| animal::type_of())
    }
    pub fn animal_farm(&self) -> String {
        self.with_rng(|| animal::farm())
    }
    pub fn animal_cat(&self) -> String {
        self.with_rng(|| animal::cat())
    }
    pub fn animal_dog(&self) -> String {
        self.with_rng(|| animal::dog())
    }

    // beer
    pub fn beer_name(&self) -> String {
        self.with_rng(|| beer::name())
    }
    pub fn beer_style(&self) -> String {
        self.with_rng(|| beer::style())
    }
    pub fn beer_hop(&self) -> String {
        self.with_rng(|| beer::hop())
    }
    pub fn beer_yeast(&self) -> String {
        self.with_rng(|| beer::yeast())
    }
    pub fn beer_malt(&self) -> String {
        self.with_rng(|| beer::malt())
    }
    pub fn beer_ibu(&self) -> String {
        self.with_rng(|| beer::ibu())
    }
    pub fn beer_alcohol(&self) -> String {
        self.with_rng(|| beer::alcohol())
    }
    pub fn beer_blg(&self) -> String {
        self.with_rng(|| beer::blg())
    }

    // bool
    pub fn bool(&self) -> bool {
        self.with_rng(|| bool_rand::bool())
    }

    // color
    pub fn color_full(&self) -> String {
        self.with_rng(|| color::full())
    }
    pub fn color_hex(&self) -> String {
        self.with_rng(|| color::hex())
    }
    pub fn color_safe(&self) -> String {
        self.with_rng(|| color::safe())
    }
    pub fn color_rgb(&self) -> [i16; 3] {
        self.with_rng(|| color::rgb())
    }

    // company
    pub fn company(&self) -> String {
        self.with_rng(|| company::company())
    }
    pub fn company_suffix(&self) -> String {
        self.with_rng(|| company::company_suffix())
    }
    pub fn company_buzzword(&self) -> String {
        self.with_rng(|| company::buzzword())
    }
    pub fn company_bs(&self) -> String {
        self.with_rng(|| company::bs())
    }

    // contact
    pub fn contact_info(&self) -> contact::Info {
        self.with_rng(|| contact::info())
    }
    pub fn contact_phone(&self) -> String {
        self.with_rng(|| contact::phone())
    }
    pub fn contact_phone_formatted(&self) -> String {
        self.with_rng(|| contact::phone_formatted())
    }
    pub fn contact_email(&self) -> String {
        self.with_rng(|| contact::email())
    }

    // currency
    pub fn currency_compact(&self) -> currency::Info {
        self.with_rng(|| currency::compact())
    }
    pub fn currency_short(&self) -> String {
        self.with_rng(|| currency::short())
    }
    pub fn currency_long(&self) -> String {
        self.with_rng(|| currency::long())
    }
    pub fn currency_price(&self, min: f64, max: f64) -> f64 {
        self.with_rng(|| currency::price(min, max))
    }

    // datetime
    pub fn datetime_month(&self) -> String {
        self.with_rng(|| datetime::month())
    }
    pub fn datetime_day(&self) -> String {
        self.with_rng(|| datetime::day())
    }
    pub fn datetime_week_day(&self) -> String {
        self.with_rng(|| datetime::week_day())
    }
    pub fn datetime_year(&self) -> String {
        self.with_rng(|| datetime::year())
    }
    pub fn datetime_hour(&self) -> String {
        self.with_rng(|| datetime::hour())
    }
    pub fn datetime_minute(&self) -> String {
        self.with_rng(|| datetime::minute())
    }
    pub fn datetime_second(&self) -> String {
        self.with_rng(|| datetime::second())
    }
    pub fn datetime_nanosecond(&self) -> String {
        self.with_rng(|| datetime::nanosecond())
    }
    pub fn datetime_timezone(&self) -> String {
        self.with_rng(|| datetime::timezone())
    }
    pub fn datetime_timezone_full(&self) -> String {
        self.with_rng(|| datetime::timezone_full())
    }
    pub fn datetime_timezone_abv(&self) -> String {
        self.with_rng(|| datetime::timezone_abv())
    }
    pub fn datetime_timezone_offset(&self) -> String {
        self.with_rng(|| datetime::timezone_offset())
    }
    pub fn datetime_date_range(&self, min: String, max: String) -> datetime::DateTime {
        self.with_rng(|| datetime::date_range(min, max))
    }
    pub fn datetime_date(&self) -> datetime::DateTime {
        self.with_rng(|| datetime::date())
    }

    // file
    pub fn file_mime_type(&self) -> String {
        self.with_rng(|| file::mime_type())
    }
    pub fn file_extension(&self) -> String {
        self.with_rng(|| file::extension())
    }

    // generator
    pub fn generate(&self, data: String) -> String {
        self.with_rng(|| generator::generate(data))
    }

    // hacker
    pub fn hacker_phrase(&self) -> String {
        self.with_rng(|| hacker::phrase())
    }
    pub fn hacker_abbreviation(&self) -> String {
        self.with_rng(|| hacker::abbreviation())
    }
    pub fn hacker_adjective(&self) -> String {
        self.with_rng(|| hacker::adjective())
    }
    pub fn hacker_noun(&self) -> String {
        self.with_rng(|| hacker::noun())
    }
    pub fn hacker_verb(&self) -> String {
        self.with_rng(|| hacker::verb())
    }
    pub fn hacker_ingverb(&self) -> String {
        self.with_rng(|| hacker::ingverb())
    }

    // hipster
    pub fn hipster_word(&self) -> String {
        self.with_rng(|| hipster::word())
    }
    pub fn hipster_sentence(&self, word_count: i64) -> String {
        self.with_rng(|| hipster::sentence(word_count))
    }
    pub fn hipster_paragraph(
        &self,
        count: i64,
        sentence_count: i64,
        word_count: i64,
        separator: String,
    ) -> String {
        self.with_rng(|| hipster::paragraph(count, sentence_count, word_count, separator))
    }

    // internet
    pub fn internet_domain_name(&self) -> String {
        self.with_rng(|| internet::domain_name())
    }
    pub fn internet_http_method(&self) -> String {
        self.with_rng(|| internet::http_method())
    }
    pub fn internet_domain_suffix(&self) -> String {
        self.with_rng(|| internet::domain_suffix())
    }
    pub fn internet_ipv4_address(&self) -> String {
        self.with_rng(|| internet::ipv4_address())
    }
    pub fn internet_ipv6_address(&self) -> String {
        self.with_rng(|| internet::ipv6_address())
    }
    pub fn internet_mac_address(&self) -> String {
        self.with_rng(|| internet::mac_address())
    }
    pub fn internet_username(&self) -> String {
        self.with_rng(|| internet::username())
    }

    // job
    pub fn job_info(&self) -> job::Info {
        self.with_rng(|| job::info())
    }
    pub fn job_title(&self) -> String {
        self.with_rng(|| job::title())
    }
    pub fn job_descriptor(&self) -> String {
        self.with_rng(|| job::descriptor())
    }
    pub fn job_level(&self) -> String {
        self.with_rng(|| job::level())
    }

    // language
    pub fn language(&self) -> String {
        self.with_rng(|| language::random())
    }
    pub fn language_abbreviation(&self) -> String {
        self.with_rng(|| language::abbreviation())
    }
    pub fn language_programming(&self) -> String {
        self.with_rng(|| language::programming())
    }

    // log_level
    pub fn log_level_general(&self) -> String {
        self.with_rng(|| log_level::general())
    }
    pub fn log_level_syslog(&self) -> String {
        self.with_rng(|| log_level::syslog())
    }
    pub fn log_level_apache(&self) -> String {
        self.with_rng(|| log_level::apache())
    }

    // password
    pub fn password(&self, upper: bool, numeric: bool, special: bool, num: i8) -> String {
        self.with_rng(|| password::generate(upper, numeric, special, num))
    }

    // payment
    pub fn payment_credit_card(&self) -> payment::CreditCard {
        self.with_rng(|| payment::credit_card())
    }
    pub fn payment_credit_card_type(&self) -> String {
        self.with_rng(|| payment::credit_card_type())
    }
    pub fn payment_credit_card_number(&self) -> String {
        self.with_rng(|| payment::credit_card_number())
    }
    pub fn payment_credit_card_luhn_number(&self) -> String {
        self.with_rng(|| payment::credit_card_luhn_number())
    }
    pub fn payment_credit_card_exp(&self) -> String {
        self.with_rng(|| payment::credit_card_exp())
    }
    pub fn payment_credit_card_cvv(&self) -> String {
        self.with_rng(|| payment::credit_card_cvv())
    }

    // person
    pub fn person_info(&self) -> person::Info {
        self.with_rng(|| person::info())
    }
    pub fn person_ssn(&self) -> String {
        self.with_rng(|| person::ssn())
    }
    pub fn person_gender(&self) -> String {
        self.with_rng(|| person::gender())
    }

    // status_code
    pub fn status_code_simple(&self) -> i16 {
        self.with_rng(|| status_code::simple())
    }
    pub fn status_code_general(&self) -> i16 {
        self.with_rng(|| status_code::general())
    }

    // unique
    pub fn unique_uuid_v1(&self) -> String {
        self.with_rng(|| unique::uuid_v1())
    }
    pub fn unique_uuid_v4(&self) -> String {
        self.with_rng(|| unique::uuid_v4())
    }

    // user_agent
    pub fn user_agent_chrome(&self) -> String {
        self.with_rng(|| user_agent::chrome())
    }
    pub fn user_agent_firefox(&self) -> String {
        self.with_rng(|| user_agent::firefox())
    }
    pub fn user_agent_safari(&self) -> String {
        self.with_rng(|| user_agent::safari())
    }
    pub fn user_agent_opera(&self) -> String {
        self.with_rng(|| user_agent::opera())
    }
    pub fn user_agent_random_platform(&self) -> String {
        self.with_rng(|| user_agent::random_platform())
    }

    // vehicle
    pub fn vehicle_info(&self) -> vehicle::Info {
        self.with_rng(|| vehicle::info())
    }
    pub fn vehicle_type(&self) -> String {
        self.with_rng(|| vehicle::vehicle_type())
    }
    pub fn vehicle_fuel(&self) -> String {
        self.with_rng(|| vehicle::fuel())
    }
    pub fn vehicle_transmission_gear(&self) -> String {
        self.with_rng(|| vehicle::transmission_gear())
    }
    pub fn vehicle_car_maker(&self) -> String {
        self.with_rng(|| vehicle::car_maker())
    }
    pub fn vehicle_car_model(&self) -> String {
        self.with_rng(|| vehicle::car_model())
    }
}

#[cfg(test)]
mod tests {
    use super::FakeIt;

    #[test]
    fn same_seed_same_output() {
        let f1 = FakeIt::new(42);
        let f2 = FakeIt::new(42);
        assert_eq!(f1.word(), f2.word());
    }

    #[test]
    fn different_seed_different_output() {
        let f1 = FakeIt::new(42);
        let f2 = FakeIt::new(9999);
        assert_ne!(f1.word(), f2.word());
    }

    #[test]
    fn deterministic_sequence() {
        let f1 = FakeIt::new(12345);
        let w1 = f1.word();
        let n1 = f1.name_first();
        let a1 = f1.address_city();

        let f2 = FakeIt::new(12345);
        let w2 = f2.word();
        let n2 = f2.name_first();
        let a2 = f2.address_city();

        assert_eq!(w1, w2);
        assert_eq!(n1, n2);
        assert_eq!(a1, a2);
    }

    #[test]
    fn advancing_state() {
        let f = FakeIt::new(42);
        let w1 = f.word();
        let w2 = f.word();
        // Successive calls should produce different results (state advances)
        assert_ne!(w1, w2);
    }

    #[test]
    fn global_seed() {
        use crate::misc;
        use crate::words;

        misc::seed(777);
        let w1 = words::word();
        misc::unseed();

        misc::seed(777);
        let w2 = words::word();
        misc::unseed();

        assert_eq!(w1, w2);
    }

    #[test]
    fn global_seed_sequence() {
        use crate::misc;
        use crate::name;
        use crate::words;

        misc::seed(555);
        let w1 = words::word();
        let n1 = name::first();
        misc::unseed();

        misc::seed(555);
        let w2 = words::word();
        let n2 = name::first();
        misc::unseed();

        assert_eq!(w1, w2);
        assert_eq!(n1, n2);
    }

    #[test]
    fn unseed_restores_random() {
        use crate::misc;
        use crate::words;

        misc::seed(42);
        misc::unseed();

        // After unseed, calls should be random (non-deterministic)
        // We can't assert randomness directly, but we can verify it doesn't panic
        let _ = words::word();
    }
}
