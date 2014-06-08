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

use rsfml::graphics::rc;
use rsfml::system::vector2::{Vector2f, Vector2u};
use rsfml::graphics::{RenderWindow, Color, Text, Font, RectangleShape};
use std::rc::Rc;
use std::cell::RefCell;

pub struct GraphicPlayList {
    musics: Vec<String>,
    texts: Vec<rc::Text>,
    graphic_size: Vector2u,
    position: Vector2u,
    to_draw: uint,
    current: uint,
    border: rc::RectangleShape,
    hover_element: Option<uint>,
    add_to_view: int
}

impl GraphicPlayList {
    fn init(mut self, font: &Font) -> GraphicPlayList {
        for tmp in self.musics.iter() {
            self.texts.push(match rc::Text::new_init(tmp.as_slice().split_terminator('/').last().unwrap(), Rc::new(RefCell::new(font.clone())), 20) {
                Some(t) => t,
                None => fail!("Cannot create Text")
            });
        }
        let tmp = self.position.clone();
        self.set_position(&tmp);
        self.set_current(0u);
        self
    }

    pub fn new(musics: Vec<String>, font: &Font) -> GraphicPlayList {
        GraphicPlayList {
            musics: musics,
            texts: Vec::new(),
            graphic_size: Vector2u{x: 0u32, y: 0u32},
            position: Vector2u{x: 0u32, y: 0u32},
            to_draw: 0u,
            current: 1u,
            border: match rc::RectangleShape::new_init(&Vector2f{x: 0f32, y: 1f32}) {
                Some(l) => l,
                None => fail!("Cannot create border for GraphicPlayList")
            },
            hover_element: None,
            add_to_view: 0i
        }.init(font)
    }

    pub fn new_init(musics: Vec<String>, font: &Font, position: &Vector2u, size: &Vector2u) -> GraphicPlayList {
        GraphicPlayList {
            musics: musics,
            texts: Vec::new(),
            graphic_size: size.clone(),
            position: position.clone(),
            to_draw: 0u,
            current: 1u,
            border: match rc::RectangleShape::new_init(&Vector2f{x: 1f32, y: size.y as f32}) {
                Some(l) => l,
                None => fail!("Cannot create border for GraphicPlayList")
            },
            hover_element: None,
            add_to_view: 0i
        }.init(font)
    }

    pub fn set_position(&mut self, position: &Vector2u) {
        let mut pos = position.y;
        let limit = self.graphic_size.y + position.y;

        self.position = position.clone();
        self.to_draw = 0;
        self.border.set_position(&Vector2f{x: position.x as f32 - 1f32, y: position.y as f32});
        for tmp in self.texts.mut_iter() {
            tmp.set_position(&Vector2f{x: self.position.x as f32 + 4f32, y: pos as f32 + self.position.y as f32});
            if pos < limit {
                self.to_draw += 1;
            }
            pos += 22u32;
        }
    }

    pub fn set_to_add(&mut self, to_add: int) {
        let tmp_add = to_add * 22i;
        let max = (self.texts.len() as int + 2i) * 22i;

        if self.add_to_view != to_add && tmp_add >= 0i && tmp_add + self.to_draw as int * 22i < max {
            let mut pos = self.position.y as int - tmp_add as int;
            for tmp in self.texts.mut_iter() {
                let x = tmp.get_position().x;
                tmp.set_position(&Vector2f{x: x as f32, y: pos as f32});
                pos += 22i;
            }
            self.add_to_view = to_add;
        }
    }

    pub fn draw(&mut self, win: &mut RenderWindow) {
        let mut it = 0i;

        for tmp in self.texts.mut_iter() {
            if it == self.to_draw as int + self.add_to_view {
                break;
            }
            if it >= self.add_to_view as int {
                win.draw(tmp);
            }
            it += 1;
        }
        win.draw(&self.border);
    }

    pub fn set_current(&mut self, current: uint) {
        if current != self.current {
            self.texts.get_mut(current).set_color(&Color::new_RGB(255, 125, 25));
            self.texts.get_mut(self.current).set_color(&Color::new_RGB(255, 255, 255));
            self.current = current;
            if self.current + 2u >= self.to_draw {
                self.set_to_add(self.current as int + 2i - self.to_draw as int);
            } else {
                self.set_to_add(0i);
            }
        }
    }

    pub fn get_current(&self) -> uint {
        self.current
    }

    pub fn get_add_to_view(&self) -> int {
        self.add_to_view
    }

    pub fn remove_music(&mut self, pos: uint) {
        self.texts.remove(pos);
        let tmp = self.position;
        self.set_position(&tmp);
    }

    pub fn is_inside(&self, pos: &Vector2u) -> bool {
        pos.y >= self.position.y && pos.y <= self.position.y + self.graphic_size.y &&
        pos.x >= self.position.x && pos.x <= self.position.x + self.graphic_size.x
    }

    pub fn mouse_leave(&mut self) {
        match self.hover_element {
            Some(s) => {
                self.texts.get_mut(s).set_color(&Color::new_RGB(255, 255, 255));
                self.hover_element = None;
            }
            None => {}
        }
    }

    pub fn click(&mut self, y: int) -> bool {
        if y >= self.position.y as int {
            let tmp = ((y as f32 - self.position.y as f32) / 22f32 + self.add_to_view as f32) as uint;
            
            if tmp < self.texts.len() {
                self.hover_element = match self.hover_element {
                    Some(s) => {
                        self.texts.get_mut(s).set_color(&Color::new_RGB(255, 255, 255));
                        None
                    }
                    None => None
                };
                self.set_current(tmp);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn cursor_moved(&mut self, y: int) {
        if y >= self.position.y as int {
            let tmp = ((y as f32 - self.position.y as f32) / 22f32 + self.add_to_view as f32) as uint;

            if tmp >= self.texts.len() {
                self.hover_element = None;
                return;
            }
            match self.hover_element {
                Some(s) => {
                    if self.current == tmp {
                        self.texts.get_mut(s).set_color(&Color::new_RGB(255, 255, 255));
                        self.hover_element = None;
                    } else if s != tmp {
                        self.texts.get_mut(s).set_color(&Color::new_RGB(255, 255, 255));
                        self.hover_element = Some(tmp);
                        self.texts.get_mut(tmp).set_color(&Color::new_RGB(255, 175, 100));
                    }
                }
                None => {
                    if self.current != tmp {
                        self.hover_element = Some(tmp);
                        self.texts.get_mut(tmp).set_color(&Color::new_RGB(255, 175, 100));
                    } 
                }
            }
        }
    }
}