use {
    jni::errors::Error,
    swt_rs::{
        constants::CENTER,
        widgets::{
            control::ControlTrait,
            display::DisplayTrait,
            label::{Label, LabelTrait},
            shell::ShellTrait,
        },
        Display,
        Shell,
    },
};

fn main() -> Result<(), Error>
{
    let display = Display::new()?;
    let shell = Shell::new(&display)?;

    let hello_world_text = Label::new(&shell, CENTER)?;

    hello_world_text.set_text(&"Hello World!".to_string())?;
    hello_world_text.set_size(100, 100)?;
    hello_world_text.set_location(20, 20)?;

    shell.open()?;
    shell.set_maximum_size(800, 500)?;
    shell.set_minimum_size(200, 150)?;

    while !shell.is_disposed()?
    {
        if !display.read_and_dispatch()?
        {
            display.sleep()?;
        }
    }

    Ok(())
}
