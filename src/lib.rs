use std::{io::{self, Write}, sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc}, thread::{self, JoinHandle}, time::Duration};

use crossterm::{cursor, execute, terminal};

pub struct Throbber {
    frame: Arc<AtomicUsize>,
    animation_length: Arc<usize>,
    animation: Arc<Vec<char>>,
    frame_time: Arc<f32>,
    run: Arc<AtomicBool>,
    killed: Arc<AtomicBool>,
    join_handle: Option<JoinHandle<()>>
}

impl Throbber {
    fn spawn(animation: Arc<Vec<char>>, animation_length: Arc<usize>, frame: Arc<AtomicUsize>, frame_time: Arc<f32>, killed: Arc<AtomicBool>, run: Arc<AtomicBool>) -> JoinHandle<()> {
        thread::spawn(move || {
            loop {
                if killed.load(Ordering::SeqCst) {
                    break;
                }

                if run.load(Ordering::SeqCst) {
                    let mut stdout = io::stdout();

                    execute!(stdout, cursor::SavePosition {}).expect("Failed to save the cursor's position");
                    print!("{}", animation.get(frame.load(Ordering::SeqCst)).unwrap());
                    stdout.flush().expect("Failed to flush stdout");
                    execute!(stdout, cursor::RestorePosition {}).expect("Failed to restore the cursor's position");
                    frame.store((frame.load(Ordering::SeqCst) + 1) % animation_length.as_ref().clone(), Ordering::SeqCst);

                    thread::sleep(Duration::from_secs_f32(frame_time.as_ref().clone()));
                }
            }
        })
    }

    pub fn custom(animation: Vec<char>, frame_time: f32) -> Self {
        let mut new = Throbber {
            frame: Arc::new(AtomicUsize::new(0)),
            animation_length: Arc::new(animation.len()),
            animation: Arc::new(animation),
            run: Arc::new(AtomicBool::new(false)),
            frame_time: Arc::new(frame_time),
            killed: Arc::new(AtomicBool::new(false)),
            join_handle: None
        };

        new.join_handle = Some(Self::spawn(new.animation.clone(), new.animation_length.clone(), new.frame.clone(), new.frame_time.clone(), new.killed.clone(), new.run.clone()));
        new
    }

    pub fn classic(frame_time: f32) -> Self {
        let mut new = Throbber {
            frame: Arc::new(AtomicUsize::new(0)),
            animation_length: Arc::new(4),
            animation: Arc::new(['|', '/', '-', '\\'].to_vec()),
            run: Arc::new(AtomicBool::new(false)),
            frame_time: Arc::new(frame_time),
            killed: Arc::new(AtomicBool::new(false)),
            join_handle: None
        };

        new.join_handle = Some(Self::spawn(new.animation.clone(), new.animation_length.clone(), new.frame.clone(), new.frame_time.clone(), new.killed.clone(), new.run.clone()));
        new
    }

    pub fn braille(frame_time: f32) -> Self {
        let mut new = Throbber {
            frame: Arc::new(AtomicUsize::new(0)),
            animation_length: Arc::new(8),
            animation: Arc::new(['⠇', '⠋', '⠙', '⠸', '⢰', '⣠', '⣄', '⡆'].to_vec()),
            run: Arc::new(AtomicBool::new(false)),
            frame_time: Arc::new(frame_time),
            killed: Arc::new(AtomicBool::new(false)),
            join_handle: None
        };

        new.join_handle = Some(Self::spawn(new.animation.clone(), new.animation_length.clone(), new.frame.clone(), new.frame_time.clone(), new.killed.clone(), new.run.clone()));
        new
    }

    /// It's recommended to use raw mode so the animation can't be messed up by users.
    pub fn start(&self, raw_mode: bool) {
        if raw_mode {
            terminal::enable_raw_mode().expect("Failed to enable raw mode");
        }

        self.frame.store(0, Ordering::SeqCst);
        self.run.store(true, Ordering::SeqCst);
    }
    
    /// It's recommended to use raw mode so the animation can't be messed up by users. If you
    /// started the throbber with raw mode you for sure want to disable it with raw mode.
    pub fn stop(&self, raw_mode: bool) {
        if raw_mode {
            terminal::disable_raw_mode().expect("Failed to disable raw mode");
        }

        self.run.store(false, Ordering::SeqCst);
    }

    /// This automatically runs once out of scope, but in case you want to kill the thread early,
    /// call this. This automatically disables raw mode, since this is meant to be a one throbber
    /// for the whole crate library.
    pub fn kill_thread(&self) {
        terminal::disable_raw_mode();
        self.killed.store(true, Ordering::SeqCst);
    }
}

impl Drop for Throbber {
    fn drop(&mut self) {
        self.kill_thread();
    }
}

