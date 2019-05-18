// @TODO

// func Generate(dataVal string) string {
// 	// Identify items between brackets: {person.first}
// 	for strings.Count(dataVal, "{") > 0 && strings.Count(dataVal, "}") > 0 {
// 		catValue := ""
// 		startIndex := strings.Index(dataVal, "{")
// 		endIndex := strings.Index(dataVal, "}")
// 		replace := dataVal[(startIndex + 1):endIndex]
// 		categories := strings.Split(replace, ".")

// 		if len(categories) >= 2 && dataCheck([]string{categories[0], categories[1]}) {
// 			catValue = getRandValue([]string{categories[0], categories[1]})
// 		}

// 		dataVal = strings.Replace(dataVal, "{"+replace+"}", catValue, 1)
// 	}

// 	// Replace # with numbers
// 	dataVal = replaceWithNumbers(dataVal)

// 	// Replace ? with letters
// 	dataVal = replaceWithLetters(dataVal)

// 	return dataVal
// }