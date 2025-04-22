use crate::lexer::token::TokenWithPosition;
use std::iter::Peekable;

pub struct TokenStream<I>
where
    I: Iterator<Item = TokenWithPosition>,
{
    iter: Peekable<I>,
}

impl<I> TokenStream<I>
where
    I: Iterator<Item = TokenWithPosition>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter: iter.peekable(),
        }
    }

    pub fn peek(&mut self) -> Option<&TokenWithPosition> {
        self.iter.peek()
    }

    pub fn next(&mut self) -> Option<TokenWithPosition> {
        self.iter.next()
    }

    pub fn clone(&self) -> Vec<TokenWithPosition>
    where
        I: Clone,
        TokenWithPosition: Clone,
    {
        self.iter.clone().collect()
    }

    pub fn nth(&self, n: usize) -> Option<TokenWithPosition>
    where
        I: Clone,
        TokenWithPosition: Clone,
    {
        self.iter.clone().nth(n)
    }
    
    pub fn is_some(&mut self) -> bool {
        self.peek().is_some()
    }
}
