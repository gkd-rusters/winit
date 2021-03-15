// Limit this example to only compatible platforms.
#[cfg(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
fn main() {
    use std::{thread::sleep, time::Duration};
    use native_dialog::{FileDialog, MessageDialog, MessageType};

    use simple_logger::SimpleLogger;
    use winit::{
        event::{Event, WindowEvent, KeyboardInput},
        event_loop::{ControlFlow, EventLoop},
        platform::run_return::EventLoopExtRunReturn,
        window::WindowBuilder,
    };
    use winit::event::{ElementState, StartCause, VirtualKeyCode};
    let mut event_loop = EventLoop::new();
    fn echo<T: std::fmt::Debug>(name: &str, value: &T) {
        MessageDialog::new()
            .set_title("Result")
            .set_text(&format!("{}:\n{:#?}", &name, &value))
            .show_alert()
            .unwrap();
    }
    SimpleLogger::new().init().unwrap();
    let _window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .build(&event_loop)
        .unwrap();

    let mut quit = false;

    while !quit {
        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            if let Event::WindowEvent { event, .. } = &event {
                // Print only Window events to reduce noise
                println!("{:?}", event);
            }

            match event {
                Event::WindowEvent {
                    event ,
                    ..
                } => match event {
                    WindowEvent::CloseRequested =>{quit = true;}
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(virtual_code),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } => match virtual_code {
                        VirtualKeyCode::R => {
                            let result = MessageDialog::new()
                                .set_title("Tour")
                                .set_text("Do you want to begin the tour?")
                                .set_type(MessageType::Info)
                                .show_confirm()
                                .unwrap();
                            if !result {
                                return;
                            }
                            echo("show_confirm", &result);
                        }

                        _ => (),
                    },
                    _ => (),
                }
                Event::MainEventsCleared => {
                    *control_flow = ControlFlow::Exit;
                }

                _ => (),
            }
        });

        // Sleep for 1/60 second to simulate rendering
        println!("rendering");
        sleep(Duration::from_millis(16));
    }
}

#[cfg(any(target_os = "ios", target_os = "android", target_arch = "wasm32"))]
fn main() {
    println!("This platform doesn't support run_return.");
}
