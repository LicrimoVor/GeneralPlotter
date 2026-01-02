#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BaudRate {
    Baud1200 = 1200,
    Baud2400 = 2400,
    Baud4800 = 4800,
    Baud9600 = 9600,
    Baud19200 = 19200,
    Baud38400 = 38400,
    Baud57600 = 57600,
    Baud115200 = 115200,
}

impl BaudRate {
    pub fn value(&self) -> u32 {
        match self {
            BaudRate::Baud1200 => 1200,
            BaudRate::Baud2400 => 2400,
            BaudRate::Baud4800 => 4800,
            BaudRate::Baud9600 => 9600,
            BaudRate::Baud19200 => 19200,
            BaudRate::Baud38400 => 38400,
            BaudRate::Baud57600 => 57600,
            BaudRate::Baud115200 => 115200,
        }
    }

    pub fn all() -> &'static [BaudRate] {
        &[
            BaudRate::Baud1200,
            BaudRate::Baud2400,
            BaudRate::Baud4800,
            BaudRate::Baud9600,
            BaudRate::Baud19200,
            BaudRate::Baud38400,
            BaudRate::Baud57600,
            BaudRate::Baud115200,
        ]
    }
}
