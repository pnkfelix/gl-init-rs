use {Event, WindowBuilder};

pub struct Window {
    _unimplemented: ()
}

pub struct MonitorID(uint);

pub fn get_available_monitors() -> Vec<MonitorID> {
    unimplemented!()
}

pub fn get_primary_monitor() -> MonitorID {
    unimplemented!()
}

impl MonitorID {
    pub fn get_name(&self) -> Option<String> {
        Some("<Unknown>".to_string())
    }

    pub fn get_dimensions(&self) -> (uint, uint) {
        unimplemented!()
    }
}

impl Window {
    pub fn new(builder: WindowBuilder) -> Result<Window, String> {
        unimplemented!()
    }

    pub fn is_closed(&self) -> bool {
        unimplemented!()
    }

    pub fn set_title(&self, title: &str) {
        unimplemented!()
    }

    pub fn get_position(&self) -> Option<(int, int)> {
        unimplemented!()
    }

    pub fn set_position(&self, x: uint, y: uint) {
        unimplemented!()
    }

    pub fn get_inner_size(&self) -> Option<(uint, uint)> {
        unimplemented!()
    }

    pub fn get_outer_size(&self) -> Option<(uint, uint)> {
        unimplemented!()
    }

    pub fn set_inner_size(&self, x: uint, y: uint) {
        unimplemented!()
    }

    pub fn poll_events(&self) -> Vec<Event> {
        unimplemented!()
    }

    pub fn wait_events(&self) -> Vec<Event> {
        unimplemented!()
    }

    pub unsafe fn make_current(&self) {
        unimplemented!()
    }

    pub fn get_proc_address(&self, addr: &str) -> *const () {
        unimplemented!()
    }

    pub fn swap_buffers(&self) {
        unimplemented!()
    }
}

impl Drop for Window {
    fn drop(&mut self) {
    }
}


#[link="support"]
extern {

}
