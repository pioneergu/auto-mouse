// 위젯 관련 코드는 향후 확장 예정
pub struct StatusWidget {
    pub text: String,
}

impl StatusWidget {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }
} 