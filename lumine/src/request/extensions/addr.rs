use std::net::IpAddr;

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Addr(pub IpAddr);

impl Addr {
    pub fn new(ip: IpAddr) -> Self {
        Self(ip)
    }

    pub fn ip(self) -> IpAddr {
        self.0
    }

    pub fn is_ipv4(self) -> bool {
        self.0.is_ipv4()
    }

    pub fn is_ipv6(self) -> bool {
        self.0.is_ipv6()
    }

    pub fn as_ip(&self) -> &IpAddr {
        &self.0
    }
}
