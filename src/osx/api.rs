use osx::netstat::*;
use types::*;

pub fn get_sockets_info(
    address_family: AddressFamily,
    protocol: Protocol,
) -> Result<Vec<SocketInfo>, Error> {
    get_netstat_info(address_family, protocol)
}
