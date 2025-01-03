use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub struct Library {
    #[clap(subcommand)]
    pub manage_library: Actions,
}

#[derive(Debug, Subcommand)]
pub enum Actions {
    /// Add a new book to the library
    Add(BookDetails),

    /// Remove a book from the library by Title
    Remove(BookTitle),

    /// List all the books of the library
    List,

    /// Search a book from the library
    Search(BookTitle),

    /// Checkout a book from the library by Title
    Checkout(BookTitle),

    /// Return book to the library by Title
    Return(BookTitle),
}

#[derive(Debug, Parser)]
pub struct BookDetails {
    /// Enter the title of the book
    #[clap(short, long)]
    pub title: String,

    /// Enter the author name of the book
    #[clap(short, long)]
    pub author: String,

    /// Enter the isbn number of the book
    #[clap(short, long)]
    pub isbn: u64,
}

#[derive(Debug, Parser)]
pub struct BookTitle {
    /// Enter the title of the book
    #[clap(short, long)]
    pub title: String,
}