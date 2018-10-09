#include <arpa/inet.h>
#include <asm/types.h>
#include <linux/inet_diag.h>
#include <linux/netlink.h>
#include <linux/rtnetlink.h>
#include <linux/sock_diag.h>
#include <linux/tcp.h>
#include <netinet/in.h>
#include <pwd.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

#define _NLMSG_ALIGNTO 4U
#define _NLMSG_ALIGN(len) (/*printf("_NLMSG_ALIGN(%i) = %i\n", (len), (((len)+_NLMSG_ALIGNTO-1) & ~(_NLMSG_ALIGNTO-1))),*/ ((len) + _NLMSG_ALIGNTO - 1) & ~(_NLMSG_ALIGNTO - 1))
#define _NLMSG_HDRLEN ((int)_NLMSG_ALIGN(sizeof(struct nlmsghdr)))
#define _NLMSG_LENGTH(len) ((len) + _NLMSG_HDRLEN)
#define _NLMSG_SPACE(len) _NLMSG_ALIGN(_NLMSG_LENGTH(len))
#define _NLMSG_DATA(nlh) ((void *)(((char *)nlh) + _NLMSG_LENGTH(0)))
#define _NLMSG_NEXT(nlh, len) ((len) -= _NLMSG_ALIGN((nlh)->nlmsg_len), \
                               (struct nlmsghdr *)(((char *)(nlh)) + _NLMSG_ALIGN((nlh)->nlmsg_len)))
#define _NLMSG_OK(nlh, len) ((len) >= (int)sizeof(struct nlmsghdr) &&       \
                             (nlh)->nlmsg_len >= sizeof(struct nlmsghdr) && \
                             (nlh)->nlmsg_len <= (len))

// Kernel TCP states. /include/net/tcp_states.h
// enum {
//     TCP_ESTABLISHED = 1,
//     TCP_SYN_SENT,
//     TCP_SYN_RECV,
//     TCP_FIN_WAIT1,
//     TCP_FIN_WAIT2,
//     TCP_TIME_WAIT,
//     TCP_CLOSE,
//     TCP_CLOSE_WAIT,
//     TCP_LAST_ACK,
//     TCP_LISTEN,
//     TCP_CLOSING
// };

#define TCPF_ALL 0xFFF
#define SOCKET_BUFFER_SIZE 8192L
// (getpagesize() < 8192L ? getpagesize() : 8192L)
// extern int getpagesize (void)  __THROW __attribute__ ((__const__));

int send_diag_msg(int sockfd, __u8 family, __u8 protocol)
{
    struct msghdr msg;
    struct nlmsghdr nlh;
    struct inet_diag_req_v2 conn_req;
    struct sockaddr_nl sa;
    struct iovec iov[4];
    int retval = 0;
    memset(&msg, 0, sizeof(msg));
    memset(&sa, 0, sizeof(sa));
    memset(&nlh, 0, sizeof(nlh));
    memset(&conn_req, 0, sizeof(conn_req));
    sa.nl_family = AF_NETLINK;
    conn_req.sdiag_family = family;     //AF_INET | AF_INET6;
    conn_req.sdiag_protocol = protocol; // IPPROTO_UDP;
    conn_req.idiag_states = TCPF_ALL;
    conn_req.idiag_ext |= (1 << (INET_DIAG_INFO - 1));
    nlh.nlmsg_len = _NLMSG_LENGTH(sizeof(conn_req));
    // printf("nlh.len = %i\n", nlh.nlmsg_len);
    nlh.nlmsg_flags = NLM_F_DUMP | NLM_F_REQUEST;
    nlh.nlmsg_type = SOCK_DIAG_BY_FAMILY;
    iov[0].iov_base = (void *)&nlh;
    iov[0].iov_len = sizeof(nlh);
    iov[1].iov_base = (void *)&conn_req;
    iov[1].iov_len = sizeof(conn_req);
    // printf("iov[0].len = %i\n", sizeof(nlh));
    // printf("iov[1].len = %i\n", sizeof(conn_req));
    msg.msg_name = (void *)&sa;
    msg.msg_namelen = sizeof(sa);
    msg.msg_iov = iov;
    msg.msg_iovlen = 2;
    retval = sendmsg(sockfd, &msg, 0);
    // printf("send_diag_msg retval = %i\n", retval);
    return retval;
}

void parse_diag_msg(struct inet_diag_msg *diag_msg, int rtalen)
{
    struct rtattr *attr;
    struct tcp_info *tcpi;
    char local_addr_buf[INET6_ADDRSTRLEN];
    char remote_addr_buf[INET6_ADDRSTRLEN];
    struct passwd *uid_info = NULL;
    memset(local_addr_buf, 0, sizeof(local_addr_buf));
    memset(remote_addr_buf, 0, sizeof(remote_addr_buf));
    if (diag_msg->idiag_family == AF_INET)
    {
        inet_ntop(AF_INET, (struct in_addr *)&(diag_msg->id.idiag_src),
                  local_addr_buf, INET_ADDRSTRLEN);
        inet_ntop(AF_INET, (struct in_addr *)&(diag_msg->id.idiag_dst),
                  remote_addr_buf, INET_ADDRSTRLEN);
    }
    else if (diag_msg->idiag_family == AF_INET6)
    {
        inet_ntop(AF_INET6, (struct in_addr6 *)&(diag_msg->id.idiag_src),
                  local_addr_buf, INET6_ADDRSTRLEN);
        inet_ntop(AF_INET6, (struct in_addr6 *)&(diag_msg->id.idiag_dst),
                  remote_addr_buf, INET6_ADDRSTRLEN);
    }
    // fprintf(stdout, "src_ip: %s ", local_addr_buf);
    fprintf(stdout, "%s:%u -> %s:%u\n",
            local_addr_buf,
            ntohs(diag_msg->id.idiag_sport),
            remote_addr_buf,
            ntohs(diag_msg->id.idiag_dport));
    // fprintf(stdout, "sport: %u ", ntohs(diag_msg->id.idiag_sport));
    // fprintf(stdout, "dport: %u ", ntohs(diag_msg->id.idiag_dport));
    // fprintf(stdout, "inode: %u ", diag_msg->idiag_inode);
    // if (rtalen > 0) {
    //     attr = (struct rtattr *)(diag_msg + 1);
    //     while (RTA_OK(attr, rtalen)) {
    //         if (attr->rta_type == INET_DIAG_INFO) {
    //             tcpi = (struct tcp_info *)RTA_DATA(attr);
    //             fprintf(stdout, "state: %u", tcpi->tcpi_state);
    //         }
    //         attr = RTA_NEXT(attr, rtalen);
    //     }
    // }
    // fprintf(stdout, "\n");
}

void get_socket_info_fp(__u8 family, __u8 protocol)
{
    int nl_sock = 0, numbytes = 0, rtalen = 0;
    struct nlmsghdr *nlh;
    uint8_t recv_buf[SOCKET_BUFFER_SIZE];
    struct inet_diag_msg *diag_msg;
    nl_sock = socket(AF_NETLINK, SOCK_DGRAM, NETLINK_INET_DIAG);
    send_diag_msg(nl_sock, family, protocol);
    // printf("sizeof(recv_buf) = %li\n", sizeof(recv_buf));
    while (1)
    {
        numbytes = recv(nl_sock, recv_buf, SOCKET_BUFFER_SIZE, /*sizeof(recv_buf),*/ 0);
        nlh = (struct nlmsghdr *)recv_buf;
        // printf("numbytes = %i\n", numbytes);
        while (_NLMSG_OK(nlh, numbytes))
        {
            if (nlh->nlmsg_type == NLMSG_DONE)
            {
                // printf("wtf?!!\n");
                close(nl_sock);
                return;
            }
            if (nlh->nlmsg_type == NLMSG_ERROR)
            {
                close(nl_sock);
                fprintf(stderr, "Error in netlink message\n");
                return;
            }
            diag_msg = (struct inet_diag_msg *)_NLMSG_DATA(nlh);
            rtalen = nlh->nlmsg_len - _NLMSG_LENGTH(sizeof(*diag_msg));
            parse_diag_msg(diag_msg, rtalen);
            nlh = _NLMSG_NEXT(nlh, numbytes);
        }
    }
    return;
}

void get_socket_info()
{
    printf("AF_INET, IPPROTO_TCP:\n");
    get_socket_info_fp(AF_INET, IPPROTO_TCP);
    printf("\nAF_INET6, IPPROTO_TCP:\n");
    get_socket_info_fp(AF_INET6, IPPROTO_TCP);
    printf("\nAF_INET, IPPROTO_UDP:\n");
    get_socket_info_fp(AF_INET, IPPROTO_UDP);
    printf("\nAF_INET6, IPPROTO_UDP:\n");
    get_socket_info_fp(AF_INET6, IPPROTO_UDP);
}

int main(int argc, char *argv[])
{
    get_socket_info();
}
