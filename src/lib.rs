use embedded_io::{Read, ReadReady, Write};

const MTU: usize = 1520;

pub struct Qca7000<U> {
    uart: U,
    rx_ptr: usize,
}

impl<U> Qca7000<U>
where
    U: Read + Write + ReadReady,
{
    pub fn transmit(&self, buf: &[u8]) {
        unimplemented!()
    }

    pub fn receive(&mut self, buf: &mut [u8]) -> Option<usize> {
        // check to see if ready to read, returning None if nothing available
        self.uart.read_ready().ok().and_then(|r| {
            if r {
                match self.uart.read(&mut buf[self.rx_ptr..]) {
                    Ok(len) => {
                        self.rx_ptr += len;
                        if rx_ptr > 3 {
                            // if we have enough, check to see if we have a
                        }
                    }
                    Err(e) => todo!(),
                }
            } else {
                None
            }
        })
    }
}

static mut TX_BUF: [u8; MTU] = [0; MTU];
// rx buffer is sized to fit MTU plus qca uart framing
static mut RX_BUF: [u8; 4 + 2 + 2 + MTU + 2] = [0; _];

impl<U> embassy_net_driver::Driver for Qca7000<U>
where
    U: Read + Write + ReadReady,
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
        // This is lifted from embassy-net-enc28j60. I'm not really sure of why this is sound, but I guess it must be - and I can't find any other examples of enet-driver crates (as opposed to enet-driver-channel).
        let rx_buf = unsafe { &mut *core::ptr::addr_of_mut!(RX_BUF) };
        let tx_buf = unsafe { &mut *core::ptr::addr_of_mut!(TX_BUF) };
        if let Some(n) = self.receive(rx_buf) {
            Some((
                RxToken {
                    buf: &mut rx_buf[..n],
                },
                TxToken {
                    buf: tx_buf,
                    qca: self,
                },
            ))
        } else {
            cx.waker().wake_by_ref();
            None
        }
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
    U: Read + Write + ReadReady,
{
    buf: &'a mut [u8],
    qca: &'a mut Qca7000<U>,
}

impl<'a, U> embassy_net_driver::TxToken for TxToken<'a, U>
where
    U: Read + Write + ReadReady,
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
