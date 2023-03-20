use dy_app::{app::App, build_css};
use sycamore::prelude::*;
use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
enum BuildError {
    #[error("Some")]
    BuildError,
}

fn main() -> Result<()> {
    run()?;

    Ok(())
}


fn run() -> Result<(), BuildError> {
    sycamore::render(|cx| {
        let app = App(cx);

        build_css();
        view! { cx, 
            (app)
        }
    });
    
    println!("Success!");
    Ok(())
}