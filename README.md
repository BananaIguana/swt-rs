# swt-rs
An experimental native GUI for Rust based on [SWT](https://www.eclipse.org/swt).

The goal is to see if the API can be bridged across to Rust maintaining API accuracy. The [documentation](https://help.eclipse.org/latest/index.jsp?topic=/org.eclipse.platform.doc.isv/reference/api/org/eclipse/swt/package-summary.html) for SWT should act as a reference for Rust usage.

_Here is a simple SWT application written in Java:_
```
public static void main(String[] args)
{
    Display display = new Display();
    Shell shell = new Shell(display);
	
    Text text = new Text(shell, SWT.NONE);
    text.setText("Hello World from Java SWT!");
    text.pack();

    shell.open();

    while (!shell.isDisposed())
    {
        if (!display.readAndDispatch())
            display.sleep();
    }

    display.dispose();
}
```

_Using swt-rs, here is the equivalent written in Rust:_
```
fn main() -> Result<(), Error>
{
    let display = Display::new()?;
    let shell = Shell::new(&display)?;

    let text = Text::new(&shell, NONE)?;

    text.set_text(&"Hello World from Rust SWT!".to_string())?;
    text.pack()?;

    shell.open()?;

    while !shell.is_disposed()?
    {
        if !display.read_and_dispatch()?
        {
            display.sleep()?;
        }
    }

    Ok(())
}
```

The API coverage is very limited at the moment and the Rust equivalent of 'listeners' is not solved yet.

# Setup

Setup is 'development only' (i.e there is no packaged solution yet) and depends on you configuring your environment. Theoretically, this is multi-platform but I've only tested on macOS up to now, hence these instructions are for macOS only but should be translatable.

1. Ensure you have the [JDK](https://www.oracle.com/java/technologies/downloads/) installed.
2. Download an SWT Jar package from [here](https://download.eclipse.org/eclipse/downloads/index.html#Latest_Release). I use the folder `res` at the root of this repository for resources as it's configured to be ignored.
3. Add `JDK_HOME` to your environment. e.g:
  ```
  export JDK_HOME="/Library/Java/JavaVirtualMachines/jdk-20.jdk/Contents/Home"
  ```
4. Add `SWT_JAR_PATH` to your environment. e.g:
  ```
  export SWT_JAR_PATH="${MY_PROJECTS}/swt-rs/res/swt-4.22-cocoa-macosx-x86_64/swt.jar"
  ```
5. Make JDK library available on your system path. e.g:
  ```
  export DYLD_LIBRARY_PATH="${JDK_HOME}/lib:${JDK_HOME}/lib/server:${DYLD_LIBRARY_PATH}"
  ```

# Running

`cargo run --example helloworld`


