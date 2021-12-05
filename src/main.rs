use std::{net::TcpListener, os::unix::prelude::AsRawFd, collections::HashMap};
use nix::{sys::epoll::{epoll_create1, EpollFlags, EpollOp, EpollCreateFlags, EpollEvent, epoll_ctl}, libc::epoll_wait };

fn main(){
    let epoll_in = EpollFlags::EPOLLIN;
    let epoll_add = EpollOp::EpollCtlAdd;
    let epoll_del = EpollOp::EpollCtlDel;

    let listenner = TcpListener::bind("127.0.0.1:10000").unwrap();

    let epfd = epoll_create1(EpollCreateFlags::empty()).unwrap();

    let listen_fd = listenner.as_raw_fd();
    let mut ev = EpollEvent::new(epoll_in, listen_fd as u64);
    epoll_ctl(epfd, epoll_add, listen_fd, &mut ev);

    let mut fd2buf = HashMap::new();
    let mut events = vec![EpollEvent::empty();1024];

    while let Ok(nfds) = epoll_wait(epfd, &mut eventsw, -1){
        for n in 0..nfds {
            if events[n].data() == listen_fd as u64 {
                if let Ok((stream, _)) = listenner.accept() {
                    let fd = stream.as_raw_fd();
                }
            }
        }
    }
}