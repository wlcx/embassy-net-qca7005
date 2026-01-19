use embedded_io::{Read, Write};

pub struct Qca7000<U> {
    uart: U,
}

impl<U> Qca7000<U>
where
    U: Read + Write,
{
    pub fn transmit(&self, buf: &[u8]) {
        unimplemented!()
    }
}

impl<U> embassy_net_driver::Driver for Qca7000<U>
where
    U: Read + Write,
{
    type RxToken<'a>
        = RxToken<'a>
    where
        Self: 'a;
    type TxToken<'a>
        = TxToken<'a, U>
    where
        Self: 'a;

    fn receive(
        &mut self,
        cx: &mut std::task::Context,
    ) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        unimplemented!()
    }

    fn transmit(&mut self, cx: &mut std::task::Context) -> Option<Self::TxToken<'_>> {
        unimplemented!()
    }
    fn link_state(&mut self, cx: &mut std::task::Context) -> embassy_net_driver::LinkState {
        unimplemented!();
    }
    fn capabilities(&self) -> embassy_net_driver::Capabilities {
        unimplemented!()
    }
    fn hardware_address(&self) -> embassy_net_driver::HardwareAddress {
        unimplemented!()
    }
}
pub struct RxToken<'a> {
    buf: &'a mut [u8],
}

impl<'a> embassy_net_driver::RxToken for RxToken<'a> {
    fn consume<R, F>(self, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        f(self.buf)
    }
}

pub struct TxToken<'a, U>
where
    U: Read + Write,
{
    buf: &'a mut [u8],
    qca: Qca7000<U>,
}

impl<'a, U> embassy_net_driver::TxToken for TxToken<'a, U>
where
    U: Read + Write,
{
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        assert!(len <= self.buf.len());
        let r = f(&mut self.buf[..len]);
        self.qca.transmit(&self.buf[..len]);
        r
    }
}
