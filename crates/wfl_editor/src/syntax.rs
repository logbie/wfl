use egui::text::{LayoutJob, TextFormat};
use egui::{Color32, FontId, Stroke};
use wfl_core::lexer::{lex_wfl_with_positions, Token, TokenWithPosition};

pub struct SyntaxHighlighter {
    dark_theme: SyntaxTheme,
    
    light_theme: SyntaxTheme,
}

struct SyntaxTheme {
    keyword: Color32,
    identifier: Color32,
    string: Color32,
    number: Color32,
    comment: Color32,
    operator: Color32,
    error: Color32,
    background: Color32,
}

impl SyntaxTheme {
    fn dark() -> Self {
        Self {
            keyword: Color32::from_rgb(86, 156, 214),    // Blue
            identifier: Color32::from_rgb(220, 220, 220), // Light gray
            string: Color32::from_rgb(206, 145, 120),    // Orange
            number: Color32::from_rgb(181, 206, 168),    // Light green
            comment: Color32::from_rgb(106, 153, 85),    // Green
            operator: Color32::from_rgb(180, 180, 180),  // Gray
            error: Color32::from_rgb(244, 71, 71),       // Red
            background: Color32::from_rgb(30, 30, 30),   // Dark gray
        }
    }
    
    fn light() -> Self {
        Self {
            keyword: Color32::from_rgb(0, 0, 255),       // Blue
            identifier: Color32::from_rgb(0, 0, 0),      // Black
            string: Color32::from_rgb(163, 21, 21),      // Red
            number: Color32::from_rgb(9, 134, 88),       // Green
            comment: Color32::from_rgb(0, 128, 0),       // Green
            operator: Color32::from_rgb(102, 102, 102),  // Gray
            error: Color32::from_rgb(255, 0, 0),         // Red
            background: Color32::from_rgb(255, 255, 255),// White
        }
    }
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            dark_theme: SyntaxTheme::dark(),
            light_theme: SyntaxTheme::light(),
        }
    }
    
    pub fn highlight(&self, text: &str, dark_mode: bool) -> LayoutJob {
        let theme = if dark_mode {
            &self.dark_theme
        } else {
            &self.light_theme
        };
        
        let mut job = LayoutJob::default();
        
        let tokens = lex_wfl_with_positions(text);
        
        for token in tokens {
            let format = self.token_format(&token.token, theme);
            let start = token.start;
            let end = token.start + token.length;
            
            if end <= text.len() {
                job.append(&text[start..end], 0.0, format);
            }
        }
        
        job
    }
    
    fn token_format(&self, token: &Token, theme: &SyntaxTheme) -> TextFormat {
        let color = match token {
            Token::Let | Token::Fn | Token::If | Token::Else | Token::While |
            Token::For | Token::Return | Token::Break | Token::Continue |
            Token::Import | Token::Export | Token::Async | Token::Await |
            Token::True | Token::False | Token::Null => theme.keyword,
            
            Token::Identifier(_) => theme.identifier,
            
            Token::StringLiteral(_) => theme.string,
            
            Token::IntLiteral(_) | Token::FloatLiteral(_) => theme.number,
            
            Token::Comment(_) => theme.comment,
            
            Token::Plus | Token::Minus | Token::Asterisk | Token::Slash |
            Token::Percent | Token::Equal | Token::NotEqual | Token::LessThan |
            Token::GreaterThan | Token::LessThanEqual | Token::GreaterThanEqual |
            Token::And | Token::Or | Token::Not | Token::Assign |
            Token::PlusAssign | Token::MinusAssign | Token::AsteriskAssign |
            Token::SlashAssign | Token::PercentAssign => theme.operator,
            
            Token::Error(_) => theme.error,
            
            _ => theme.identifier,
        };
        
        TextFormat {
            font_id: FontId::monospace(14.0),
            color,
            ..Default::default()
        }
    }
}
