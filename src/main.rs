/*
 * This is the sxeme Compiler src This is to be the main or *driver* file
 * the parsers dir houses all parser content
 */
#![feature(buf_read_has_data_left)]
#![feature(file_create_new)]

mod parsers;
mod scanner;

fn main() {
    parsers::help();
}
