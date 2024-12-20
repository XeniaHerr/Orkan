#[allow(unused_imports)]
use smithay_client_toolkit::{self, compositor::{CompositorHandler, CompositorState}, delegate_compositor, delegate_keyboard, delegate_layer, delegate_output, delegate_registry, delegate_seat, delegate_shm, output::{OutputHandler, OutputState}, registry::{ProvidesRegistryState, RegistryHandler, RegistryState}, registry_handlers, seat::{keyboard::{KeyCode, KeyEvent, KeyboardHandler, Keysym}, 
                         Capability, SeatHandler, SeatState}, shell::{wlr_layer::{LayerShell, LayerShellHandler, LayerSurface, LayerSurfaceConfigure}, xdg::window::{Window, WindowHandler}, WaylandSurface}, shm::{slot::{Buffer, SlotPool}, Shm, ShmHandler}
                    };


use wayland_client::{protocol::{wl_keyboard, wl_seat, wl_shm, wl_surface}, Connection, QueueHandle};

use crate::orkan::search_element;

use super::search_element::{Searcher, SearchElement};

use super::draw_utils::Renderer;


use nix::unistd::execve;

use std::{cmp::{max, min}, ffi::CString};

use std::env;



/**
 * Window struct
 *
 * Struct that represents the drawn window. Should only exist once.
 */
#[allow(dead_code)]
pub struct OrkanWindow {

    pub registry_state : RegistryState,
    pub output_state : OutputState,
    pub shm : Shm,
    pub seat_state : SeatState,
    pub layer_surface: LayerSurface,
    pub pool : SlotPool,

    pub shift : Option<u32>,
    pub keyboard : Option<wl_keyboard::WlKeyboard>,
    pub buffer : Option<Buffer>,
    pub first_configure : bool,

    pub need_update : bool,

    pub padding_abs : i32,



    pub renderer : Renderer,

    pub valid_elements : Vec<SearchElement>,


    pub exists : bool,
    pub has_keyboard : bool,
    pub data :  Searcher,
    pub highlighted_pos : usize,
    pub handler : fn(&String),
}


impl OrkanWindow {

    fn draw2(&mut self, conn: &Connection, qh: &QueueHandle<Self>) {
        let (width, height) = (self.renderer.get_width(), self.renderer.get_height());

            let buffer = self.buffer.get_or_insert_with(|| {
                self.pool.create_buffer(width as i32, height as i32, width as i32 * 4, wl_shm::Format::Argb8888).expect("create Buffer").0
            });


    }

//TODO: Try to reuse the existing buffer if no change is needed.
    fn draw(&mut self, _conn: &Connection, qh: &QueueHandle<Self>) {


            let (width, height) = (self.renderer.get_width() , self.renderer.get_height());

       //     println!("Width: {width}, Height: {height}");
            let buffer = self.buffer.get_or_insert_with(|| {
                self.pool.create_buffer(width as i32, height as i32, width as i32 * 4, wl_shm::Format::Argb8888).expect("create Buffer").0
            });


            let canvas = match self.pool.canvas(buffer) {
                Some(canvas) => canvas,

                None => {
                    let (second, canvas) = self.pool.create_buffer(width as i32, height as i32, width as i32 * 4, wl_shm::Format::Argb8888).expect("create Buffer");

                    // A new buffer has to be always redrawn
                    self.need_update = true;
                    *buffer = second;
                    canvas
                }
            };

            //the Framework is making a lot of uneccecary calls to frame(). We only redraw the
            //buffer when we know that it is nececcary
            if self.need_update == true {
                self.renderer.render_full_image(canvas, self.valid_elements.clone(), self.highlighted_pos);
                self.need_update = false;
            }
            self.layer_surface.wl_surface().damage_buffer(0,0, width as i32, height as i32);

            self.layer_surface.wl_surface().frame(qh, self.layer_surface.wl_surface().clone());
            buffer.attach_to(self.layer_surface.wl_surface()).expect("buffer attach");

            self.layer_surface.commit();

    }


}



impl KeyboardHandler for OrkanWindow {


    fn enter(
        &mut self, 
        _: &Connection, 
        _: &QueueHandle<Self>, 
        _: &wl_keyboard::WlKeyboard, 
        _surface : &wl_surface::WlSurface, 
        _ : u32, 
        _: &[u32], 
        _keysyms: &[Keysym])  {
        println!("Keyboard Entered");

    }

    fn leave(
        &mut self, 
        _: &Connection, 
        _: &QueueHandle<Self>, 
        _: &wl_keyboard::WlKeyboard, 
        surface: &wl_surface::WlSurface, 
        _: u32) {

        if self.layer_surface.wl_surface() == surface {
            self.has_keyboard = false;
        }

        println!("Keyboard leave");

    }


    fn press_key(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _keyboard: &wl_keyboard::WlKeyboard,
            _serial: u32,
            _event: KeyEvent,
        ) {

        match _event.keysym {

            Keysym::Escape => {
            self.exists = false;
            return;
            }


            Keysym::Return => {
            println!("Return Pressed");
            println!("Selected: {}", self.valid_elements[self.highlighted_pos].search_string);
            self.exists = false;


            (self.handler)(&self.valid_elements[self.highlighted_pos].ful_path);
            }

            Keysym::BackSpace => {
            self.renderer.cur_search.pop();
            println!("Backspace Pressed");
            self.valid_elements = self.data.simple_search(self.renderer.cur_search.iter().collect::<String>().as_str());
            }

            Keysym::Right => {
            self.highlighted_pos = min(self.highlighted_pos +1, self.valid_elements.len()-1);
            }

            Keysym::Left => {
            self.highlighted_pos = if self.highlighted_pos == 0 { 0 } else {self.highlighted_pos -1};
            }


            _ => {

                if let Some(key) = _event.keysym.key_char() {
            self.renderer.cur_search.push(key);
            println!("Key Pressed: {key:?}");
            self.valid_elements = self.data.simple_search(self.renderer.cur_search.iter().collect::<String>().as_str());
                }
            }
        }

        
       self.need_update = true; 
    }


    fn release_key(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _keyboard: &wl_keyboard::WlKeyboard,
            _serial: u32,
            _event: KeyEvent,
        ) {
    }

    fn update_keymap(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _keyboard: &wl_keyboard::WlKeyboard,
            _keymap: smithay_client_toolkit::seat::keyboard::Keymap<'_>,
        ) {
        
    }

    fn update_modifiers(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _keyboard: &wl_keyboard::WlKeyboard,
            _serial: u32,
            _modifiers: smithay_client_toolkit::seat::keyboard::Modifiers,
            _layout: u32,
        ) {
        
    }

}



impl OutputHandler for OrkanWindow {

    fn new_output(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _output: wayland_client::protocol::wl_output::WlOutput,
        ) {


        
    }

    fn output_destroyed(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _output: wayland_client::protocol::wl_output::WlOutput,
        ) {
        
    }
    fn output_state(&mut self) -> &mut OutputState {
       &mut self.output_state
    }

    fn update_output(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _output: wayland_client::protocol::wl_output::WlOutput,
        ) {
        
    }

}



impl SeatHandler for OrkanWindow {

    fn new_seat(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _seat: wl_seat::WlSeat) {
        
    }

    fn new_capability(
            &mut self,
            _conn: &Connection,
            qh: &QueueHandle<Self>,
            seat: wl_seat::WlSeat,
            capability: smithay_client_toolkit::seat::Capability,
        ) {
        if capability == smithay_client_toolkit::seat::Capability::Keyboard && self.keyboard.is_none() {
            let kb = self.seat_state.get_keyboard(qh, &seat, None).expect("Keyboard error");

            self.keyboard = Some(kb);

        }
    }
    
    fn seat_state(&mut self) -> &mut SeatState {
        &mut self.seat_state
    }

    fn remove_seat(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _seat: wl_seat::WlSeat) {
        
    }

    fn remove_capability(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _seat: wl_seat::WlSeat,
            capability: Capability,
        ) {
        
        if capability == smithay_client_toolkit::seat::Capability::Keyboard && self.keyboard.is_some() {
            self.keyboard.take().unwrap().release();
            println!("Keyboard Removed");
        }
    }
}


impl CompositorHandler for OrkanWindow {

    fn frame(
            &mut self,
            conn: &Connection,
            qh: &QueueHandle<Self>,
            _surface: &wl_surface::WlSurface,
            _time: u32,
        ) {
        self.draw(conn, qh);
    }

    fn surface_enter(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _surface: &wl_surface::WlSurface,
            _output: &wayland_client::protocol::wl_output::WlOutput,
        ) {
    }

    fn surface_leave(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _surface: &wl_surface::WlSurface,
            _output: &wayland_client::protocol::wl_output::WlOutput,
        ) {
        
    }

    fn transform_changed(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _surface: &wl_surface::WlSurface,
            _new_transform: wayland_client::protocol::wl_output::Transform,
        ) {
        
    }

    fn scale_factor_changed(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _surface: &wl_surface::WlSurface,
            _new_factor: i32,
        ) {
        
    }
}

impl ShmHandler for OrkanWindow {

    fn shm_state(&mut self) -> &mut Shm {
        &mut self.shm
    }
}


impl  LayerShellHandler for OrkanWindow {

    fn closed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _layer: &LayerSurface) {

        self.exists = false;
        
    }

    fn configure(
            &mut self,
            conn: &Connection,
            qh: &QueueHandle<Self>,
            _layer: &LayerSurface,
            configure: LayerSurfaceConfigure,
            _serial: u32,
        ) {

        self.need_update = true;

        if configure.new_size.0 == 0 || configure.new_size.1 == 0 {
            *self.renderer.get_height_mut() = 20;
        } else {
            println!("Size determined by Hyprland");
            *self.renderer.get_width_mut() = configure.new_size.0;
            *self.renderer.get_height_mut() = configure.new_size.1;
            println!("New Size: {}*{}", self.renderer.get_width(), self.renderer.get_height());
        }



        if self.first_configure {
            self.first_configure = false;
            println!("First Configure");
            self.draw(conn, qh);
        }
        
    }
    
}
delegate_keyboard!(OrkanWindow);
delegate_compositor!(OrkanWindow);
delegate_layer!(OrkanWindow);
delegate_seat!(OrkanWindow);
delegate_output!(OrkanWindow);
delegate_registry!(OrkanWindow);
delegate_shm!(OrkanWindow);

impl ProvidesRegistryState for OrkanWindow {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }
    registry_handlers![OutputState, SeatState];
}


