use integrations::windows::ffi;
use types::TcpState;

impl From<ffi::DWORD> for TcpState {
    fn from(tcp_state: ffi::DWORD) -> TcpState {
        match tcp_state {
            ffi::MIB_TCP_STATE_CLOSED => TcpState::MIB_TCP_STATE_CLOSED,
            ffi::MIB_TCP_STATE_LISTEN => TcpState::MIB_TCP_STATE_LISTEN,
            ffi::MIB_TCP_STATE_SYN_SENT => TcpState::MIB_TCP_STATE_SYN_SENT,
            ffi::MIB_TCP_STATE_SYN_RCVD => TcpState::MIB_TCP_STATE_SYN_RCVD,
            ffi::MIB_TCP_STATE_ESTAB => TcpState::MIB_TCP_STATE_ESTAB,
            ffi::MIB_TCP_STATE_FIN_WAIT1 => TcpState::MIB_TCP_STATE_FIN_WAIT1,
            ffi::MIB_TCP_STATE_FIN_WAIT2 => TcpState::MIB_TCP_STATE_FIN_WAIT2,
            ffi::MIB_TCP_STATE_CLOSE_WAIT => TcpState::MIB_TCP_STATE_CLOSE_WAIT,
            ffi::MIB_TCP_STATE_CLOSING => TcpState::MIB_TCP_STATE_CLOSING,
            ffi::MIB_TCP_STATE_LAST_ACK => TcpState::MIB_TCP_STATE_LAST_ACK,
            ffi::MIB_TCP_STATE_TIME_WAIT => TcpState::MIB_TCP_STATE_TIME_WAIT,
            ffi::MIB_TCP_STATE_DELETE_TCB => TcpState::MIB_TCP_STATE_DELETE_TCB,
            _ => panic!("Unknown TcpState!"),
        }
    }
}
