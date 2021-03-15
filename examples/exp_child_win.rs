use winit::window::WindowId;

// #[cfg(not(target_arch = "wasm32"))]
#[cfg(target_os = "macos")]
fn main() {
    use std::{collections::HashMap, sync::mpsc, thread, time::Duration};

    use simple_logger::SimpleLogger;
    use winit::{
        dpi::{PhysicalPosition, PhysicalSize, Position, Size},
        event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{CursorIcon, Fullscreen, WindowBuilder},
    };

    const WINDOW_COUNT: usize = 2;
    const WINDOW_SIZE: PhysicalSize<u32> = PhysicalSize::new(600, 400);

    SimpleLogger::new().init().unwrap();
    let event_loop = EventLoop::new();
    let mut window_senders = HashMap::with_capacity(WINDOW_COUNT);
    let mut win_vec = Vec::with_capacity(WINDOW_COUNT);
    for win_idx in 0..WINDOW_COUNT {
        let window = WindowBuilder::new()
            .with_inner_size(WINDOW_SIZE)
            .with_title(win_idx.to_string())
            .build(&event_loop)
            .unwrap();

        let mut video_modes: Vec<_> = window.current_monitor().unwrap().video_modes().collect();
        let mut video_mode_id = 0usize;
        let mut relevant_id = window.id();
        let (tx, rx) = mpsc::channel();
        window_senders.insert(window.id(), tx);
        if win_idx == 0 {
            win_vec.push(window.id());
        } else {
            let parent_id: WindowId = win_vec.pop().unwrap();
            // println!("0 add child 1...");
            // window.add_child_to(parent_id);
            relevant_id = parent_id;
        }
        let pair_info = (win_idx, relevant_id);
        thread::spawn(move || {
            let mut status = 1;
            while let Ok(event) = rx.recv() {
                match event {
                    WindowEvent::Moved { .. } => {
                        // We need to update our chosen video mode if the window
                        // was moved to an another monitor, so that the window
                        // appears on this monitor instead when we go fullscreen
                        let previous_video_mode = video_modes.iter().cloned().nth(video_mode_id);
                        video_modes = window.current_monitor().unwrap().video_modes().collect();
                        video_mode_id = video_mode_id.min(video_modes.len());
                        let video_mode = video_modes.iter().nth(video_mode_id);

                        // Different monitors may support different video modes,
                        // and the index we chose previously may now point to a
                        // completely different video mode, so notify the user
                        if video_mode != previous_video_mode.as_ref() {
                            println!(
                                "Window moved to another monitor, picked video mode: {}",
                                video_modes.iter().nth(video_mode_id).unwrap()
                            );
                        }
                    }
                    #[allow(deprecated)]
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Released,
                                virtual_keycode: Some(key),
                                modifiers,
                                ..
                            },
                        ..
                    } => {
                        window.set_title(&format!("{:?}", key));
                        let state = !modifiers.shift();
                        use VirtualKeyCode::*;
                        match key {
                            A => window.set_always_on_top(state),
                            C => window.set_cursor_icon(match state {
                                true => CursorIcon::Progress,
                                false => CursorIcon::Default,
                            }),
                            // D => window.set_decorations(!state),
                            D => {
                                if pair_info.0 == 1 {
                                    println!("this is window 1");
                                    if status == 1 {
                                        println!("status==1, add child");
                                        window.add_child_to(pair_info.1);
                                        status = 0;
                                    } else {
                                        println!("status==0, remove child");
                                        window.remove_self_as_child_from_parent();
                                        status = 1;
                                    }
                                } else {
                                    println!("this is window 0, id({:?})", &pair_info.1);
                                    window.child_windows();
                                }
                            }
                            // Cycle through video modes
                            Right | Left => {
                                video_mode_id = match key {
                                    Left => video_mode_id.saturating_sub(1),
                                    Right => (video_modes.len() - 1).min(video_mode_id + 1),
                                    _ => unreachable!(),
                                };
                                println!(
                                    "Picking video mode: {}",
                                    video_modes.iter().nth(video_mode_id).unwrap()
                                );
                            }
                            F => window.set_fullscreen(match (state, modifiers.alt()) {
                                (true, false) => Some(Fullscreen::Borderless(None)),
                                (true, true) => Some(Fullscreen::Exclusive(
                                    video_modes.iter().nth(video_mode_id).unwrap().clone(),
                                )),
                                (false, _) => None,
                            }),
                            G => window.set_cursor_grab(state).unwrap(),
                            H => window.set_cursor_visible(!state),
                            I => {
                                println!("Info:");
                                println!("-> outer_position : {:?}", window.outer_position());
                                println!("-> inner_position : {:?}", window.inner_position());
                                println!("-> outer_size     : {:?}", window.outer_size());
                                println!("-> inner_size     : {:?}", window.inner_size());
                                println!("-> fullscreen     : {:?}", window.fullscreen());
                            }
                            L => window.set_min_inner_size(match state {
                                true => Some(WINDOW_SIZE),
                                false => None,
                            }),
                            M => window.set_maximized(state),
                            P => window.set_outer_position({
                                let mut position = window.outer_position().unwrap();
                                let sign = if state { 1 } else { -1 };
                                position.x += 10 * sign;
                                position.y += 10 * sign;
                                position
                            }),
                            Q => window.request_redraw(),
                            R => window.set_resizable(state),
                            S => window.set_inner_size(match state {
                                true => PhysicalSize::new(
                                    WINDOW_SIZE.width + 100,
                                    WINDOW_SIZE.height + 100,
                                ),
                                false => WINDOW_SIZE,
                            }),
                            W => {
                                window.close_window();
                            }
                            Z => {
                                window.set_visible(false);
                                println!("set invisible");

                                thread::sleep(Duration::from_secs(15));

                                window.close_window();
                                println!("close_window");

                                thread::sleep(Duration::from_secs(15));

                                // window.remove_self_as_child_from_parent();
                                // window.set_visible(true);
                                if pair_info.0 == 1 {
                                    // status = 1;
                                    // if status == 1{
                                    // window.remove_self_as_child_from_parent();
                                    println!("status==1, add child");
                                    window.add_child_to(pair_info.1);
                                    status = 0;
                                    // }
                                }

                                thread::sleep(Duration::from_secs(15));
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }
        });
    }

    event_loop.run(move |event, _event_loop, control_flow| {
        *control_flow = match !window_senders.is_empty() {
            true => ControlFlow::Wait,
            false => ControlFlow::Exit,
        };
        match event {
            Event::WindowEvent { event, window_id } => match event {
                WindowEvent::CloseRequested
                | WindowEvent::Destroyed
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Released,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => {
                    window_senders.remove(&window_id);
                }
                _ => {
                    if let Some(tx) = window_senders.get(&window_id) {
                        if let Some(event) = event.to_static() {
                            tx.send(event).unwrap();
                        }
                    }
                }
            },
            _ => (),
        }
    })
}

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("Example not supported on Wasm");
}
