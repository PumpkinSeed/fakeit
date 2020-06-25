// url will generate a random Image Based Upon Height And Width. https://picsum.photos
pub fn url(width: i64, height: i64) -> String {
    format!("https://picsum.photos/{}/{}", width, height)
}
