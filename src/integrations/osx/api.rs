use integrations::osx::netstat::*;
use types::*;

pub fn get_sockets_info(
    af_flags: AddressFamilyFlags,
    proto_flags: ProtocolFlags,
) -> Result<Vec<SocketInfo>, Error> {
    get_netstat_info(af_flags, proto_flags)
}
