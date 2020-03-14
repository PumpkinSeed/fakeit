use crate::company;
use crate::data::internet;
use crate::data::person;
use crate::job;
use crate::misc;

pub fn domain_name() -> String {
	format!(
		"{}{}.{}",
		job::descriptor().to_lowercase(),
		company::bs().to_lowercase(),
		domain_suffix()
	)
}

pub fn http_method() -> String {
	misc::random_data(internet::HTTP_METHOD).to_string()
}

pub fn domain_suffix() -> String {
	misc::random_data(internet::DOMAIN_SUFFIX).to_string()
}

pub fn ipv4_address() -> String {
	format!(
		"{}.{}.{}.{}",
		misc::random::<i16>(2, 254),
		misc::random::<i16>(2, 254),
		misc::random::<i16>(2, 254),
		misc::random::<i16>(2, 254),
	)
}

pub fn ipv6_address() -> String {
	let num: i64 = 65536;
	misc::random::<i64>(0, num);
	format!(
		"2001:cafe:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",
		misc::random::<i64>(0, num),
		misc::random::<i64>(0, num),
		misc::random::<i64>(0, num),
		misc::random::<i64>(0, num),
		misc::random::<i64>(0, num),
		misc::random::<i64>(0, num),
	)
}

pub fn username() -> String {
	format!(
        "{}{}",
        misc::random_data(person::LAST).to_string(),
        misc::replace_with_numbers("####".to_string()),
	)
}

#[cfg(test)]
mod tests {
	use crate::internet;

	#[test]
	fn ipv6_address() {
		let new = internet::ipv6_address();
		println!("{}", new);
	}

	#[test]
	fn ipv4_address() {
		let new = internet::ipv4_address();
		println!("{}", new);
	}

	#[test]
	fn username() {
		let new = internet::username();
		println!("{}", new);
	}

	#[test]
	fn domain_name() {
		let new = internet::domain_name();
		println!("{}", new);
	}
}

// // URL will generate a random url string
// func URL() string {
// 	url := "http" + RandString([]string{"s", ""}) + "://www."
// 	url += DomainName()

// 	// Slugs
// 	num := Number(1, 4)
// 	slug := make([]string, num)
// 	for i := 0; i < num; i++ {
// 		slug[i] = BS()
// 	}
// 	url += "/" + strings.ToLower(strings.Join(slug, "/"))

// 	return url
// }
