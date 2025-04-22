use crate::lexer::token::TokenWithPosition;
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct TokenStream<I>
where
    I: Iterator,
{
    iter: Peekable<I>,
}

impl<I> TokenStream<I>
where
    I: Iterator,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter: iter.peekable(),
        }
    }

    pub fn peek(&mut self) -> Option<&I::Item> {
        self.iter.peek()
    }

    pub fn next(&mut self) -> Option<I::Item> {
        self.iter.next()
    }

    pub fn clone_iter(&self) -> Peekable<IntoIter<I::Item>>
    where
        I: Clone,
        I::Item: Clone,
    {
        let items: Vec<_> = self.iter.clone().collect();
        items.into_iter().peekable()
    }

    pub fn nth(&mut self, n: usize) -> Option<I::Item> {
        for _ in 0..n {
            if self.next().is_none() {
                return None;
            }
        }
        self.next()
    }
    
    pub fn is_some(&mut self) -> bool {
        self.peek().is_some()
    }
}
