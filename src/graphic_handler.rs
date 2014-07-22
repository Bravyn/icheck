/*
* Rust-music-player - Copyright (c) 2014 Gomez Guillaume.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

#![allow(dead_code)]

use rsfml::system::vector2::{Vector2f};
use rsfml::window::{event, keyboard, mouse};
use rsfml::graphics::{RenderWindow, Color, Font};
use rfmod::enums::*;
use rfmod::*;
use rfmod::types::*;
use playlist::PlayList;
use graphic_timer::GraphicTimer;
use graphic_spectrum::GraphicSpectrum;
use graphic_playlist::GraphicPlayList;
use progress_bar::ProgressBar;
use graphic_button::GraphicButton;
use graphic_sound_position::GraphicSoundPosition;
use graphic_element::GraphicElement;

pub struct GraphicHandler {
    font: Font,
    musics: GraphicPlayList,
    timer: GraphicTimer,
    music_bar: ProgressBar,
    volume_bar: ProgressBar,
    playlist: PlayList,
    spectrum: GraphicSpectrum,
    graph_sound: GraphicSoundPosition,
    spectrum_button: GraphicButton,
    position_button: GraphicButton
}

impl GraphicHandler {
    fn init(mut self) -> GraphicHandler {
        self.music_bar.set_maximum(1u);
        self.musics.add_musics(&self.playlist.to_vec());
        self.volume_bar.set_maximum(100u);
        self.volume_bar.set_progress(100);
        self.spectrum_button.set_pushed(true);
        self.spectrum_button.set_label(&String::from_str("Spectrum"));
        self.position_button.set_label(&String::from_str("3D position"));
        self
    }

    pub fn new(window: &RenderWindow, playlist: PlayList) -> GraphicHandler {
        let font = match Font::new_from_file("./font/arial.ttf") {
            Some(s) => s,
            None => fail!("Cannot create Font")
        };
        GraphicHandler {
            font: font.clone(),
            musics: GraphicElement::new_init(&Vector2f{x: window.get_size().x as f32 - 511f32, y: window.get_size().y as f32 - 32f32},
                &Vector2f{x: 513f32, y: 0f32},
                &Color::black(),
                Some(&font)),
            timer: GraphicElement::new_init(&Vector2f{x: window.get_size().x as f32 - 633f32, y: 27f32},
                &Vector2f{x: window.get_size().x as f32 - (window.get_size().x as f32 - 634f32), y: window.get_size().y as f32 - 34f32},
                &Color::black(),
                Some(&font)),
            music_bar: GraphicElement::new_init(&Vector2f{x: window.get_size().x as f32 + 2f32, y: 8f32},
                &Vector2f{x: -1f32, y: window.get_size().y as f32 - 8f32},
                &Color::new_RGB(255, 255, 255),
                None),
            volume_bar: GraphicElement::new_init(&Vector2f{x: 120f32, y: 20f32},
                &Vector2f{x: 512f32, y: window.get_size().y as f32 - 30f32},
                &Color::new_RGB(255, 25, 25),
                None),
            playlist: playlist,
            spectrum_button: GraphicElement::new_init(&Vector2f{x: 256f32, y: 25f32},
                &Vector2f{x: 0f32, y: 0f32},
                &Color::black(),
                Some(&font)),
            position_button: GraphicElement::new_init(&Vector2f{x: 256f32, y: 25f32},
                &Vector2f{x: 256f32, y: 0f32},
                &Color::black(),
                Some(&font)),
            spectrum: GraphicElement::new_init(&Vector2f{x: 512f32, y: window.get_size().y as f32 - 33f32},
                &Vector2f{x: 0f32, y: 25f32},
                &Color::new_RGB(50, 100, 30),
                None),
            graph_sound: GraphicElement::new_init(&Vector2f{x: 512f32, y: window.get_size().y as f32 - 35f32},
                &Vector2f{x: 0f32, y: 26f32},
                &Color::black(),
                Some(&font))
        }.init()
    }

    pub fn set_music(&mut self, fmod: &FmodSys, name: String) -> Result<Sound, String> {
        match fmod.create_sound(name.as_slice(), Some(FmodMode(FMOD_SOFTWARE | FMOD_3D)), None) {
            Ok(s) => {
                s.set_3D_min_max_distance(4f32, 10000f32);
                self.musics.set_current(self.playlist.get_pos());
                self.music_bar.maximum = s.get_length(FMOD_TIMEUNIT_MS).unwrap() as uint;
                Ok(s)
            }
            Err(err) => {
                println!("FmodSys.create_sound failed on this file : {}\nError : {}", name, err);
                self.musics.remove_music(self.playlist.get_pos());
                self.playlist.remove_current();
                if self.playlist.get_nb_musics() == 0 {
                    Err(String::from_str("No more music"))
                } else {
                    let tmp_s = self.playlist.get_current();

                    self.set_music(fmod, tmp_s)
                }
            }
        }
    }

    pub fn set_music_position(&mut self, position: uint) {
        self.music_bar.set_progress(position);
    }

    pub fn update(&mut self, win: &mut RenderWindow) {
        self.musics.draw(win);
        self.volume_bar.draw(win);
        self.timer.draw(win);
        self.spectrum_button.draw(win);
        self.position_button.draw(win);
        if self.spectrum_button.is_pushed() {
            self.spectrum.draw(win);
        } else {
            self.graph_sound.draw(win);
        }
        self.music_bar.draw(win);
        win.display();
    }

    fn main_loop(&mut self, chan: &Channel, old_position: uint, length: u32) -> Option<uint> {
        match chan.is_playing() {
            Ok(b) => {
                if b == true {
                    let position = chan.get_position(FMOD_TIMEUNIT_MS).unwrap();

                    if position != old_position {
                        match chan.get_spectrum(256u, Some(1i32), Some(fmod::DSP_FFT_WindowRect)) {
                            Ok(f) => {
                                self.spectrum.update_spectrum(&chan.get_spectrum(256u, Some(0i32), Some(fmod::DSP_FFT_WindowRect)).unwrap(), &f);
                            }
                            Err(_) => {
                                self.spectrum.update_spectrum(&chan.get_spectrum(512u, Some(0i32), Some(fmod::DSP_FFT_WindowRect)).unwrap(), &Vec::new());
                            }
                        };
                        self.timer.update_display(position, length as uint);
                        Some(position)
                    } else {
                        Some(old_position)
                    }
                } else {
                    None
                }
            }
            Err(e) => fail!("fmod error : {}", e)
        }
    }

    pub fn start(&mut self, window: &mut RenderWindow, fmod: &FmodSys, update_time: f32) {
        let mut old_position = 100u;
        let mut tmp_s = self.playlist.get_current();
        let mut sound = match self.set_music(fmod, tmp_s) {
            Ok(s) => s,
            Err(e) => fail!("Error : {}", e)
        };
        let mut chan = match sound.play() {
            Ok(c) => c,
            Err(e) => fail!("sound.play : {}", e)
        };
        let forward = FmodVector {
            x: 0f32,
            y: 0f32,
            z: 0f32
        };
        let up = FmodVector {
            x: 0f32,
            y: 1f32,
            z: 0f32
        };
        let length = self.music_bar.maximum as u32;
        let mut listener_pos = FmodVector::new();
        let mut last_pos = FmodVector::new();

        window.clear(&Color::black());

        while window.is_open() {
            loop {
                match window.poll_event() {
                    event::Closed => window.close(),
                    event::KeyReleased{code, ..} => match code {
                        keyboard::Escape => window.close(),
                        keyboard::Up => {
                            tmp_s = self.playlist.get_prev();
                            sound = match self.set_music(fmod, tmp_s) {
                                Ok(s) => s,
                                Err(e) => fail!("Error : {}", e)
                            };
                            chan = match sound.play() {
                                Ok(c) => c,
                                Err(e) => fail!("sound.play : {}", e)
                            };
                            chan.set_volume(self.volume_bar.get_real_value() as f32 / 100f32);
                        }
                        keyboard::Down => {
                            tmp_s = self.playlist.get_next();
                            sound = match self.set_music(fmod, tmp_s) {
                                Ok(s) => s,
                                Err(e) => fail!("Error : {}", e)
                            };
                            chan = match sound.play() {
                                Ok(c) => c,
                                Err(e) => fail!("sound.play : {}", e)
                            };
                            chan.set_volume(self.volume_bar.get_real_value() as f32 / 100f32);
                        }
                        keyboard::Space => {
                            chan.set_paused(!chan.get_paused().unwrap());
                        }
                        keyboard::Delete => {
                            self.musics.remove_music(self.playlist.get_pos());
                            self.playlist.remove_current();
                            tmp_s = self.playlist.get_current();
                            sound = match self.set_music(fmod, tmp_s) {
                                Ok(s) => s,
                                Err(e) => fail!("Error : {}", e)
                            };
                            chan = match sound.play() {
                                Ok(c) => c,
                                Err(e) => fail!("sound.play : {}", e)
                            };
                            chan.set_volume(self.volume_bar.get_real_value() as f32 / 100f32);
                        }
                        _ => {}
                    },
                    event::KeyPressed{code, ..} => match code {
                        keyboard::Add => {
                            let tmp = self.volume_bar.get_real_value();
                            self.volume_bar.set_progress(tmp + 1);
                            chan.set_volume(self.volume_bar.get_real_value() as f32 / 100f32);
                        }
                        keyboard::Subtract => {
                            let tmp = self.volume_bar.get_real_value();
                            self.volume_bar.set_progress(tmp - 1);
                            chan.set_volume(self.volume_bar.get_real_value() as f32 / 100f32);
                        }
                        _ => {}
                    },
                    event::MouseButtonReleased{button, x, y} => match button {
                        mouse::MouseLeft => {
                            let v = Vector2f{x: x as f32, y: y as f32};

                            if self.music_bar.is_inside(&v) {
                                self.music_bar.clicked(&v);
                                chan.set_position(self.music_bar.get_real_value(), FMOD_TIMEUNIT_MS);
                            } else if self.volume_bar.is_inside(&v) {
                                self.volume_bar.clicked(&v);
                                chan.set_volume(self.volume_bar.get_real_value() as f32 / 100f32);
                            } else if self.musics.is_inside(&v) {
                                let old_c = self.musics.get_current();
                                self.musics.clicked(&v);
                                if old_c != self.musics.get_current() {
                                    self.playlist.set_actual(self.musics.get_current());

                                    let tmp_s = self.playlist.get_current();

                                    sound = match self.set_music(fmod, tmp_s) {
                                        Ok(s) => s,
                                        Err(e) => fail!("Error : {}", e)
                                    };
                                    chan = match sound.play() {
                                        Ok(c) => c,
                                        Err(e) => fail!("sound.play : {}", e)
                                    };
                                    chan.set_volume(self.volume_bar.get_real_value() as f32 / 100f32);
                                }
                            } else if self.graph_sound.is_inside(&v) {
                                self.graph_sound.clicked(&v);
                            } else if self.spectrum_button.is_inside(&v) && !self.spectrum_button.is_pushed() {
                                self.spectrum_button.clicked(&v);
                                self.position_button.clicked(&v);
                                self.spectrum.need_to_draw = true;
                            } else if self.position_button.is_inside(&v) && !self.position_button.is_pushed() {
                                self.position_button.clicked(&v);
                                self.spectrum_button.clicked(&v);
                                self.graph_sound.need_to_draw = true;
                            }
                        },
                        _ => {}
                    },
                    event::MouseWheelMoved{delta, ..} => {
                        let tmp = self.musics.get_add_to_view();
                        self.musics.set_to_add(tmp - delta);
                    },
                    event::MouseMoved{x, y} => {
                        let v = Vector2f{x: x as f32, y: y as f32};

                        if self.musics.is_inside(&v) {
                            self.musics.cursor_moved(&v);
                        } else {
                            self.musics.mouse_leave();
                        }
                        if self.spectrum_button.is_inside(&v) {
                            self.spectrum_button.cursor_moved(&v);
                        } else {
                            self.spectrum_button.mouse_leave();
                        }
                        if self.position_button.is_inside(&v) {
                            self.position_button.cursor_moved(&v);
                        } else {
                            self.position_button.mouse_leave();
                        }
                    }
                    event::NoEvent => break,
                    _ => {}
                }
            }

            let new_position = match self.main_loop(&chan, old_position, length) {
                Some(p) => {
                    self.set_music_position(p);
                    p
                },
                None => {
                    tmp_s = self.playlist.get_next();
                    sound = match self.set_music(fmod, tmp_s) {
                        Ok(s) => s,
                        Err(e) => fail!("Error : {}", e)
                    };
                    chan = match sound.play() {
                        Ok(c) => c,
                        Err(e) => fail!("sound.play : {}", e)
                    };
                    chan.set_volume(self.volume_bar.get_real_value() as f32 / 100f32);
                    100u
                }
            };

            listener_pos.x = self.graph_sound.x;
            //listener_pos.y = self.graph_sound.y;

            let mut vel = FmodVector::new();
            vel.x = (listener_pos.x - last_pos.x) * update_time;
            vel.y = (listener_pos.y - last_pos.y) * update_time;
            vel.z = (listener_pos.z - last_pos.z) * update_time;

            last_pos = listener_pos;
            match fmod.set_3D_listener_attributes(0, &listener_pos, &vel, &forward, &up) {
                fmod::Ok => {}
                e => {println!("set_3D_listener_attributes error : {}", e);}
            };

            if old_position != new_position {
                old_position = new_position;
                self.update(window);
            }

            unsafe {
                let mut t1 = format!("|.......................<1>......................<2>....................|");
                let t = t1.as_mut_vec();
                let le = t.len();
                *t.get_mut((listener_pos.x as uint + 30u) * le / 60u) = 'L' as u8;
                print!("{}\r", String::from_utf8(t.clone()).unwrap());
            }
            //println!("{}", listener_pos.x);
        }
    }
}