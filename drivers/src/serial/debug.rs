//! Debug serial.

util::proc_macros::statemachine!(
    static DEBUGSERIAL = {}
    impl{
        /// Puts a byte onto serial output.
        pub fn putb(&self, b: u8){
            reqm!(SerialOut => |serial|{
                serial.putb(b)
            });
        }

        /// Puts a string onto serial output.
        pub fn puts(&self, s: &str){
            for b in s.bytes(){
                self.putb(b);
            }
        }
    }
);
