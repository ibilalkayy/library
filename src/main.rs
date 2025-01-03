mod commands;
mod actions;

use clap::Parser;
use crate::commands::commands::{Actions, Library, BookDetails, BookTitle};
use crate::actions::actions::list_data;

fn main() {
    let library: Library = Library::parse();
    match library.manage_library {
        Actions::Add(details) => {
            let info = BookDetails {
                title: details.title,
                author: details.author,
                isbn: details.isbn,
            };

            let _ = info.write_data();
            println!("Data added successfully!");
        },

        Actions::Remove(details) => {
            let info = BookTitle { title: details.title };
            info.remove_data();
            println!("Data removed successfully!");
        },

        Actions::List => {
            list_data();
        },

        Actions::Search(details) => {
            let info = BookTitle { title: details.title };
            info.search_data();
        },

        Actions::Checkout(details) => {
            let info = BookTitle { title: details.title };
            info.checkout_book();
        },

        Actions::Return(details) => {
            let info = BookTitle { title: details.title };
            info.return_book();
        }
    }
}